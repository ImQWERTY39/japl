use crate::error::JAPLError;

#[derive(Debug, PartialEq, Eq)]
pub enum Keyword {
    // body decleration
    Struct,
    Function,
    End,

    // types
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
    Character,
    Boolean,

    // operations
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,

    // Bitwise
    And,
    Or,
    Not,
    Xor,

    // relation
    Equals,
    NotEquals,
    LessThan,
    GreaterThan,
    LessThanEqualTo,
    GreaterThanEqualTo,

    // other
    Increment,
    Decrement,
    LeftShift,
    RightShift,

    // statements
    Push,
    Set,
    Load,
    Unload,
    Move,

    // control flow
    Call,
    CallIf,
    Jump,
    JumpIf,

    // Register
    RegClassA,
    RegisterA0,
    RegisterA1,
    RegisterA2,
    RegisterA3,

    RegClassB,
    RegisterB0,
    RegisterB1,
    RegisterB2,
    RegisterB3,

    RegClassC,
    RegisterC0,
    RegisterC1,
    RegisterC2,
    RegisterC3,

    RegClassD,
    RegisterD0,
    RegisterD1,
    RegisterD2,
    RegisterD3,

    RegClassF,
    RegisterF0,
    RegisterF1,
    RegisterF2,
    RegisterF3,

    RegClassG,
    RegisterG0,
    RegisterG1,
    RegisterG2,
    RegisterG3,

    RegClassI,
    RegisterI0,
    RegisterI1,
    RegisterI2,
    RegisterI3,
}

impl TryFrom<&str> for Keyword {
    type Error = JAPLError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "struct" => Ok(Self::Struct),
            "fn" => Ok(Self::Function),
            "end" => Ok(Self::End),

            "int8" => Ok(Self::Int8),
            "int16" => Ok(Self::Int16),
            "int32" => Ok(Self::Int32),
            "int64" => Ok(Self::Int64),
            "uint8" => Ok(Self::Uint8),
            "uint16" => Ok(Self::Uint16),
            "uint32" => Ok(Self::Uint32),
            "uint64" => Ok(Self::Uint64),
            "float32" => Ok(Self::Float32),
            "float64" => Ok(Self::Float64),
            "char" => Ok(Self::Character),
            "bool" => Ok(Self::Boolean),

            "add" => Ok(Self::Add),
            "sub" => Ok(Self::Subtract),
            "mul" => Ok(Self::Multiply),
            "div" => Ok(Self::Divide),
            "mod" => Ok(Self::Modulus),
            "and" => Ok(Self::And),
            "or" => Ok(Self::Or),
            "not" => Ok(Self::Not),
            "xor" => Ok(Self::Xor),

            "eq" => Ok(Self::Equals),
            "ne" => Ok(Self::NotEquals),
            "lt" => Ok(Self::LessThan),
            "le" => Ok(Self::LessThanEqualTo),
            "gt" => Ok(Self::GreaterThan),
            "ge" => Ok(Self::GreaterThanEqualTo),

            "push" => Ok(Self::Push),
            "set" => Ok(Self::Set),
            "load" => Ok(Self::Load),
            "unload" => Ok(Self::Unload),
            "move" => Ok(Self::Move),

            "call" => Ok(Self::Call),
            "callif" => Ok(Self::CallIf),
            "jump" => Ok(Self::Jump),
            "jumpif" => Ok(Self::JumpIf),

            "a" => Ok(Self::RegClassA),
            "a0" => Ok(Self::RegisterA0),
            "a1" => Ok(Self::RegisterA1),
            "a2" => Ok(Self::RegisterA2),
            "a3" => Ok(Self::RegisterA3),

            "b" => Ok(Self::RegClassB),
            "b0" => Ok(Self::RegisterB0),
            "b1" => Ok(Self::RegisterB1),
            "b2" => Ok(Self::RegisterB2),
            "b3" => Ok(Self::RegisterB3),

            "c" => Ok(Self::RegClassC),
            "c0" => Ok(Self::RegisterC0),
            "c1" => Ok(Self::RegisterC1),
            "c2" => Ok(Self::RegisterC2),
            "c3" => Ok(Self::RegisterC3),

            "d" => Ok(Self::RegClassD),
            "d0" => Ok(Self::RegisterD0),
            "d1" => Ok(Self::RegisterD1),
            "d2" => Ok(Self::RegisterD2),
            "d3" => Ok(Self::RegisterD3),

            "f" => Ok(Self::RegClassF),
            "f0" => Ok(Self::RegisterF0),
            "f1" => Ok(Self::RegisterF1),
            "f2" => Ok(Self::RegisterF2),
            "f3" => Ok(Self::RegisterF3),

            "g" => Ok(Self::RegClassG),
            "g0" => Ok(Self::RegisterG0),
            "g1" => Ok(Self::RegisterG1),
            "g2" => Ok(Self::RegisterG2),
            "g3" => Ok(Self::RegisterG3),

            "i" => Ok(Self::RegClassI),
            "i0" => Ok(Self::RegisterI0),
            "i1" => Ok(Self::RegisterI1),
            "i2" => Ok(Self::RegisterI2),
            "i3" => Ok(Self::RegisterI3),

            "inc" => Ok(Self::Increment),
            "dec" => Ok(Self::Decrement),
            "ls" => Ok(Self::LeftShift),
            "rs" => Ok(Self::RightShift),

            _ => Err(JAPLError::InvalidIdentifier(value.into())),
        }
    }
}
