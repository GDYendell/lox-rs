use crate::tokens::Token;

use super::error::LexerError;

macro_rules! scan_operator {
    ($self:ident, $char:literal, $token1:ident, $token2:ident) => {
        if let Some($char) = $self.peek() {
            $self.next();
            Ok(Token::$token1)
        } else {
            Ok(Token::$token2)
        }
    };
}

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

    fn peek_two(&self, n: usize) -> Option<Vec<char>> {
        (self.position..self.position + n)
            .into_iter()
            .map(|i| self.source.get(i).copied())
            .collect()
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

                '!' => tokens.push(scan_operator!(self, '=', BangEqual, Bang)),
                '=' => tokens.push(scan_operator!(self, '=', EqualEqual, Equal)),
                '<' => tokens.push(scan_operator!(self, '=', LessEqual, Less)),
                '>' => tokens.push(scan_operator!(self, '=', GreaterEqual, Greater)),

                '/' => match self.peek() {
                    Some('/') => self.scan_comment(),
                    _ => tokens.push(Ok(Token::Slash)),
                },

                ' ' | '\t' | '\r' => {}

                '\n' => {
                    self.line_count += 1;
                }

                '"' => tokens.push(self.scan_string()),

                char if char.is_digit(10) => tokens.push(self.scan_number()),

                _ => tokens.push(Err(LexerError::UnexpectedChar(char, self.line_count))),
            }
        }

        tokens
    }

    fn scan_comment(&mut self) {
        while let Some(char) = self.peek()
            && char != '\n'
        {
            self.next();
        }
    }

    fn scan_string(&mut self) -> Result<Token, LexerError> {
        let start = self.position;
        while let Some(char) = self.peek()
            && char != '"'
        {
            if self.source[self.position] == '\n' {
                self.line_count += 1;
            }
            self.next();
        }

        if self.position == self.source.len() {
            return Err(LexerError::UnterminatedString(
                self.source[start..].iter().collect(),
                self.line_count,
            ));
        }

        self.next();
        Ok(Token::String(
            self.source[start..self.position - 1].iter().collect(),
        ))
    }

    fn scan_number(&mut self) -> Result<Token, LexerError> {
        let start = self.position - 1;
        while let Some(char) = self.peek()
            && char.is_digit(10)
        {
            self.next();
        }

        if let Some([dot, digit]) = self.peek_two(2).as_deref() {
            if dot == &'.' && digit.is_digit(10) {
                self.next();
                self.next();
                while let Some(char) = self.peek()
                    && char.is_digit(10)
                {
                    self.next();
                }
            }
        }

        let number = self.source[start..self.position].iter().collect::<String>();
        number.parse().map_or(
            Err(LexerError::InvalidNumber(number, self.line_count)),
            |number| Ok(Token::Number(number)),
        )
    }
}
