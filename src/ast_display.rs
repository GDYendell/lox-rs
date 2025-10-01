use std::ops::Deref;

use crate::expression::Expr;

pub trait AstDisplay {
    fn ast(&self) -> String;
}

impl AstDisplay for Expr {
    fn ast(&self) -> String {
        match self {
            Expr::NumberLiteralExpr(value) => value.to_string(),
            Expr::StringLiteralExpr(value) => format!("\"{}\"", value),
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
