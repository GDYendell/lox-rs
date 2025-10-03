use std::fmt::Display;

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
        Token { kind, value: Some(TokenValue::String(value)) }
    }
}

impl From<(TokenKind, f64)> for Token {
    fn from((kind, value): (TokenKind, f64)) -> Token {
        Token { kind, value: Some(TokenValue::Number(value)) }
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

#[derive(Debug, PartialEq, Clone)]
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


impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            TokenKind::LeftParen => "(",
            TokenKind::RightParen => ")",
            TokenKind::LeftBrace => "{",
            TokenKind::RightBrace => "}",
            TokenKind::Comma => ",",
            TokenKind::Dot => ".",
            TokenKind::Minus => "-",
            TokenKind::Plus => "+",
            TokenKind::Semicolon => ";",
            TokenKind::Star => "*",
            TokenKind::Slash => "/",

            TokenKind::Bang => "!",
            TokenKind::BangEqual => "!=",
            TokenKind::Equal => "=",
            TokenKind::EqualEqual => "==",
            TokenKind::Greater => ">",
            TokenKind::GreaterEqual => ">=",
            TokenKind::Less => "<",
            TokenKind::LessEqual => "<=",

            TokenKind::Identifier => "identifier",
            TokenKind::String => "string",
            TokenKind::Number => "number",

            TokenKind::And => "and",
            TokenKind::Class => "class",
            TokenKind::Else => "else",
            TokenKind::False => "false",
            TokenKind::Fun => "fun",
            TokenKind::For => "for",
            TokenKind::If => "if",
            TokenKind::Nil => "nil",
            TokenKind::Or => "or",
            TokenKind::Print => "print",
            TokenKind::Return => "return",
            TokenKind::Super => "super",
            TokenKind::This => "this",
            TokenKind::True => "true",
            TokenKind::Var => "var",
            TokenKind::While => "while",

            TokenKind::EOF => "EOF",
        };

        write!(f, "{}", value)
    }
}


#[derive(Debug, PartialEq, Clone)]
pub enum TokenValue {
    String(String),
    Number(f64),
}

impl Display for TokenValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenValue::String(value) => write!(f, "\"{}\"", value),
            TokenValue::Number(value) => write!(f, "{}", value),
        }
    }
}