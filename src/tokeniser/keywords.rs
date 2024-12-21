use crate::errors::TokenizeError;

#[derive(PartialEq, Eq, Debug)]
pub enum Keywords {
    Fn,
    Return,
    I32,
}

impl TryFrom<&str> for Keywords {
    type Error = TokenizeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "fn" => Ok(Keywords::Fn),
            "return" => Ok(Keywords::Return),
            "i32" => Ok(Keywords::I32),
            other => Err(TokenizeError::InvalidKeyword(other.into())),
        }
    }
}
