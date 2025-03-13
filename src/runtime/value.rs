use crate::alias::Name;
use crate::lexer::{Literal, Token};

#[derive(Debug, PartialEq)]
pub enum Value {
    RValue(Literal),
    LValue(Name),
}

impl TryFrom<Token> for Value {
    type Error = ();

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::Identifier(i) => Ok(Self::LValue(Name::from(i))),
            Token::Literal(i) => Ok(Self::RValue(i)),
            _ => Err(()),
        }
    }
}
