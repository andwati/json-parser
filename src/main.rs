use std::process::exit;
use std::{env, fs};

#[derive(Debug)]
enum Token {
    CurlyOpen,
    CurlyClose,
    Colon,
    Comma,
    String(String),
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: json_parser <file_path>");
        exit(1);
    }

    let file_path = &args[1];
    // match fs::read_to_string(file_path) {
    //     Ok(content) => {
    //         if is_valid_json(&content) {
    //             println!("Valid JSON");
    //             exit(0);
    //         } else {
    //             eprintln!("Invalid JSON");
    //             exit(1);
    //         }
    //     }
    //
    //     Err(_) => {
    //         eprintln!("Unable to read file");
    //         exit(1);
    //     }
    // }
    match fs::read_to_string(file_path) {
        Ok(content) => match parse_json_object(&content) {
            Ok(_) => {
                println!("Valid JSON");
                exit(0);
            }
            Err(err) => {
                eprintln!("Invalid JSON: {}", err);
                exit(1);
            }
        },
        Err(_) => {
            eprintln!("Failed to read file");
            exit(1);
        }
    }
}

fn is_valid_json(content: &str) -> bool {
    content.trim() == "{}"
}

fn tokenize(content: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = content.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        match chars[i] {
            '{' => tokens.push(Token::CurlyOpen),
            '}' => tokens.push(Token::CurlyClose),
            ':' => tokens.push(Token::Colon),
            ',' => tokens.push(Token::Comma),
            '"' => {
                let mut j = i + 1;
                while j < chars.len() && chars[j] != '"' {
                    j += 1;
                }
                if j >= chars.len() {
                    return Err("Unclose string".into());
                }
                let string_content: String = chars[i + 1..j].iter().collect();
                tokens.push(Token::String(string_content));
                i = j;
            }
            c if c.is_whitespace() => {}
            _ => return Err(format!("Unexpected character '{}'", chars[i])),
        }
        i += 1;
    }
    Ok(tokens)
}
fn parse_json_object(content: &str) -> Result<(), String> {
    let tokens = tokenize(content)?;

    let mut iter = tokens.iter().peekable();
    if let Some(Token::CurlyOpen) = iter.next() {
        loop {
            match iter.next() {
                Some(Token::String(key)) => {
                    match iter.next() {
                        Some(Token::Colon) => {
                            if let Some(Token::String(value)) = iter.next() {
                                println!("Parsed key-value pair: {}: {}", key, value);
                            } else {
                                return Err("Expected string value after ':'".into());
                            }
                        }
                        _ => return Err("Expected ':' after key".into()),
                    }
                }
                Some(Token::CurlyClose) => break,
                _ => return Err("Expected string key or '}'".into()),
            }


            match iter.next() {
                Some(Token::Comma) => {
                    if let Some(Token::CurlyClose) = iter.peek() {
                        return Err("Trailing comma before '}' is not allowed".into());
                    }
                }
                Some(Token::CurlyClose) => break,
                _ => return Err("Expected ',' or '}' after value".into()),
            }
        }
        Ok(())
    } else {
        Err("Expected '{' at beginning".into())
    }
}