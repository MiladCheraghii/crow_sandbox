// This module is crow engine that managment every think that is about sandbox core.
use anyhow::Result;
use sha2::{Digest, Sha256};
use std::io::Read;
use std::str;
use text_colorizer::*;


//use std::fs::{self, DirBuilder};

static ROOT_WORK_DIR: &str = "result_files\\";

pub fn init() {
    match std::fs::create_dir(ROOT_WORK_DIR) {
        Ok(()) => println!("result_files root directory created."),
        Err(_e) => println!("crow_core_alert:{}", _e),
    };
}

pub fn init_for_any_file(config: String) {
    
    let hash_file = calculate_hash_file(String::from(&config));
   
    //println!("{}", hash_file.unwrap());
    //let s = append_dir(ROOT_WORK_DIR.as_ref(), &config);
    //println!("{}",s.to_string_lossy());

    //let mmmm = ROOT_WORK_DIR.to_string().push_str(&hash_file.unwrap());
   
    match std::fs::create_dir_all(&hash_file.unwrap()){
        Ok(()) => println!("Success: result_file_dir directory created."),
        Err(e) => println!("crow_core_alert:{}", e),
    }
  
    

    //let path = ROOT_WORK_DIR.to_string();
    //DirBuilder::new().recursive(true).create(path);


    /*
    match std::fs::copy(&config, re){
        Ok(k) => println!("{} ",k),
        Err(_e) => println!("{} Eroooooor -E {} {}", _e, ROOT_WORK_DIR.to_string(), &config),
    }
    */
}

/*
fn append_dir(p: &std::path::Path, d: &str) -> std::path::PathBuf {
    let dirs = p.parent().unwrap();
    println!(" dirs {}", dirs.to_string_lossy());
    println!("............");
    println!("join {}", dirs.join(p).join(d).to_string_lossy());
    dirs.join(p).join(d)
}
*/

pub fn calculate_hash_file(threat_file_name: String) -> Result<String, anyhow::Error> {
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
