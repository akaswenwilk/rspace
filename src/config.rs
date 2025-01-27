use serde::Deserialize;
use std::{env, fs};

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub config: SpaceConfig,
    pub repos: ReposList,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SpaceConfig {
    #[serde(default = "default_spaces_dir")]
    pub spaces_dir: String,
    #[serde(default = "master")]
    pub default_branch: String,
    pub default_username: String,
    pub default_token: String,
}

pub type ReposList = Vec<Repo>;

#[derive(Debug, Deserialize, Clone)]
pub struct Repo {
    pub name: String,
    pub default_branch: Option<String>,
    pub username: Option<String>,
    pub token: Option<String>,
}

pub fn load() -> Config {
    let spaces_file = env::var("SPACES_CONFIG").unwrap_or(default_spaces_file());

    let data = fs::read_to_string(spaces_file).expect("Unable to read file");
    serde_yaml::from_str(&data).unwrap()
}

const DEFAULT_SPACES_FILE_NAME: &str = ".spaces.yml";
const DEFAULT_SPACES_DIR: &str = "spaces";

fn default_spaces_file() -> String {
    let home = dirs::home_dir().unwrap();
    let spaces_file = home.join(DEFAULT_SPACES_FILE_NAME);
    spaces_file.to_str().unwrap().to_string()
}

fn default_spaces_dir() -> String {
    let home = dirs::home_dir().unwrap();
    let spaces_dir = home.join(DEFAULT_SPACES_DIR);
    spaces_dir.to_str().unwrap().to_string()
}

fn master() -> String {
    "master".to_string()
}
