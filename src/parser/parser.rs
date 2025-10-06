use super::error::ParserError;

use crate::{
    expression::{Binary, Expr, Grouping, Literal, Unary},
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
        return self.tokens.get(self.current).cloned();
    }

    fn next(&mut self) -> Option<Token> {
        self.peek().map(|token| {
            self.current += 1;
            token
        })
    }

    fn match_next(&mut self, kinds: &[TokenKind]) -> Option<Token> {
        if let Some(token) = self.peek() {
            if kinds.contains(&token.kind) {
                return self.next();
            }
        }
        None
    }

    pub fn parse(&mut self) -> Result<Expr, ParserError> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr, ParserError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.comparison()?;

        while let Some(operator) = self.match_next(&[TokenKind::BangEqual, TokenKind::EqualEqual]) {
            expr = Expr::BinaryExpr(Binary::new(expr, operator, self.comparison()?));
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
            expr = Expr::BinaryExpr(Binary::new(expr, operator, self.term()?));
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.factor()?;

        while let Some(operator) = self.match_next(&[TokenKind::Minus, TokenKind::Plus]) {
            expr = Expr::BinaryExpr(Binary::new(expr, operator, self.factor()?));
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.unary()?;

        while let Some(operator) = self.match_next(&[TokenKind::Slash, TokenKind::Star]) {
            expr = Expr::BinaryExpr(Binary::new(expr, operator, self.unary()?));
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParserError> {
        if let Some(operator) = self.match_next(&[TokenKind::Bang, TokenKind::Minus]) {
            return Ok(Expr::UnaryExpr(Unary::new(operator, self.unary()?)));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParserError> {
        if let Some(Token { kind, value }) = self.next() {
            match kind {
                TokenKind::False => Ok(Expr::LiteralExpr(Literal::Boolean(false))),
                TokenKind::True => Ok(Expr::LiteralExpr(Literal::Boolean(true))),
                TokenKind::Nil => Ok(Expr::LiteralExpr(Literal::Nil)),
                TokenKind::Number | TokenKind::String => {
                    if let Some(value) = value {
                        match value {
                            TokenValue::String(value) => {
                                Ok(Expr::LiteralExpr(Literal::String(value)))
                            }
                            TokenValue::Number(value) => {
                                Ok(Expr::LiteralExpr(Literal::Number(value)))
                            }
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

        Ok(Expr::GroupingExpr(Grouping::new(expr)))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        expression::{Binary, Expr, Grouping, Literal, Unary},
        parser::{Parser, error::ParserError},
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
            crate::Expr::BinaryExpr(Binary::new(
                Expr::LiteralExpr(Literal::Number(1.0)),
                Token::from(TokenKind::EqualEqual),
                Expr::LiteralExpr(Literal::Number(1.0))
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
            crate::Expr::BinaryExpr(Binary::new(
                Expr::LiteralExpr(Literal::Number(1.0)),
                Token::from(TokenKind::Greater),
                Expr::LiteralExpr(Literal::Number(2.0))
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
            crate::Expr::BinaryExpr(Binary::new(
                Expr::LiteralExpr(Literal::Number(1.0)),
                Token::from(TokenKind::Plus),
                Expr::LiteralExpr(Literal::Number(2.0))
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
            crate::Expr::BinaryExpr(Binary::new(
                Expr::LiteralExpr(Literal::Number(1.0)),
                Token::from(TokenKind::Star),
                Expr::LiteralExpr(Literal::Number(2.0))
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
            crate::Expr::UnaryExpr(Unary::new(
                Token::from(TokenKind::Minus),
                Expr::LiteralExpr(Literal::Number(1.0))
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
            Expr::BinaryExpr(Binary::new(
                Expr::BinaryExpr(Binary::new(
                    Expr::GroupingExpr(Grouping::new(Expr::BinaryExpr(Binary::new(
                        Expr::LiteralExpr(Literal::Number(1.0)),
                        Token::from(TokenKind::Plus),
                        Expr::LiteralExpr(Literal::Number(2.0))
                    )))),
                    Token::from(TokenKind::Star),
                    Expr::LiteralExpr(Literal::Number(3.0))
                )),
                Token::from(TokenKind::Minus),
                Expr::BinaryExpr(Binary::new(
                    Expr::LiteralExpr(Literal::Number(4.0)),
                    Token::from(TokenKind::Slash),
                    Expr::LiteralExpr(Literal::Number(2.0))
                ))
            ))
        );
        Ok(())
    }
}
