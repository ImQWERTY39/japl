use crate::alias::{Name, Str};

use super::{
    operation::{BinOperator, UnOperator},
    RegisterName, Type, Value,
};

#[derive(Debug)]
pub enum Instruction {
    Push(Type, Name),
    Set(Name, Value),

    Load(Value, RegisterName),
    Unload(RegisterName, Name),
    Move(RegisterName, RegisterName),

    Call(Str),
    CallIf(Str),
    Jump(Str),
    JumpIf(Str, RegisterName),

    BinaryOp(BinOperator, RegisterName, RegisterName, RegisterName),
    UnaryOp(UnOperator, RegisterName, RegisterName),
}
