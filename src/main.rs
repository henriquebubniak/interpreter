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
    Equal,
    EqualEqual,
    Unmatched,
}
impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let self_s: String = format!("{:?}", self);
        let mut s: String = self_s.chars().take(1).collect();
        for c in self_s.chars().skip(1) {
            if c.is_uppercase() {
                s += "_";
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

struct Scanner<'a> {
    start: usize,
    current: usize,
    source: &'a [u8],
    line: u32,
    contains_error: bool,
}

impl Scanner<'_> {
    pub fn new(source: &str) -> Scanner {
        Scanner {
            start: 0,
            current: 0,
            source: source.as_bytes(),
            line: 1,
            contains_error: false,
        }
    }
    pub fn scan(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let n = self.source.len();
        self.start = 0;
        self.current = 0;
        while self.current < n {
            let token = self.next_token();
            self.start = self.current;
            tokens.push(token);
        }
        tokens.push(Token::new(TokenType::Eof, "".to_string()));
        tokens
    }
    fn next_token(&mut self) -> Token {
        self.current += 1;
        let ttype = match self.source[self.start] {
            b'(' => TokenType::LeftParen,
            b')' => TokenType::RightParen,
            b'{' => TokenType::LeftBrace,
            b'}' => TokenType::RightBrace,
            b',' => TokenType::Comma,
            b'.' => TokenType::Dot,
            b'-' => TokenType::Minus,
            b'+' => TokenType::Plus,
            b';' => TokenType::Semicolon,
            b'/' => TokenType::Slash,
            b'*' => TokenType::Star,
            b'=' if self.current < self.source.len() && self.source[self.current] == b'=' => {
                self.current += 1;
                TokenType::EqualEqual
            }
            b'=' => TokenType::Equal,
            x if x.is_ascii_whitespace() => {
                while self.current < self.source.len()
                    && self.source[self.current].is_ascii_whitespace()
                {
                    self.current += 1;
                }
                TokenType::Whitespace
            }
            _ => {
                eprintln!(
                    "[line {}] Error: Unexpected character: {}",
                    self.line, self.source[self.start] as char
                );
                self.contains_error = true;
                TokenType::Unmatched
            }
        };
        let token = Token::new(
            ttype,
            std::str::from_utf8(&self.source[self.start..self.current])
                .unwrap()
                .to_string(),
        );
        token
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
                let mut scanner = Scanner::new(&file_contents);
                let tokens = scanner.scan();
                for token in tokens {
                    if let TokenType::Unmatched = token.ttype {
                    } else if let TokenType::Whitespace = token.ttype {
                    } else {
                        println!("{token}")
                    }
                }
                if scanner.contains_error {
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
