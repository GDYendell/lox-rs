use crate::token::Token;

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

                char if char.is_alphabetic() || char == '_' => tokens.push(self.scan_word()),

                _ => tokens.push(Err(LexerError::UnexpectedChar(char, self.line_count))),
            }
        }

        tokens.push(Ok(Token::EOF));

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

    fn scan_word(&mut self) -> Result<Token, LexerError> {
        let start = self.position - 1;
        while let Some(char) = self.peek()
            && (char.is_alphanumeric() || char == '_')
        {
            self.next();
        }

        let word = self.source[start..self.position].iter().collect::<String>();
        match word.as_str() {
            "and" => Ok(Token::And),
            "class" => Ok(Token::Class),
            "else" => Ok(Token::Else),
            "false" => Ok(Token::False),
            "for" => Ok(Token::For),
            "fun" => Ok(Token::Fun),
            "if" => Ok(Token::If),
            "nil" => Ok(Token::Nil),
            "or" => Ok(Token::Or),
            "print" => Ok(Token::Print),
            "return" => Ok(Token::Return),
            "super" => Ok(Token::Super),
            "this" => Ok(Token::This),
            "true" => Ok(Token::True),
            "var" => Ok(Token::Var),
            "while" => Ok(Token::While),
            _ => Ok(Token::Identifier(word)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number() {
        let mut lexer = Lexer::new("1".to_string());
        assert_eq!(
            lexer.scan_tokens(),
            vec![Ok(Token::Number(1.0)), Ok(Token::EOF)]
        );
    }

    #[test]
    fn test_string() {
        let mut lexer = Lexer::new("\"this is a string\"".to_string());
        assert_eq!(
            lexer.scan_tokens(),
            vec![
                Ok(Token::String("this is a string".to_string())),
                Ok(Token::EOF)
            ]
        );
    }

    #[test]
    fn test_expression() {
        let mut lexer = Lexer::new("var _true = (true or false)".to_string());
        assert_eq!(
            lexer.scan_tokens(),
            vec![
                Ok(Token::Var),
                Ok(Token::Identifier("_true".to_string())),
                Ok(Token::Equal),
                Ok(Token::LeftParen),
                Ok(Token::True),
                Ok(Token::Or),
                Ok(Token::False),
                Ok(Token::RightParen),
                Ok(Token::EOF)
            ]
        );
    }
}
