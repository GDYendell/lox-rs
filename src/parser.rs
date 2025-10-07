use std::fmt::Display;

use crate::{
    expression::{BinaryExpr, Expr, GroupingExpr, UnaryExpr},
    token::{Token, TokenKind, TokenValue},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    fn peek(&self) -> Option<Token> {
        self.tokens.get(self.current).cloned()
    }

    fn next(&mut self) -> Option<Token> {
        self.peek().inspect(|_token| {
            self.current += 1;
        })
    }

    fn match_next(&mut self, kinds: &[TokenKind]) -> Option<Token> {
        if let Some(token) = self.peek()
            && kinds.contains(&token.kind)
        {
            return self.next();
        }
        None
    }

    pub fn parse(&mut self) -> Result<Expr, ParserError> {
        match self.expression() {
            Ok(expr) => Ok(expr),
            Err(err) => {
                self.synchronise();
                Err(err)
            }
        }
    }

    fn expression(&mut self) -> Result<Expr, ParserError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.comparison()?;

        while let Some(operator) = self.match_next(&[TokenKind::BangEqual, TokenKind::EqualEqual]) {
            expr = Expr::Binary(BinaryExpr::new(expr, operator, self.comparison()?));
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.term()?;

        while let Some(operator) = self.match_next(&[
            TokenKind::Greater,
            TokenKind::GreaterEqual,
            TokenKind::Less,
            TokenKind::LessEqual,
        ]) {
            expr = Expr::Binary(BinaryExpr::new(expr, operator, self.term()?));
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.factor()?;

        while let Some(operator) = self.match_next(&[TokenKind::Minus, TokenKind::Plus]) {
            expr = Expr::Binary(BinaryExpr::new(expr, operator, self.factor()?));
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.unary()?;

        while let Some(operator) = self.match_next(&[TokenKind::Slash, TokenKind::Star]) {
            expr = Expr::Binary(BinaryExpr::new(expr, operator, self.unary()?));
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParserError> {
        if let Some(operator) = self.match_next(&[TokenKind::Bang, TokenKind::Minus]) {
            return Ok(Expr::Unary(UnaryExpr::new(operator, self.unary()?)));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParserError> {
        if let Some(Token { kind, value }) = self.next() {
            match kind {
                TokenKind::False => Ok(Expr::BooleanLiteral(false)),
                TokenKind::True => Ok(Expr::BooleanLiteral(true)),
                TokenKind::Nil => Ok(Expr::NilLiteral),
                TokenKind::Number | TokenKind::String => {
                    if let Some(value) = value {
                        match value {
                            TokenValue::String(value) => Ok(Expr::StringLiteral(value)),
                            TokenValue::Number(value) => Ok(Expr::NumberLiteral(value)),
                        }
                    } else {
                        Err(ParserError::ExpectedPrimaryExpressionGot(Token {
                            kind,
                            value,
                        }))
                    }
                }
                TokenKind::LeftParen => self.parenthesis(),
                _ => Err(ParserError::ExpectedPrimaryExpressionGot(Token {
                    kind,
                    value,
                })),
            }
        } else {
            Err(ParserError::ExpectedExpression)
        }
    }

    fn parenthesis(&mut self) -> Result<Expr, ParserError> {
        let expr = self.expression()?;

        if self.next()
            != Some(Token {
                kind: TokenKind::RightParen,
                value: None,
            })
        {
            return Err(ParserError::UnclosedParenthesis);
        }

        Ok(Expr::Grouping(GroupingExpr::new(expr)))
    }

    fn synchronise(&mut self) {
        while let Some(Token {
            kind: token_kind,
            value: _,
        }) = self.peek()
        {
            match token_kind {
                TokenKind::Semicolon => {
                    self.next();
                    return;
                }
                TokenKind::Class
                | TokenKind::Fun
                | TokenKind::Var
                | TokenKind::For
                | TokenKind::If
                | TokenKind::While
                | TokenKind::Print
                | TokenKind::Return => return,
                _ => {
                    self.next();
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum ParserError {
    ExpectedExpression,
    ExpectedPrimaryExpressionGot(Token),
    UnclosedParenthesis,
}

impl From<&ParserError> for String {
    fn from(value: &ParserError) -> Self {
        match value {
            ParserError::ExpectedExpression => "Expected expression".to_string(),
            ParserError::ExpectedPrimaryExpressionGot(token) => {
                format!("Expected primary expression got {}", token)
            }
            ParserError::UnclosedParenthesis => "Unclosed parenthesis".to_string(),
        }
    }
}

impl From<ParserError> for String {
    fn from(value: ParserError) -> Self {
        String::from(&value)
    }
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        expression::{BinaryExpr, Expr, GroupingExpr, UnaryExpr},
        parser::{Parser, ParserError},
        token::{Token, TokenKind},
    };

    #[test]
    fn test_equality() -> Result<(), ParserError> {
        let tokens: Vec<Token> = vec![
            Token::from((TokenKind::Number, 1.0)),
            Token::from(TokenKind::EqualEqual),
            Token::from((TokenKind::Number, 1.0)),
        ];
        let mut parser = Parser::new(tokens);
        let expression = parser.parse();
        assert_eq!(
            expression?,
            crate::Expr::Binary(BinaryExpr::new(
                Expr::NumberLiteral(1.0),
                Token::from(TokenKind::EqualEqual),
                Expr::NumberLiteral(1.0)
            ))
        );
        Ok(())
    }

    #[test]
    fn test_comparison() -> Result<(), ParserError> {
        let tokens: Vec<Token> = vec![
            Token::from((TokenKind::Number, 1.0)),
            Token::from(TokenKind::Greater),
            Token::from((TokenKind::Number, 2.0)),
        ];
        let mut parser = Parser::new(tokens);
        let expression = parser.parse();
        assert_eq!(
            expression?,
            crate::Expr::Binary(BinaryExpr::new(
                Expr::NumberLiteral(1.0),
                Token::from(TokenKind::Greater),
                Expr::NumberLiteral(2.0)
            ))
        );
        Ok(())
    }

    #[test]
    fn test_term() -> Result<(), ParserError> {
        let tokens: Vec<Token> = vec![
            Token::from((TokenKind::Number, 1.0)),
            Token::from(TokenKind::Plus),
            Token::from((TokenKind::Number, 2.0)),
        ];
        let mut parser = Parser::new(tokens);
        let expression = parser.parse();
        assert_eq!(
            expression?,
            crate::Expr::Binary(BinaryExpr::new(
                Expr::NumberLiteral(1.0),
                Token::from(TokenKind::Plus),
                Expr::NumberLiteral(2.0)
            ))
        );
        Ok(())
    }

    #[test]
    fn test_factor() -> Result<(), ParserError> {
        let tokens: Vec<Token> = vec![
            Token::from((TokenKind::Number, 1.0)),
            Token::from(TokenKind::Star),
            Token::from((TokenKind::Number, 2.0)),
        ];
        let mut parser = Parser::new(tokens);
        let expression = parser.parse();
        assert_eq!(
            expression?,
            crate::Expr::Binary(BinaryExpr::new(
                Expr::NumberLiteral(1.0),
                Token::from(TokenKind::Star),
                Expr::NumberLiteral(2.0)
            ))
        );
        Ok(())
    }

    #[test]
    fn test_unary() -> Result<(), ParserError> {
        let tokens: Vec<Token> = vec![
            Token::from(TokenKind::Minus),
            Token::from((TokenKind::Number, 1.0)),
        ];
        let mut parser = Parser::new(tokens);
        let expression = parser.parse();
        assert_eq!(
            expression?,
            crate::Expr::Unary(UnaryExpr::new(
                Token::from(TokenKind::Minus),
                Expr::NumberLiteral(1.0)
            ))
        );
        Ok(())
    }

    #[test]
    fn test_maths() -> Result<(), ParserError> {
        // (1 + 2) * 3 - 4 / 2
        let tokens: Vec<Token> = vec![
            Token::from(TokenKind::LeftParen),
            Token::from((TokenKind::Number, 1.0)),
            Token::from(TokenKind::Plus),
            Token::from((TokenKind::Number, 2.0)),
            Token::from(TokenKind::RightParen),
            Token::from(TokenKind::Star),
            Token::from((TokenKind::Number, 3.0)),
            Token::from(TokenKind::Minus),
            Token::from((TokenKind::Number, 4.0)),
            Token::from(TokenKind::Slash),
            Token::from((TokenKind::Number, 2.0)),
        ];
        let mut parser = Parser::new(tokens);
        let expression = parser.parse();
        assert_eq!(
            expression?,
            Expr::Binary(BinaryExpr::new(
                Expr::Binary(BinaryExpr::new(
                    Expr::Grouping(GroupingExpr::new(Expr::Binary(BinaryExpr::new(
                        Expr::NumberLiteral(1.0),
                        Token::from(TokenKind::Plus),
                        Expr::NumberLiteral(2.0)
                    )))),
                    Token::from(TokenKind::Star),
                    Expr::NumberLiteral(3.0)
                )),
                Token::from(TokenKind::Minus),
                Expr::Binary(BinaryExpr::new(
                    Expr::NumberLiteral(4.0),
                    Token::from(TokenKind::Slash),
                    Expr::NumberLiteral(2.0)
                ))
            ))
        );
        Ok(())
    }
}
