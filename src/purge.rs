use crate::config;
use std::fs;

pub fn run(conf: config::Config) {
    if fs::read_dir(conf.config.spaces_dir.clone()).is_err() {
        println!("no spaces found in {:?}", conf.config.spaces_dir);
        return;
    }
    fs::remove_dir_all(conf.config.spaces_dir.clone()).unwrap();
    println!("all spaces in {:?} purged successfully", conf.config.spaces_dir);
}
