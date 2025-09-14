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

pub fn scan_tokens(source: &str) -> Result<Vec<Token>, LexerError> {
    source
        .chars()
        .map(|char| match char {
            '(' => Ok(Token::LeftParen),
            ')' => Ok(Token::RightParen),
            '{' => Ok(Token::LeftBrace),
            '}' => Ok(Token::RightBrace),
            _ => Err(LexerError::UnexpectedChar(char, 1)),
        })
        .collect::<Result<Vec<Token>, LexerError>>()
}
