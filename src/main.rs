mod scanner;
use crate::scanner::*;

use std::{env, fs};
use std::process::exit;
use log::info;
use std::io::{self, BufRead, Write};


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: jlox [script]");
        exit(64);
    } else if args.len() == 2 {
        match run_file(&args[1]){
            Ok(_) => exit(0),
            Err(msg) => {
                println!("Error: {}", msg);
                exit(1);
            }
        }
    } else {
        match run_prompt() {
            Ok(_) => exit(0),
            Err(msg) => {
                println!("Error: {}", msg);
                exit(1);
            }
        }
    }
}

fn run_file(path: &str) -> Result<(), String> {
    info!("Running file inside func");
    match fs::read_to_string(path) {
        Err(msg) => Err(msg.to_string()),
        Ok(bytes) => run(&bytes)
    }
}

fn run(bytes: &str) -> Result<(), String> {
    let scanner = Scanner::new(&bytes);
    let tokens = scanner.scan_tokens()?;

    for token in tokens {
        println!("{:?}", token);
    }
    Ok(())
}

fn run_prompt() -> Result<(), String> {
    loop {
        match io::stdout().flush() {
            Ok(_) => {},
            Err(_) => return Err("Could not flush the stdout".to_string()),
        }

        let mut buf = String::new();

        let input = io::stdin();
        let mut handle = input.lock();

        match handle.read_line(&mut buf) {
            Ok(n) => {
                if n <= 1 {
                    return Ok(())
                }
            },
            Err(_) => return Err("Could not read line".to_string())
        }
        println!("> You wrote: {}", buf);

        match run(&buf) {
            Ok(_) => {}
            Err(msg) => println!("Error: {}", msg)
        }
    }
}

