use std::collections::HashMap;
use std::{env, fs};

pub type Config = HashMap<Username, ReposConfig>;
pub type Username = String;
pub type Repo = String;
pub type ReposConfig = HashMap<Token, Repos>;
pub type Repos = Vec<Repo>;
pub type Token = String;

pub fn load() -> Config {
    let spaces_file = env::var("SPACES_CONFIG").unwrap_or(default_spaces_file());

    println!("Loading config from {:?}", spaces_file);

    let data = fs::read_to_string(spaces_file).expect("Unable to read file");
    serde_yaml::from_str(&data).unwrap()
}

const DEFAULT_SPACES_FILE_NAME: &str = ".spaces.yml";

fn default_spaces_file() -> String {
    let home = dirs::home_dir().unwrap();
    let spaces_file = home.join(DEFAULT_SPACES_FILE_NAME);
    spaces_file.to_str().unwrap().to_string()
}
