use crate::errors::TokenizeError;

#[derive(PartialEq, Eq, Debug)]
pub enum Keywords {
    // code blocks?
    Fn,
    While,
    For,
    If,
    Else,

    // idk
    Return,
    Break,
    Continue,
    True,
    False,
    Const,

    // types
    I8,
    I16,
    I32,
    I64,
    I128,
    U8,
    U16,
    U32,
    U64,
    U128,
    Character,
    Boolean,
}

impl TryFrom<&str> for Keywords {
    type Error = TokenizeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "fn" => Ok(Keywords::Fn),
            "while" => Ok(Keywords::While),
            "for" => Ok(Keywords::For),
            "if" => Ok(Keywords::If),
            "else" => Ok(Keywords::Else),
            "return" => Ok(Keywords::Return),
            "break" => Ok(Keywords::Break),
            "continue" => Ok(Keywords::Continue),
            "true" => Ok(Keywords::True),
            "false" => Ok(Keywords::False),
            "const" => Ok(Keywords::Const),
            "i8" => Ok(Keywords::I8),
            "i16" => Ok(Keywords::I16),
            "i32" => Ok(Keywords::I32),
            "i64" => Ok(Keywords::I64),
            "i128" => Ok(Keywords::I128),
            "u8" => Ok(Keywords::U8),
            "u16" => Ok(Keywords::U16),
            "u32" => Ok(Keywords::U32),
            "u64" => Ok(Keywords::U64),
            "u128" => Ok(Keywords::U128),
            "char" => Ok(Keywords::Character),
            "bool" => Ok(Keywords::Boolean),
            other => Err(TokenizeError::InvalidKeyword(other.into())),
        }
    }
}
