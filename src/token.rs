#[derive(Debug, PartialEq)]
pub struct Token {
    kind: TokenKind,
    value: Option<TokenValue>,
}

impl From<TokenKind> for Token {
    fn from(kind: TokenKind) -> Token {
        Token { kind, value: None }
    }
}

impl From<(TokenKind, String)> for Token {
    fn from((kind, value): (TokenKind, String)) -> Token {
        Token { kind, value: Some(TokenValue::String(value)) }
    }
}

impl From<(TokenKind, f64)> for Token {
    fn from((kind, value): (TokenKind, f64)) -> Token {
        Token { kind, value: Some(TokenValue::Number(value)) }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Star,
    Slash,

    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Identifier,
    String,
    Number,

    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
}

#[derive(Debug, PartialEq)]
pub enum TokenValue {
    String(String),
    Number(f64),
}
