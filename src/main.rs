mod scanner;
mod exception;
mod token;
mod literal_value;
mod token_type;
mod expr;

use crate::scanner::*;

use std::{env, fs, io};
use std::io::{stdout, Write};
use std::process::exit;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    return match args.len() {
        1 => run_prompt(),
        2 => {
            match run_file(&args[1]) {
                Ok(_) => exit(0),
                Err(msg) => {
                    println!("Error: \n{}", msg);
                    exit(1);
                }
            }
        }
        _ => {
            println!("Usage: `platypus [script]` or `platypus`");
            exit(64);
        }
    };
}

fn run_prompt() -> Result<(), String> {
    loop {
        println!("platypus> ");
        stdout().flush().expect("TODO: panic message");
        let mut input = String::new();
        let _byte_size = io::stdin().read_line(&mut input).unwrap();

        if input.is_empty() || input == '\n'.to_string() {
            return Ok(());
        }

        match run(&input) {
            Ok(_) => (),
            Err(msg) => println!("{}", msg),
        }
    }
}

fn run_file(path: &str) -> Result<(), String> {
    return match fs::read_to_string(path) {
        Err(msg) => Err(msg.to_string()),
        Ok(input) => run(&input),
    }
}

fn run(input: &str) -> Result<(), String> {
    let mut scanner = Scanner::new(input);
    let tokens = scanner.scan_tokens()?;

    for token in tokens {
        println!("{:?}", token);
    }

    return Ok(());
}
