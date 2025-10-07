use std::fmt::Display;

use crate::token::Token;

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
