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

    pub fn parse(&mut self) -> Expr {
        self.expression()
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        if let Some(token) = self.peek() {
            let Token { ref kind, value: _ } = token;
            match kind {
                TokenKind::BangEqual | TokenKind::EqualEqual => {
                    self.next();
                    let right = self.equality();
                    expr = Expr::BinaryExpr(Binary::new(expr, token, right));
                }
                _ => (),
            }
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();

        if let Some(token) = self.peek() {
            let Token { ref kind, value: _ } = token;
            match kind {
                TokenKind::Greater
                | TokenKind::GreaterEqual
                | TokenKind::Less
                | TokenKind::LessEqual => {
                    self.next();
                    let right = self.term();
                    expr = Expr::BinaryExpr(Binary::new(expr, token, right));
                }
                _ => (),
            }
        }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        if let Some(token) = self.peek() {
            let Token { ref kind, value: _ } = token;
            match kind {
                TokenKind::Minus | TokenKind::Plus => {
                    self.next();
                    let right = self.factor();
                    expr = Expr::BinaryExpr(Binary::new(expr, token, right));
                }
                _ => (),
            }
        }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        if let Some(token) = self.peek() {
            let Token { ref kind, value: _ } = token;
            match kind {
                TokenKind::Slash | TokenKind::Star => {
                    self.next();
                    let right = self.unary();
                    expr = Expr::BinaryExpr(Binary::new(expr, token, right));
                }
                _ => (),
            }
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        let mut expr = self.primary();

        if let Some(token) = self.peek() {
            let Token { ref kind, value: _ } = token;
            match kind {
                TokenKind::Bang | TokenKind::Minus => {
                    self.next();
                    expr = Expr::UnaryExpr(Unary::new(token, expr));
                }
                _ => (),
            }
        }

        expr
    }

    fn primary(&mut self) -> Expr {
        if let Some(Token { kind, value }) = self.next() {
            match kind {
                TokenKind::False => Expr::LiteralExpr(Literal::Boolean(false)),
                TokenKind::True => Expr::LiteralExpr(Literal::Boolean(true)),
                TokenKind::Nil => Expr::LiteralExpr(Literal::Nil),
                TokenKind::Number | TokenKind::String => {
                    if let Some(value) = value {
                        match value {
                            TokenValue::String(value) => Expr::LiteralExpr(Literal::String(value)),
                            TokenValue::Number(value) => Expr::LiteralExpr(Literal::Number(value)),
                        }
                    } else {
                        todo!("String/Number Token without value")
                    }
                }
                TokenKind::LeftParen => self.parenthesis(),
                _ => todo!("Token is not a primary expression"),
            }
        } else {
            todo!("No tokens left")
        }
    }

    fn parenthesis(&mut self) -> Expr {
        let expr = self.expression();

        if self.next()
            != Some(Token {
                kind: TokenKind::RightParen,
                value: None,
            })
        {
            todo!("Parenthesis expression is not closed")
        }

        Expr::GroupingExpr(Grouping::new(expr))
    }
}
