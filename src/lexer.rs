#[derive(Debug)]
pub enum Token {
    CurlyOpen,
    CurlyClose,
    Colon,
    Comma,
    String(String),
    Number(f64),
    True,
    False,
    Null,
}

pub fn tokenize(content: &str) -> Result<Vec<Token>, String> {
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
                    return Err("Unclosed string".into());
                }
                let string_content: String = chars[i + 1..j].iter().collect();
                tokens.push(Token::String(string_content));
                i = j;
            }
            c if c.is_digit(10) => {
                let mut j = i;
                while j < chars.len() && (chars[j].is_digit(10) || chars[j] == '.') {
                    j += 1;
                }
                let number: f64 = chars[i..j].iter().collect::<String>().parse()
                    .map_err(|_| "Invalid number format".to_string())?;
                tokens.push(Token::Number(number));
                i = j - 1;
            }
            't' if chars[i..].starts_with(&['t', 'r', 'u', 'e']) => {
                tokens.push(Token::True);
                i += 3;
            }
            'f' if chars[i..].starts_with(&['f', 'a', 'l', 's', 'e']) => {
                tokens.push(Token::False);
                i += 4;
            }
            'n' if chars[i..].starts_with(&['n', 'u', 'l', 'l']) => {
                tokens.push(Token::Null);
                i += 3;
            }
            c if c.is_whitespace() => {} // Ignore whitespace
            _ => return Err(format!("Unexpected character '{}'", chars[i])),
        }
        i += 1;
    }

    Ok(tokens)
}
