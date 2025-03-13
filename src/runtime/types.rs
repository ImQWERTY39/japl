use crate::lexer::Keyword;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Type {
    Int8,
    Int16,
    Int32,
    Int64,
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Float32,
    Float64,
    Boolean,
    Character,
}

impl Type {
    pub fn size(&self) -> usize {
        match self {
            Type::Int8 => 1,
            Type::Int16 => 2,
            Type::Int32 => 4,
            Type::Int64 => 8,
            Type::Uint8 => 1,
            Type::Uint16 => 2,
            Type::Uint32 => 4,
            Type::Uint64 => 8,
            Type::Float32 => 4,
            Type::Float64 => 8,
            Type::Boolean => 1,
            Type::Character => 4,
        }
    }
}

impl TryFrom<Keyword> for Type {
    type Error = ();

    fn try_from(value: Keyword) -> Result<Self, Self::Error> {
        match value {
            Keyword::Int8 => Ok(Self::Int8),
            Keyword::Int16 => Ok(Self::Int16),
            Keyword::Int32 => Ok(Self::Int32),
            Keyword::Int64 => Ok(Self::Int64),
            Keyword::Uint8 => Ok(Self::Uint8),
            Keyword::Uint16 => Ok(Self::Uint16),
            Keyword::Uint32 => Ok(Self::Uint32),
            Keyword::Uint64 => Ok(Self::Uint64),
            Keyword::Float32 => Ok(Self::Float32),
            Keyword::Float64 => Ok(Self::Float64),
            Keyword::Character => Ok(Self::Character),
            Keyword::Boolean => Ok(Self::Boolean),
            _ => todo!(),
        }
    }
}
