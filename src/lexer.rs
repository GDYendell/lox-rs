use crate::tokens::Token;

use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum LexerError {
    UnexpectedChar(char, usize),
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexerError::UnexpectedChar(char, line) => {
                write!(f, "Line {}: Unexpected character: '{}'", line, char)
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

pub fn scan_tokens(source: &str) -> Vec<Result<Token, LexerError>> {
    let mut source_iter = source.chars().peekable();

    let mut tokens = Vec::<Result<Token, LexerError>>::new();
    while let Some(char) = source_iter.next() {
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

            '!' => match source_iter.peek() {
                Some('=') => {
                    source_iter.next();
                    tokens.push(Ok(Token::BangEqual))
                }
                _ => tokens.push(Ok(Token::Bang)),
            },

            '=' => match source_iter.peek() {
                Some('=') => {
                    source_iter.next();
                    tokens.push(Ok(Token::EqualEqual))
                }
                _ => tokens.push(Ok(Token::Equal)),
            },

            '<' => match source_iter.peek() {
                Some('=') => {
                    source_iter.next();
                    tokens.push(Ok(Token::LessEqual))
                }
                _ => tokens.push(Ok(Token::Less)),
            },

            '>' => match source_iter.peek() {
                Some('=') => {
                    source_iter.next();
                    tokens.push(Ok(Token::GreaterEqual))
                }
                _ => tokens.push(Ok(Token::Greater)),
            },

            '/' => match source_iter.peek() {
                Some('/') => {
                    while let Some(char) = source_iter.next() {
                        if char == '\n' {
                            break;
                        }
                    }
                }
                _ => tokens.push(Ok(Token::Slash)),
            },

            _ => tokens.push(Err(LexerError::UnexpectedChar(char, 1))),
        }
    }

    tokens
}
