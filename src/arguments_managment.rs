use text_colorizer::*;
use std::{env,fs};


#[derive(Debug)]
#[allow(dead_code)]

struct Arguments {
    file_name: String,
    machine_name: String,
}

fn print_help(){
    eprintln!("Welcome to {} :)", "Crow sandbox".green());
}

fn args_parser() -> Arguments {
    let args : Vec<String> = env::args().skip(1).collect(); 
    if args.len() != 2 {
        print_help();
        eprintln!("{} wrong number of argument give. got {}.", "Error".red().bold(), args.len());
        std::process::exit(1);
    }

    
    Arguments {
        file_name: args[0].clone(),
        machine_name: args[1].clone(),
    }
}

pub fn run(){
    let arg = args_parser();
    println!("{:?}", arg);

}

