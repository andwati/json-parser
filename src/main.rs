use std::env;
use std::fs;
use std::process::exit;

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
    match fs::read_to_string(file_path) {
        Ok(content) => {
            if is_valid_json(&content) {
                println!("Valid JSON");
                exit(0);
            } else {
                eprintln!("Invalid JSON");
                exit(1);
            }
        }

        Err(_) => {
            eprintln!("Unable to read file");
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
