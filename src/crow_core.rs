// This module is crow engine that managment every think that is about sandbox core.
use anyhow::Result;
use sha2::{Digest, Sha256};
use std::io::Read;
use std::str;

static ROOT_WORK_DIR: &str = "result_files/";

pub fn init() {
   println!(
        "User's Name            whoami::realname():    {}",
        whoami::realname(),
    );

    match std::fs::create_dir(ROOT_WORK_DIR) {
        Ok(()) => println!("result_files root directory created."),
        Err(_e) => println!("crow_core_alert:{}", _e),
    };
}

pub fn init_for_any_file(file_hash_name: &str) {
    match std::fs::create_dir_all(ROOT_WORK_DIR.to_string() + file_hash_name) {
        Ok(()) => println!("result_file_dir directory created."),
        Err(_e) => println!("crow_core_alert:{}", _e),
    };
}

pub fn get_file(_threat_file: &str) {}

pub fn calculate_hash_file(threat_file_name: &str) -> Result<String> {
    let mut file = std::fs::File::open(threat_file_name)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0; 4096];
    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

// we must check that the Vms is exist.
