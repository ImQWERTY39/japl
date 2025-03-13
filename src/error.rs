use crate::alias::Str;

#[derive(Debug)]
pub enum JAPLError {
    InvalidIdentifier(Str),
    IncorrectArgument(Str),
}

impl std::fmt::Display for JAPLError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JAPLError::InvalidIdentifier(i) => write!(f, "Invalid identifier: {}", i),
            JAPLError::IncorrectArgument(i) => write!(f, "Invalid arguments passed: {}", i),
        }
    }
}

impl std::error::Error for JAPLError {}
