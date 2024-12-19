pub type StrPtr = Box<str>;

pub const KEYWORDS: [&str; 3] = ["fn", "i32", "return"];
pub const OPERATORS: [&str; 26] = [
    "+", "-", "*", "/", "%", // binary operators
    "=", "+=", "-=", "*=", "/=", "%=", // binary assign?
    "&", "|", "^", // bitwise
    "&&", "||", "!", // logical
    "<", ">", "==", "<=", ">=", "!=", // relation
    ".", ";", "->", // misc.
];
pub const BRACKETS: [&str; 6] = ["(", ")", "{", "}", "[", "]"];
