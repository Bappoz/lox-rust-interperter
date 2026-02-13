mod error;
mod scanner;
mod token;

use crate::scanner::*;

use crate::error::LoxError;
use std::fs::read_to_string;
use std::process::exit;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]),
        _ => {
            eprintln!("Usage: jlox [script]");
            exit(64);
        }
    }
}

fn run_file(path: &str) {
    let source = read_to_string(path).expect("Could not read file");

    if let Err(err) = self::run(&source) {
        eprintln!("[line {}] Error: {}", err.line, err.message);
        exit(65)
    }
}

fn run(bytes: &str) -> Result<(), LoxError> {
    let mut scanner = Scanner::new(bytes);
    let tokens = match scanner.scan_tokens() {
        Ok(value) => value,
        Err(err) => return Err(LoxError::new(0, err)),
    };

    for token in tokens {
        println!("{:?}", token);
    }
    Ok(())
}

fn run_prompt() {
    use std::io::{self, BufRead, Write};
    let input = io::stdin();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut buf = String::new();
        let mut handle = input.lock();

        if handle.read_line(&mut buf).is_err() {
            break;
        }
        println!("ECHO: {}", buf);

        if let Err(err) = self::run(&buf) {
            eprintln!("[line {}] Error: {}", err.line, err.message);
        }
    }
}
