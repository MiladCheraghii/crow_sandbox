// This module is crow engine that managment every think that is about sandbox core.
use std::str;
use std::{io::Read};
use anyhow::Result;
use sha2::{Sha256, Digest};

static root_work_dir : &str ="result_files/";

pub fn init() {
	match std::fs::create_dir(root_work_dir) {
	Ok(()) => println!("result_files root directory created."),
	Err(_e) => println!("result_files root directory dodon't created."),};
}


pub fn init_for_any_file(file_hash_name : &str) { 
	match std::fs::create_dir_all(root_work_dir.to_string() + file_hash_name) {
	Ok(()) => println!("result_file_dir directory created."),
	Err(_e) => println!("result_file_dir directory don't created."),};
}

pub fn get_file(_threat_file : &str) {}

	// Maybe we must create a specific folder for any file that we want analysis it.

pub fn calculate_hash_file(threat_file_name : &str) -> Result<String> {
	
	// Open the file
    let mut file = std::fs::File::open(threat_file_name)?;

    // Create a SHA-256 "hasher"
    let mut hasher = Sha256::new();

    // Read the file in 4KB chunks and feed them to the hasher
    let mut buffer = [0; 4096];
    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }
     // Finalize the hash and get the result as a byte array
    Ok(format!("{:x}", hasher.finalize()))
}


// we must check that the Vms is exist.