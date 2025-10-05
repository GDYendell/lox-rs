use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenValue {
    String(String),
    Number(f64),
}

impl Display for TokenValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenValue::String(value) => write!(f, "\"{}\"", value),
            TokenValue::Number(value) => write!(f, "{}", value),
        }
    }
}
