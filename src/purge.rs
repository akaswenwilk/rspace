use crate::{config, error};
use std::fs;

pub fn run(conf: config::Config) -> Result<String, error::CustomError> {
    fs::read_dir(conf.config.spaces_dir.clone())?;
    fs::remove_dir_all(conf.config.spaces_dir.clone())?;

    Ok(format!("Purged {}", conf.config.spaces_dir))
}
