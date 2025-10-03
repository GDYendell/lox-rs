use std::ops::Deref;

use crate::expression::{Expr, Literal};

pub trait AstDisplay {
    fn ast(&self) -> String;
}

impl AstDisplay for Expr {
    fn ast(&self) -> String {
        match self {
            Expr::LiteralExpr(Literal::Boolean(value)) => value.to_string(),
            Expr::LiteralExpr(Literal::Number(value)) => value.to_string(),
            Expr::LiteralExpr(Literal::String(value)) => format!("\"{}\"", value),
            Expr::LiteralExpr(Literal::Nil) => "nil".to_string(),
            Expr::UnaryExpr(unary) => format!("({} {})", unary.operator, unary.right.deref().ast()),
            Expr::BinaryExpr(binary) => {
                format!(
                    "({} {} {})",
                    binary.left.ast(),
                    binary.operator,
                    binary.right.ast()
                )
            }
            Expr::GroupingExpr(grouping) => format!("(group {})", grouping.expression.ast()),
        }
    }
}
