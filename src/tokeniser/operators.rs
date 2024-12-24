use crate::errors::TokenizeError;

#[derive(PartialEq, Eq, Debug)]
pub enum Operators {
    // math ops
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,

    // logical ops
    And,
    Or,
    Not,
    BitAnd,
    BitOr,
    BitXor,

    // relational ops
    Equals,
    NotEquals,
    LessThan,
    LessThanEquals,
    GreaterThan,
    GreaterThanEquals,

    // misc.
    Dot,
    End,
    Arrow,
    Comma,
}

impl TryFrom<&str> for Operators {
    type Error = TokenizeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "+" => Ok(Operators::Add),
            "-" => Ok(Operators::Sub),
            "*" => Ok(Operators::Mul),
            "/" => Ok(Operators::Div),
            "%" => Ok(Operators::Mod),
            "=" => Ok(Operators::Assign),
            "+=" => Ok(Operators::AddAssign),
            "-=" => Ok(Operators::SubAssign),
            "*=" => Ok(Operators::MulAssign),
            "/=" => Ok(Operators::DivAssign),
            "%=" => Ok(Operators::ModAssign),
            "&" => Ok(Operators::BitAnd),
            "|" => Ok(Operators::BitOr),
            "^" => Ok(Operators::BitXor),
            "&&" => Ok(Operators::And),
            "||" => Ok(Operators::Or),
            "!" => Ok(Operators::Not),
            "<" => Ok(Operators::LessThan),
            ">" => Ok(Operators::GreaterThan),
            "==" => Ok(Operators::Equals),
            "<=" => Ok(Operators::LessThanEquals),
            ">=" => Ok(Operators::GreaterThanEquals),
            "!=" => Ok(Operators::NotEquals),
            "." => Ok(Operators::Dot),
            ";" => Ok(Operators::End),
            "->" => Ok(Operators::Arrow),
            "," => Ok(Operators::Comma),
            other => Err(TokenizeError::InvalidToken(other.into())),
        }
    }
}
