use super::StrPtr;
use crate::errors::TokenizeError;

#[derive(PartialEq, Debug)]
pub enum LiteralKind {
    Integer(i64),
    Float(f64),
    Character(char),
    String(StrPtr),
}

impl TryFrom<&str> for LiteralKind {
    type Error = TokenizeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(i) = value.parse::<i64>() {
            return Ok(LiteralKind::Integer(i));
        }

        if let Ok(i) = value.parse::<f64>() {
            return Ok(LiteralKind::Float(i));
        }

        if value.starts_with('\'') && value.ends_with('\'') && value.len() == 3 {
            return Ok(LiteralKind::Character(
                value
                    .chars()
                    .nth(1)
                    .expect("Length has to be atleast 3, so index 1 exists"),
            ));
        }

        if value.starts_with("\'\\") && value.ends_with('\'') && value.len() == 4 {
            return Ok(LiteralKind::Character(as_escaped(
                value
                    .chars()
                    .nth(2)
                    .expect("Length is 4, so index 2 exists"),
            )?));
        }

        if value.starts_with('"') && value.ends_with('"') && value.len() > 2 {
            return Ok(LiteralKind::String(value[1..value.len() - 1].into()));
        }

        Err(TokenizeError::InvalidLiteral(value.into()))
    }
}

fn as_escaped(escape_char: char) -> Result<char, TokenizeError> {
    match escape_char {
        'n' => Ok('\n'),
        't' => Ok('\t'),
        'r' => Ok('\r'),
        '0' => Ok('\0'),
        error_chr => {
            let mut invalid_token = String::from("\\");
            invalid_token.push(error_chr);
            Err(TokenizeError::InvalidLiteral(
                invalid_token.into_boxed_str(),
            ))
        }
    }
}
