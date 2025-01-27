use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;
use std::{env, fs};

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub config: SpaceConfig,
    pub repos: ReposList,
    #[serde(default = "HashMap::new")]
    pub current_spaces: HashMap<String, Vec<String>>,
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
    let mut conf: Config = serde_yaml::from_str(&data).unwrap();

    conf.gather_current_spaces();

    conf
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

impl Config {
    fn gather_current_spaces(&mut self) {
        let spaces_dir = Path::new(&self.config.spaces_dir);
        let dirs = fs::read_dir(spaces_dir);
        if let Err(_) = dirs {
            return;
        }

        spaces_dir.read_dir().unwrap().for_each(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                let owner = path.file_name().unwrap().to_str().unwrap().to_string();
                let mut spaces = Vec::new();
                path.read_dir().unwrap().for_each(|entry| {
                    let entry = entry.unwrap();
                    let path = entry.path();
                    if path.is_dir() {
                        let space = path.file_name().unwrap().to_str().unwrap().to_string();
                        spaces.push(space);
                    }
                });
                self.current_spaces.insert(owner, spaces);
            }
        });
    }
}
