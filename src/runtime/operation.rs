#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BinOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,

    LeftShift,
    RightShift,

    And,
    Or,
    Xor,

    Equals,
    NotEquals,
    LessThan,
    GreaterThan,
    LessThanEqualTo,
    GreaterThanEqualTo,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum UnOperator {
    Not,
    Increment,
    Decrement,
}
