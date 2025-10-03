use crate::token::Token;


#[derive(Debug, PartialEq)]
pub enum Literal {
    Boolean(bool),
    Number(f64),
    String(String),
    Nil,
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    LiteralExpr(Literal),
    UnaryExpr(Unary),
    BinaryExpr(Binary),
    GroupingExpr(Grouping),
}

#[derive(Debug, PartialEq)]
pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}

impl Unary {
    pub fn new(operator: Token, right: Expr) -> Unary {
        Unary {
            operator,
            right: Box::new(right),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

impl Binary {
    pub fn new(left: Expr, operator: Token, right: Expr) -> Binary {
        Binary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Grouping {
    pub expression: Box<Expr>,
}

impl Grouping {
    pub fn new(expression: Expr) -> Grouping {
        Grouping {
            expression: Box::new(expression),
        }
    }

}
