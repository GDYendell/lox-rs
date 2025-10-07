use std::ops::Deref;

use crate::expression::{Expr };

pub trait AstDisplay {
    fn ast(&self) -> String;
}

impl AstDisplay for Expr {
    fn ast(&self) -> String {
        match self {
            Expr::BooleanLiteral(value) => value.to_string(),
            Expr::NumberLiteral(value) => value.to_string(),
            Expr::StringLiteral(value) => format!("\"{}\"", value),
            Expr::NilLiteral => "nil".to_string(),
            Expr::Unary(unary) => format!("({} {})", unary.operator, unary.right.deref().ast()),
            Expr::Binary(binary) => {
                format!(
                    "({} {} {})",
                    binary.left.ast(),
                    binary.operator,
                    binary.right.ast()
                )
            }
            Expr::Grouping(grouping) => format!("(group {})", grouping.expression.ast()),
        }
    }
}
