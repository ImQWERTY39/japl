use std::collections::HashMap;

use crate::alias::Str;
use crate::error::JAPLError;
use crate::lexer::{Keyword, Symbol, Token};
use crate::runtime::{BinOperator, Instruction, UnOperator, Value};

mod convert;

pub fn parse(tokens: Vec<Token>) -> Result<(Vec<Instruction>, HashMap<Str, usize>), JAPLError> {
    let mut token_iter = tokens.into_iter().peekable();

    let mut instruction_set = Vec::new();
    let mut labels = HashMap::new();

    while let Some(cur_tkn) = token_iter.next() {
        if let Token::Keyword(kw) = cur_tkn {
            match kw {
                Keyword::Add => instruction_set.push(Instruction::BinaryOp(
                    BinOperator::Add,
                    convert::get_register_name(token_iter.next())?,
                    convert::get_register_name(token_iter.next())?,
                    convert::get_register_name(token_iter.next())?,
                )),
                Keyword::Subtract => instruction_set.push(Instruction::BinaryOp(
                    BinOperator::Subtract,
                    convert::get_register_name(token_iter.next())?,
                    convert::get_register_name(token_iter.next())?,
                    convert::get_register_name(token_iter.next())?,
                )),
                Keyword::Multiply => instruction_set.push(Instruction::BinaryOp(
                    BinOperator::Multiply,
                    convert::get_register_name(token_iter.next())?,
                    convert::get_register_name(token_iter.next())?,
                    convert::get_register_name(token_iter.next())?,
                )),
                Keyword::Divide => instruction_set.push(Instruction::BinaryOp(
                    BinOperator::Divide,
                    convert::get_register_name(token_iter.next())?,
                    convert::get_register_name(token_iter.next())?,
                    convert::get_register_name(token_iter.next())?,
                )),
                Keyword::Modulus => instruction_set.push(Instruction::BinaryOp(
                    BinOperator::Modulus,
                    convert::get_register_name(token_iter.next())?,
                    convert::get_register_name(token_iter.next())?,
                    convert::get_register_name(token_iter.next())?,
                )),
                Keyword::And => instruction_set.push(Instruction::BinaryOp(
                    BinOperator::And,
                    convert::get_register_name(token_iter.next())?,
                    convert::get_register_name(token_iter.next())?,
                    convert::get_register_name(token_iter.next())?,
                )),
                Keyword::Or => instruction_set.push(Instruction::BinaryOp(
                    BinOperator::Or,
                    convert::get_register_name(token_iter.next())?,
                    convert::get_register_name(token_iter.next())?,
                    convert::get_register_name(token_iter.next())?,
                )),
                Keyword::Not => instruction_set.push(Instruction::UnaryOp(
                    UnOperator::Not,
                    convert::get_register_name(token_iter.next())?,
                    convert::get_register_name(token_iter.next())?,
                )),
                Keyword::Xor => instruction_set.push(Instruction::BinaryOp(
                    BinOperator::Xor,
                    convert::get_register_name(token_iter.next())?,
                    convert::get_register_name(token_iter.next())?,
                    convert::get_register_name(token_iter.next())?,
                )),
                Keyword::Equals => instruction_set.push(Instruction::BinaryOp(
                    BinOperator::Equals,
                    convert::get_register_name(token_iter.next())?,
                    convert::get_register_name(token_iter.next())?,
                    convert::get_register_name(token_iter.next())?,
                )),
                Keyword::NotEquals => instruction_set.push(Instruction::BinaryOp(
                    BinOperator::NotEquals,
                    convert::get_register_name(token_iter.next())?,
                    convert::get_register_name(token_iter.next())?,
                    convert::get_register_name(token_iter.next())?,
                )),
                Keyword::LessThan => instruction_set.push(Instruction::BinaryOp(
                    BinOperator::LessThan,
                    convert::get_register_name(token_iter.next())?,
                    convert::get_register_name(token_iter.next())?,
                    convert::get_register_name(token_iter.next())?,
                )),
                Keyword::LessThanEqualTo => instruction_set.push(Instruction::BinaryOp(
                    BinOperator::LessThanEqualTo,
                    convert::get_register_name(token_iter.next())?,
                    convert::get_register_name(token_iter.next())?,
                    convert::get_register_name(token_iter.next())?,
                )),
                Keyword::GreaterThan => instruction_set.push(Instruction::BinaryOp(
                    BinOperator::GreaterThan,
                    convert::get_register_name(token_iter.next())?,
                    convert::get_register_name(token_iter.next())?,
                    convert::get_register_name(token_iter.next())?,
                )),
                Keyword::GreaterThanEqualTo => instruction_set.push(Instruction::BinaryOp(
                    BinOperator::GreaterThanEqualTo,
                    convert::get_register_name(token_iter.next())?,
                    convert::get_register_name(token_iter.next())?,
                    convert::get_register_name(token_iter.next())?,
                )),
                Keyword::LeftShift => instruction_set.push(Instruction::BinaryOp(
                    BinOperator::LeftShift,
                    convert::get_register_name(token_iter.next())?,
                    convert::get_register_name(token_iter.next())?,
                    convert::get_register_name(token_iter.next())?,
                )),
                Keyword::RightShift => instruction_set.push(Instruction::BinaryOp(
                    BinOperator::RightShift,
                    convert::get_register_name(token_iter.next())?,
                    convert::get_register_name(token_iter.next())?,
                    convert::get_register_name(token_iter.next())?,
                )),
                Keyword::Increment => instruction_set.push(Instruction::UnaryOp(
                    UnOperator::Increment,
                    convert::get_register_name(token_iter.next())?,
                    convert::get_register_name(token_iter.next())?,
                )),
                Keyword::Decrement => instruction_set.push(Instruction::UnaryOp(
                    UnOperator::Decrement,
                    convert::get_register_name(token_iter.next())?,
                    convert::get_register_name(token_iter.next())?,
                )),

                Keyword::Push => {
                    let var_type = convert::get_variable_type(token_iter.next())?;
                    let var_name = convert::get_ident_name(token_iter.next())?;

                    instruction_set.push(Instruction::Push(var_type, var_name))
                }
                Keyword::Set => {
                    let var_name = convert::get_ident_name(token_iter.next())?;
                    let value = Value::try_from(
                        token_iter
                            .next()
                            .ok_or(JAPLError::InvalidArgument("Missing token: Value".into()))?,
                    )
                    .map_err(|_| JAPLError::InvalidArgument("Expected token: Value".into()))?;

                    instruction_set.push(Instruction::Set(var_name, value));
                }

                Keyword::Load => {
                    let value = Value::try_from(
                        token_iter
                            .next()
                            .ok_or(JAPLError::InvalidArgument("Missing token: Value".into()))?,
                    )
                    .map_err(|_| JAPLError::InvalidArgument("Expected token: Value".into()))?;
                    let reg = convert::get_register_name(token_iter.next())?;

                    instruction_set.push(Instruction::Load(value, reg));
                }
                Keyword::Unload => {
                    let reg = convert::get_register_name(token_iter.next())?;
                    let var_name = convert::get_ident_name(token_iter.next())?;

                    instruction_set.push(Instruction::Unload(reg, var_name));
                }
                Keyword::Move => todo!(),
                Keyword::Call => todo!(),
                Keyword::CallIf => todo!(),
                Keyword::Jump => {
                    let label = convert::get_label_name(token_iter.next())?;
                    instruction_set.push(Instruction::Jump(label));
                }

                Keyword::JumpIf => {
                    let label = convert::get_label_name(token_iter.next())?;
                    let reg = convert::get_register_name(token_iter.next())?;
                    instruction_set.push(Instruction::JumpIf(label, reg));
                }
                _ => unimplemented!(),
            }
        } else if let Token::Identifier(ident) = cur_tkn {
            if !matches!(token_iter.peek(), Some(Token::Symbol(Symbol::Colon))) {
                panic!("Labels must be follow by a ':'");
            }

            labels.insert(ident, instruction_set.len().checked_sub(1).unwrap_or(0));
        }
    }

    Ok((instruction_set, labels))
}
