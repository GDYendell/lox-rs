use crate::token::Token;

#[derive(Debug, PartialEq)]
pub enum Expr {
    BooleanLiteral(bool),
    NumberLiteral(f64),
    StringLiteral(String),
    NilLiteral,
    Unary(UnaryExpr),
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
}

#[derive(Debug, PartialEq)]
pub struct UnaryExpr {
    pub operator: Token,
    pub right: Box<Expr>,
}

impl UnaryExpr {
    pub fn new(operator: Token, right: Expr) -> UnaryExpr {
        UnaryExpr {
            operator,
            right: Box::new(right),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

impl BinaryExpr {
    pub fn new(left: Expr, operator: Token, right: Expr) -> BinaryExpr {
        BinaryExpr {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct GroupingExpr {
    pub expression: Box<Expr>,
}

impl GroupingExpr {
    pub fn new(expression: Expr) -> GroupingExpr {
        GroupingExpr {
            expression: Box::new(expression),
        }
    }
}
