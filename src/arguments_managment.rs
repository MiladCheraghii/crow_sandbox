/// # Arguments_managment
/// ! This module give a input arguments and create a struct.

use text_colorizer::*;
use std::env;

#[derive(Debug)]
#[allow(dead_code)]

pub struct Arguments {
    pub file_name: String,
    pub vm_name  : String,
}

pub fn print_help(){
    eprintln!("......");
    eprintln!("Welcome to {} :)", "Crow sandbox".green());
    eprintln!("Usage: ");
    eprintln!("cargo run {} {}", "<file name>".green(), "<vm name>".yellow());
    eprintln!("......");
}

/// We need parse arguments and create those on struct.
fn args_parser() -> Arguments {
    let args : Vec<String> = env::args().skip(1).collect(); 

    if args.len() == 1 && &args[0].clone() == "help" {
        print_help();
        std::process::exit(0);
    }
    else if !std::path::Path::new(&args[0].clone()).exists() {
        println!("{} path is wrong.", args[0].red().bold());
        std::process::exit(1);
    }

    if args.len() != 2 {
        eprintln!("{} wrong number of argument give. got {}.", "Error".red().bold(), args.len());
        std::process::exit(1);
    }

    Arguments {
        file_name: args[0].clone(),
        vm_name: args[1].clone(),
    }
}

pub fn run() -> Arguments {
    let arg = args_parser();
    arg
}

