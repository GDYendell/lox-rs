use std::fmt::Display;

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

    EoF,
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

            TokenKind::EoF => "EOF",
        };

        write!(f, "{}", value)
    }
}
