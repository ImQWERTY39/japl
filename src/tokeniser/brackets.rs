use crate::errors::TokenizeError;

#[derive(PartialEq, Eq, Debug)]
pub enum Brackets {
    OpenParen,
    CloseParen,
    OpenSquare,
    CloseSquare,
    OpenCurly,
    CloseCurly,
}

impl TryFrom<&str> for Brackets {
    type Error = TokenizeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "(" => Ok(Brackets::OpenParen),
            ")" => Ok(Brackets::CloseParen),
            "[" => Ok(Brackets::OpenSquare),
            "]" => Ok(Brackets::CloseSquare),
            "{" => Ok(Brackets::OpenCurly),
            "}" => Ok(Brackets::CloseCurly),
            other => Err(TokenizeError::InvalidToken(other.into())),
        }
    }
}
