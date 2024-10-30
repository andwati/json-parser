mod lexer;
mod parser;

use std::process::exit;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: json_parser <file_path>");
        exit(1);
    }

    let file_path = &args[1];
    match fs::read_to_string(file_path) {
        Ok(content) => {
            match parser::parse_json_object(&content) {
                Ok(_) => {
                    println!("Valid JSON");
                    exit(0);
                }
                Err(err) => {
                    eprintln!("Invalid JSON: {}", err);
                    exit(1);
                }
            }
        }
        Err(_) => {
            eprintln!("Failed to read file");
            exit(1);
        }
    }
}