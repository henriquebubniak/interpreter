use std::env;
use std::fmt::Display;
use std::fs;

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

#[derive(Clone)]
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

struct Scanner {
    buffer: Vec<Token>,
}

impl Scanner {
    pub fn scan(&mut self, source: &str) -> Vec<Token> {
        for line in source.lines() {
            self.parse_line(line)
        }
        self.buffer.push(Token::new(TokenType::Eof, "".to_string()));
        self.buffer.clone()
    }
    fn parse_line(&mut self, line: &str) {
        for c in line.chars() {
            match c {
                '(' => self
                    .buffer
                    .push(Token::new(TokenType::LeftParen, c.to_string())),
                ')' => self
                    .buffer
                    .push(Token::new(TokenType::RightParen, c.to_string())),
                '{' => self
                    .buffer
                    .push(Token::new(TokenType::LeftBrace, c.to_string())),
                '}' => self
                    .buffer
                    .push(Token::new(TokenType::RightBrace, c.to_string())),
                ',' => self
                    .buffer
                    .push(Token::new(TokenType::Comma, c.to_string())),
                '.' => self.buffer.push(Token::new(TokenType::Dot, c.to_string())),
                '-' => self
                    .buffer
                    .push(Token::new(TokenType::Minus, c.to_string())),
                '+' => self.buffer.push(Token::new(TokenType::Plus, c.to_string())),
                ';' => self
                    .buffer
                    .push(Token::new(TokenType::Semicolon, c.to_string())),
                '/' => self
                    .buffer
                    .push(Token::new(TokenType::Slash, c.to_string())),
                '*' => self.buffer.push(Token::new(TokenType::Star, c.to_string())),
                _ => (),
            }
        }
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
                let mut scanner = Scanner { buffer: Vec::new() };
                let tokens = scanner.scan(&file_contents);
                for token in tokens {
                    println!("{token}");
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
