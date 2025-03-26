use crate::alias::Str;
use crate::error::JAPLError;

use super::keyword::Keyword;
use super::literal::Literal;
use super::symbol::Symbol;

#[derive(Debug, PartialEq)]
pub enum Token {
    Keyword(Keyword),
    Symbol(Symbol),
    Identifier(Str),
    Literal(Literal),
}

impl TryFrom<&str> for Token {
    type Error = JAPLError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(i) = Keyword::try_from(value) {
            return Ok(Self::Keyword(i));
        }

        if let Ok(i) = Symbol::try_from(value) {
            return Ok(Self::Symbol(i));
        }

        if let Ok(i) = Literal::try_from(value) {
            return Ok(Self::Literal(i));
        }

        if valid_identifier(value) {
            return Ok(Self::Identifier(value.into()));
        }

        Err(JAPLError::InvalidIdentifier(value.into()))
    }
}

fn valid_identifier(value: &str) -> bool {
    if value.is_empty() {
        return false;
    }

    let mut char_iter = value.chars();
    let first = char_iter.next().expect("`value` cannot be empty");

    (first.is_ascii_alphabetic() || first == '_')
        && char_iter.all(|c| c.is_alphanumeric() || c == '_')
}
