use crate::constants::StrPtr;

#[derive(Debug)]
pub enum TokenizeError {
    InvalidToken(StrPtr),
    InvalidLiteral(StrPtr),
}

impl std::fmt::Display for TokenizeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenizeError::InvalidToken(i) => write!(f, "Invalid token: {i}"),
            TokenizeError::InvalidLiteral(i) => write!(f, "Invalid literal: {i}"),
        }
    }
}

impl std::error::Error for TokenizeError {}
