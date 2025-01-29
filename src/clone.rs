use crate::{config, error};
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use std::io;
use std::path::Path;
use std::process::Command;
use url::Url;

pub fn clone(
    conf: config::Config,
    repo: String,
    branch: String,
    base_branch: String,
) -> Result<String, error::CustomError> {
    let mut message = String::new();
    let binding = config::Repo {
        name: repo.clone(),
        default_branch: None,
        username: None,
        token: None,
    };
    let matching_repo = conf
        .repos
        .iter()
        .find(|r| r.name == repo)
        .unwrap_or(&binding);

    let branch_name = get_branch_name(&conf, &branch, &matching_repo);

    let destination_path = space_path(&conf, &matching_repo, &branch_name)?;

    let repo_url = repo_url(&conf, &matching_repo)?;

    if base_branch.is_empty() {
        clone_repo_branch(&branch_name, &repo_url, &destination_path)?;
    } else {
        clone_repo_branch(&base_branch, &repo_url, &destination_path)?;
        checkout_repo(&branch_name, &destination_path)?;
    }

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(destination_path.clone()).unwrap();

    message.push_str(&format!(
        "Cloned into {}",
        destination_path
    ));

    return Ok(message);
}

fn space_path(
    conf: &config::Config,
    repo: &config::Repo,
    branch: &String,
) -> Result<String, io::Error> {
    let spaces_dir = Path::new(&conf.config.spaces_dir);
    let mut split_repo = repo.name.split('/').collect::<Vec<&str>>();
    let repo_name = &split_repo.pop().unwrap().replace(".git", "");
    let owner_name = &split_repo.pop().unwrap();

    let sub_dir = format!("{}-{}", repo_name, branch);

    let mut final_path = String::new();

    match spaces_dir.join(owner_name).join(sub_dir).to_str() {
        Some(path) => {
            final_path.push_str(path);
        }
        None => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Destination path not found",
            ));
        }
    }

    return Ok(final_path);
}

fn get_branch_name(conf: &config::Config, branch: &String, repo: &config::Repo) -> String {
    let mut branch_name = String::from(branch);

    if branch_name.is_empty() {
        match &repo.default_branch {
            Some(r) => {
                branch_name.push_str(&r);
            }
            None => {
                branch_name.push_str(&conf.config.default_branch);
            }
        }
    }

    return branch_name;
}

fn repo_url(conf: &config::Config, repo: &config::Repo) -> Result<String, error::CustomError> {
    let mut parsed_url = Url::parse(&repo.name)?;
    let username = match &repo.username {
        Some(u) => u,
        None => &conf.config.default_username,
    };
    let token = match &repo.token {
        Some(t) => t,
        None => &conf.config.default_token,
    };

    parsed_url.set_username(username)?;
    parsed_url.set_password(Some(token))?;
    Ok(String::from(parsed_url.as_str()))
}

fn clone_repo_branch(
    branch: &String,
    repo_url: &String,
    destination_path: &String,
) -> Result<(), error::CustomError> {
    let output = Command::new("git")
        .args(["clone", "--branch", branch, repo_url, destination_path])
        .output()
        .expect("Failed to execute git clone");

    if output.status.success() {
        return Ok(());
    } else {
        let err_message = String::from_utf8_lossy(&output.stderr).to_string();
        if err_message.contains("Remote branch") && err_message.contains("not found") {
            clone_repo(repo_url, destination_path)?;
            return checkout_repo(branch, destination_path);
        }
        if err_message.contains("already exists") {
            return Ok(());
        }
        return Err(error::CustomError::Io(io::Error::new(
            io::ErrorKind::Other,
            String::from_utf8_lossy(&output.stderr).to_string(),
        )));
    }
}

fn clone_repo(repo_url: &String, destination_path: &String) -> Result<(), error::CustomError> {
    let output = Command::new("git")
        .args(["clone", repo_url, destination_path])
        .output()
        .expect("Failed to execute git clone");

    if output.status.success() {
        return Ok(());
    } else {
        return Err(error::CustomError::Io(io::Error::new(
            io::ErrorKind::Other,
            String::from_utf8_lossy(&output.stderr).to_string(),
        )));
    }
}

fn checkout_repo(branch: &String, destination_path: &String) -> Result<(), error::CustomError> {
    let output = Command::new("git")
        .args(["checkout", "-b", branch])
        .current_dir(destination_path)
        .output()
        .expect("Failed to execute git checkout");

    if output.status.success() {
        return Ok(());
    } else {
        return Err(error::CustomError::Io(io::Error::new(
            io::ErrorKind::Other,
            String::from_utf8_lossy(&output.stderr).to_string(),
        )));
    }
}
