use crate::config;
use std::{fs, io};

pub fn run(conf: config::Config) -> Result<String, io::Error> {
    fs::read_dir(conf.config.spaces_dir.clone())?;
    fs::remove_dir_all(conf.config.spaces_dir.clone())?;

    Ok(format!("Purged {}", conf.config.spaces_dir))
}
