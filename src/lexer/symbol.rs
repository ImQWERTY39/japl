use crate::error::JAPLError;

#[derive(Debug, PartialEq, Eq)]
pub enum Symbol {
    Colon,
    Semicolon,
    Dot,
    Comma,
    RoundOpen,
    RoundClose,
    SquareOpen,
    SquareClose,
    CurlyOpen,
    CurlyClose,
}

impl TryFrom<&str> for Symbol {
    type Error = JAPLError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            ":" => Ok(Self::Colon),
            ";" => Ok(Self::Semicolon),
            "." => Ok(Self::Dot),
            "," => Ok(Self::Comma),
            "(" => Ok(Self::RoundOpen),
            ")" => Ok(Self::RoundClose),
            "[" => Ok(Self::SquareOpen),
            "]" => Ok(Self::SquareClose),
            "{" => Ok(Self::CurlyOpen),
            "}" => Ok(Self::CurlyClose),
            _ => Err(JAPLError::InvalidIdentifier(value.into())),
        }
    }
}
