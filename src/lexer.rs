use crate::tokens::Token;

use std::fmt;

pub struct Lexer {
    source: Vec<char>,

    position: usize,
    line_count: usize,
}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        Lexer {
            source: source.chars().collect(),
            position: 0,
            line_count: 1,
        }
    }

    fn peek(&self) -> Option<char> {
        self.source.get(self.position).copied()
    }

    fn next(&mut self) -> Option<char> {
        self.peek().map(|char| {
            self.position += 1;
            char
        })
    }

    pub fn scan_tokens(&mut self) -> Vec<Result<Token, LexerError>> {
        let mut tokens = Vec::<Result<Token, LexerError>>::new();

        while let Some(char) = self.next() {
            match char {
                '(' => tokens.push(Ok(Token::LeftParen)),
                ')' => tokens.push(Ok(Token::RightParen)),
                '{' => tokens.push(Ok(Token::LeftBrace)),
                '}' => tokens.push(Ok(Token::RightBrace)),
                ',' => tokens.push(Ok(Token::Comma)),
                '.' => tokens.push(Ok(Token::Dot)),
                '-' => tokens.push(Ok(Token::Minus)),
                '+' => tokens.push(Ok(Token::Plus)),
                ';' => tokens.push(Ok(Token::Semicolon)),
                '*' => tokens.push(Ok(Token::Star)),

                '!' => match self.peek() {
                    Some('=') => {
                        self.position += 1;
                        tokens.push(Ok(Token::BangEqual))
                    }
                    _ => tokens.push(Ok(Token::Bang)),
                },

                '=' => match self.peek() {
                    Some('=') => {
                        self.position += 1;
                        tokens.push(Ok(Token::EqualEqual))
                    }
                    _ => tokens.push(Ok(Token::Equal)),
                },

                '<' => match self.peek() {
                    Some('=') => {
                        self.position += 1;
                        tokens.push(Ok(Token::LessEqual))
                    }
                    _ => tokens.push(Ok(Token::Less)),
                },

                '>' => match self.peek() {
                    Some('=') => {
                        self.position += 1;
                        tokens.push(Ok(Token::GreaterEqual))
                    }
                    _ => tokens.push(Ok(Token::Greater)),
                },

                '/' => match self.peek() {
                    Some('/') => self.scan_comment(),
                    _ => tokens.push(Ok(Token::Slash)),
                },

                ' ' | '\t' | '\r' => {}

                '\n' => {
                    self.line_count += 1;
                }

                '"' => tokens.push(self.scan_string()),

                _ => tokens.push(Err(LexerError::UnexpectedChar(char, self.line_count))),
            }
        }

        tokens
    }

    fn scan_comment(&mut self) {
        while self.position < self.source.len() && self.source[self.position] != '\n' {
            self.position += 1;
        }
    }

    fn scan_string(&mut self) -> Result<Token, LexerError> {
        let start = self.position;
        while self.position < self.source.len() && self.source[self.position] != '"' {
            if self.source[self.position] == '\n' {
                self.line_count += 1;
            }
            self.position += 1;
        }

        if self.position == self.source.len() {
            return Err(LexerError::UnterminatedString(
                self.source[start..].iter().collect(),
                self.line_count,
            ));
        }

        self.position += 1;
        Ok(Token::String(
            self.source[start..self.position - 1].iter().collect(),
        ))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum LexerError {
    UnexpectedChar(char, usize),
    UnterminatedString(String, usize),
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexerError::UnexpectedChar(char, line) => {
                write!(f, "Line {}: Unexpected character: '{}'", line, char)
            }
            LexerError::UnterminatedString(string, line) => {
                write!(f, "Line {}: Unterminated string: '{}'", line, string)
            }
        }
    }
}

impl std::error::Error for LexerError {}

impl From<LexerError> for String {
    fn from(e: LexerError) -> Self {
        e.to_string()
    }
}
