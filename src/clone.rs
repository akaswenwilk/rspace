use crate::config;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn clone(
    conf: config::Config,
    repo: String,
    branch: String,
    _base_branch: String,
) -> io::Result<String> {
    let mut message = String::new();
    let destination_path = space_path(conf, &repo, &branch);
    match destination_path.to_str() {
        Some(path) => {
            Command::new("git")
                .args(["clone", &repo, path])
                .status()
                .expect("Failed to execute git clone");
            message.push_str(&format!("Cloned into {}", path));
        }
        None => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Destination path not found",
            ));
        }
    }

    return Ok(message);
}

fn space_path(conf: config::Config, repo: &String, branch: &String) -> PathBuf {
    let spaces_dir = Path::new(&conf.config.spaces_dir);
    let mut split_repo = repo.split('/').collect::<Vec<&str>>();
    let repo_name = &split_repo.pop().unwrap().replace(".git", "");
    let owner_name = &split_repo.pop().unwrap();

    let mut branch_name = String::new();

    if branch.is_empty() {
        let matched_repo = conf
            .repos
            .iter()
            .map(|r| r.repos.clone())
            .flatten()
            .find(|r| &r.name == repo);

        if let Some(r) = matched_repo {
            branch_name.push_str(&r.default_branch);
        }
    } else {
        branch_name.push_str(branch);
    }

    let sub_dir = format!("{}-{}-{}", owner_name, repo_name, branch_name);

    spaces_dir.join(sub_dir).to_path_buf()
}
