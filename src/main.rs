use std::env;
use std::fmt::Display;
use std::fs;
use std::process::exit;

#[derive(Clone, Debug)]
enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Eof,
    Whitespace,
}
impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let self_s: String = format!("{:?}", self);
        let mut s: String = self_s.chars().take(1).collect();
        for c in self_s.chars().skip(1) {
            if c.is_uppercase() {
                s = s + "_";
            }
            s = s + &c.to_string().to_uppercase();
        }
        write!(f, "{}", s)
    }
}

#[derive(Clone, Debug)]
struct Token {
    ttype: TokenType,
    lexeme: String,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} null", self.ttype, self.lexeme)
    }
}
impl Token {
    fn new(ttype: TokenType, lexeme: String) -> Token {
        Token { ttype, lexeme }
    }
}

struct Scanner {}

impl Scanner {
    pub fn scan(&mut self, source: &str) -> Vec<Result<Token, String>> {
        let mut buffer = Vec::new();
        let line = 1;
        for c in source.chars() {
            let ttype = match c {
                '(' => Ok(TokenType::LeftParen),
                ')' => Ok(TokenType::RightParen),
                '{' => Ok(TokenType::LeftBrace),
                '}' => Ok(TokenType::RightBrace),
                ',' => Ok(TokenType::Comma),
                '.' => Ok(TokenType::Dot),
                '-' => Ok(TokenType::Minus),
                '+' => Ok(TokenType::Plus),
                ';' => Ok(TokenType::Semicolon),
                '/' => Ok(TokenType::Slash),
                '*' => Ok(TokenType::Star),
                ' ' | '\n' => Ok(TokenType::Whitespace),
                _ => Err(format!("[line {line}] Error: Unexpected character: {c}")),
            };
            buffer.push(match ttype {
                Ok(ok_ttype) => Ok(Token::new(ok_ttype, c.to_string())),
                Err(s) => Err(s),
            });
        }
        buffer.push(Ok(Token::new(TokenType::Eof, "".to_string())));
        buffer
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                String::new()
            });

            if !file_contents.is_empty() {
                let mut scanner = Scanner {};
                let tokens = scanner.scan(&file_contents);
                let mut contains_error = false;
                for result_token in tokens {
                    match result_token {
                        Ok(token) => match token.ttype {
                            TokenType::Whitespace => (),
                            _ => println!("{token}"),
                        },
                        Err(s) => {
                            eprintln!("{s}");
                            contains_error = true;
                        }
                    }
                }
                if contains_error {
                    exit(65);
                }
            } else {
                println!("EOF  null"); // Placeholder, remove this line when implementing the scanner
            }
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}
