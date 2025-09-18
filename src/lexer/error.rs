use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum LexerError {
    UnexpectedChar(char, usize),
    UnterminatedString(String, usize),
    InvalidNumber(String, usize),
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexerError::UnexpectedChar(char, line) => {
                write!(f, "Line {}: Unexpected character: '{}'", line, char)
            }
            LexerError::UnterminatedString(string, line) => {
                write!(f, "Line {}: Unterminated string: '{}'", line, string)
            }
            LexerError::InvalidNumber(number, line) => {
                write!(f, "Line {}: Invalid number: '{}'", line, number)
            }
        }
    }
}

impl std::error::Error for LexerError {}

impl From<LexerError> for String {
    fn from(e: LexerError) -> Self {
        e.to_string()
    }
}
