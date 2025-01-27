use crate::config;
use std::{fs, io};

pub fn run(conf: config::Config) -> Result<(), io::Error> {
    fs::read_dir(conf.config.spaces_dir.clone())?;
    fs::remove_dir_all(conf.config.spaces_dir.clone())
}
