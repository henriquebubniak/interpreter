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
    Equal,
    EqualEqual,
    Unmatched,
    Bang,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    String,
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
    literal: Option<String>,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.ttype,
            self.lexeme,
            if let Some(s) = &self.literal {
                s
            } else {
                "null"
            }
        )
    }
}
impl Token {
    fn new(ttype: TokenType, lexeme: String, literal: Option<String>) -> Token {
        Token {
            ttype,
            lexeme,
            literal,
        }
    }
}

#[derive(Debug)]
struct Scanner<'a> {
    source: &'a [u8],
    pos: usize,
    errors: Vec<String>,
    line: u32,
}
impl<'a> Scanner<'a> {
    fn new(source: &str) -> Scanner {
        Scanner {
            source: source.as_bytes(),
            pos: 0,
            errors: Vec::new(),
            line: 1,
        }
    }
}
impl<'a> Iterator for Scanner<'a> {
    type Item = Option<Token>;
    fn next(&mut self) -> Option<Option<Token>> {
        if self.pos > self.source.len() {
            return None;
        }
        if self.pos == self.source.len() {
            self.pos += 1;
            return Some(Some(Token::new(TokenType::Eof, "".to_string(), None)));
        }
        let start = self.pos;
        let mut literal = None;
        let (ttype, advance, new_lines) = match self.source[start] {
            b'"' => {
                let mut i = start + 1;
                let mut new_lines = 0;
                while i < self.source.len() && self.source[i] != b'"' {
                    if self.source[i] == b'\n' {
                        new_lines += 1;
                    }
                    i += 1;
                }
                let advance = i - start;
                if i == self.source.len() {
                    self.errors
                        .push(format!("[line {}] Error: Unterminated string.", self.line));
                    (TokenType::Unmatched, advance, new_lines)
                } else {
                    literal =
                        Some(String::from_utf8_lossy(&self.source[start + 1..i]).into_owned());
                    (TokenType::String, advance + 1, new_lines)
                }
            }
            b'(' => (TokenType::LeftParen, 1, 0),
            b')' => (TokenType::RightParen, 1, 0),
            b'{' => (TokenType::LeftBrace, 1, 0),
            b'}' => (TokenType::RightBrace, 1, 0),
            b',' => (TokenType::Comma, 1, 0),
            b'.' => (TokenType::Dot, 1, 0),
            b'-' => (TokenType::Minus, 1, 0),
            b'+' => (TokenType::Plus, 1, 0),
            b';' => (TokenType::Semicolon, 1, 0),
            b'/' if start + 1 < self.source.len() && self.source[start + 1] == b'/' => {
                let mut i = start + 1;
                while i < self.source.len() && self.source[i] != b'\n' {
                    i += 1;
                }
                let advance = i - start;
                (TokenType::Unmatched, advance, 0)
            }
            b'/' => (TokenType::Slash, 1, 0),
            b'*' => (TokenType::Star, 1, 0),
            b'!' if start + 1 < self.source.len() && self.source[start + 1] == b'=' => {
                (TokenType::BangEqual, 2, 0)
            }
            b'!' => (TokenType::Bang, 1, 0),
            b'=' if start + 1 < self.source.len() && self.source[start + 1] == b'=' => {
                (TokenType::EqualEqual, 2, 0)
            }
            b'=' => (TokenType::Equal, 1, 0),
            b'<' if start + 1 < self.source.len() && self.source[start + 1] == b'=' => {
                (TokenType::LessEqual, 2, 0)
            }
            b'<' => (TokenType::Less, 1, 0),
            b'>' if start + 1 < self.source.len() && self.source[start + 1] == b'=' => {
                (TokenType::GreaterEqual, 2, 0)
            }
            b'>' => (TokenType::Greater, 1, 0),
            c if c.is_ascii_whitespace() => {
                let mut i = start;
                let mut new_lines = 0;
                while i < self.source.len() && self.source[i].is_ascii_whitespace() {
                    if self.source[i] == b'\n' {
                        new_lines += 1;
                    }
                    i += 1;
                }
                let advance = i - start;
                (TokenType::Unmatched, advance, new_lines)
            }
            _ => {
                self.errors.push(format!(
                    "[line {}] Error: Unexpected character: {}",
                    self.line, self.source[start] as char
                ));
                (TokenType::Unmatched, 1, 0)
            }
        };
        self.pos += advance;
        self.line += new_lines;
        if let TokenType::Unmatched = ttype {
            Some(None)
        } else {
            Some(Some(Token::new(
                ttype,
                String::from_utf8_lossy(&self.source[start..start + advance]).into_owned(),
                literal,
            )))
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
                let mut scanner = Scanner::new(&file_contents);
                for token in scanner.by_ref() {
                    if token.is_none() {
                        continue;
                    }
                    let token = token.unwrap();
                    if let TokenType::Unmatched = token.ttype {
                    } else {
                        println!("{}", token);
                    }
                }
                for error in scanner.errors.iter().by_ref() {
                    eprintln!("{error}");
                }
                if scanner.errors.len() > 0 {
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
