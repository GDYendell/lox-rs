use std::fmt::Display;

use crate::token::{TokenKind, TokenValue};

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub value: Option<TokenValue>,
}

impl From<TokenKind> for Token {
    fn from(kind: TokenKind) -> Token {
        Token { kind, value: None }
    }
}

impl From<(TokenKind, String)> for Token {
    fn from((kind, value): (TokenKind, String)) -> Token {
        Token {
            kind,
            value: Some(TokenValue::String(value)),
        }
    }
}

impl From<(TokenKind, f64)> for Token {
    fn from((kind, value): (TokenKind, f64)) -> Token {
        Token {
            kind,
            value: Some(TokenValue::Number(value)),
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(value) = &self.value {
            write!(f, "{}({})", self.kind, value)
        } else {
            write!(f, "{}", self.kind)
        }
    }
}
