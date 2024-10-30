use crate::lexer::{tokenize, Token};

pub fn parse_json_object(content: &str) -> Result<(), String> {
    let tokens = tokenize(content)?;

    let mut iter = tokens.iter().peekable();
    if let Some(Token::CurlyOpen) = iter.next() {
        loop {
            // Expect a string key or a closing brace if there are no more key-value pairs
            match iter.next() {
                Some(Token::String(key)) => {
                    // Expect a colon after the key
                    match iter.next() {
                        Some(Token::Colon) => {
                            // Expect a valid value after the colon
                            match iter.next() {
                                Some(Token::String(value)) => println!("Parsed key-value pair: {}: {}", key, value),
                                Some(Token::Number(value)) => println!("Parsed key-value pair: {}: {}", key, value),
                                Some(Token::True) => println!("Parsed key-value pair: {}: true", key),
                                Some(Token::False) => println!("Parsed key-value pair: {}: false", key),
                                Some(Token::Null) => println!("Parsed key-value pair: {}: null", key),
                                _ => return Err("Expected a valid JSON value after ':'".into()),
                            }
                        }
                        _ => return Err("Expected ':' after key".into()),
                    }
                }
                Some(Token::CurlyClose) => break, // End of JSON object
                _ => return Err("Expected string key or '}'".into()),
            }

            // After each key-value pair, expect either a comma or closing brace
            match iter.next() {
                Some(Token::Comma) => {
                    if let Some(Token::CurlyClose) = iter.peek() {
                        return Err("Trailing comma before '}' is not allowed".into());
                    }
                }
                Some(Token::CurlyClose) => break, // End of JSON object
                _ => return Err("Expected ',' or '}' after value".into()),
            }
        }
        Ok(())
    } else {
        Err("Expected '{' at beginning".into())
    }
}
