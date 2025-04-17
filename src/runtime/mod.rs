use std::collections::HashMap;

use crate::alias::{Name, Str};

mod instruction;
pub use instruction::Instruction;

mod types;
pub use types::Type;

mod register;
pub use register::{Register, RegisterName};

mod operation;
pub use operation::{BinOperator, UnOperator};

mod value;
pub use value::Value;

pub fn run(instruction_set: Vec<Instruction>, labels: HashMap<Str, usize>) {
    let mut variables: Vec<(Name, Type, usize)> = Vec::new();

    let mut register = Register::default();
    let mut stack = [0u8; 16];

    let mut pc = 0;

    while pc < instruction_set.len() {
        execute(
            &instruction_set[pc],
            &mut variables,
            &mut register,
            &mut stack,
            &labels,
            &mut pc,
        );
        pc += 1;
    }

    println!("stack: {:?}", stack);
    println!("register: {:#?}", register);
}

fn execute(
    instruction: &Instruction,
    variables: &mut Vec<(Name, Type, usize)>,
    register: &mut Register,
    stack: &mut [u8],
    labels: &HashMap<Str, usize>,
    pc: &mut usize,
) {
    match instruction {
        Instruction::Push(var_type, name) => push_var(var_type.clone(), name.clone(), variables),
        Instruction::Set(var_name, value) => set_var(var_name.clone(), value, variables, stack),
        Instruction::Load(value, register_name) => {
            load_reg(value, register_name, variables, stack, register)
        }
        Instruction::Unload(reg, var_name) => {
            unload_reg(reg, var_name, variables, stack, register).unwrap()
        }
        Instruction::Move(_, _) => todo!(),
        Instruction::Call(_) => todo!(),
        Instruction::CallIf(_, _) => todo!(),
        Instruction::Jump(label) => *pc = labels[label],
        Instruction::JumpIf(label, reg) => {
            if register.i[reg.index()] {
                *pc = labels[label];
            }
        }

        Instruction::BinaryOp(op, src1, src2, dst) => {
            register.bin_operate(*op, *src1, *src2, *dst).unwrap()
        }
        Instruction::UnaryOp(op, src, dst) => register.un_operate(*op, *src, *dst).unwrap(),
    }
}

fn push_var(var_type: Type, name: Name, variables: &mut Vec<(Name, Type, usize)>) {
    let idx = variables
        .last()
        .map(|(_, prev_type, prev_idx)| prev_idx + prev_type.size())
        .unwrap_or(0);

    variables.push((name, var_type, idx))
}

fn set_var(var_name: Name, value: &Value, variables: &[(Name, Type, usize)], stack: &mut [u8]) {
    let (start, end) = find_var_idx(&var_name, variables);

    match value {
        Value::RValue(literal) => stack[start..end].copy_from_slice(&literal.as_bytes(end - start)),
        Value::LValue(var) => {
            let (start_clone, end_clone) = find_var_idx(var, variables);

            for (i, j) in (start..end).zip(start_clone..end_clone) {
                stack[i] = stack[j];
            }
        }
    };
}

fn find_var_idx(var_name: &str, variables: &[(Name, Type, usize)]) -> (usize, usize) {
    variables
        .iter()
        .find_map(|(name, var_type, idx)| {
            if *var_name == **name {
                Some((*idx, idx + var_type.size()))
            } else {
                None
            }
        })
        .unwrap()
}

fn load_reg(
    value: &Value,
    reg_name: &RegisterName,
    variables: &[(Name, Type, usize)],
    stack: &mut [u8],
    register: &mut Register,
) {
    let bytes = match value {
        Value::RValue(i) => i.as_bytes(reg_name.size()),
        Value::LValue(i) => {
            let (start, end) = find_var_idx(i, variables);
            stack[start..end].to_vec()
        }
    };

    match reg_name {
        RegisterName::A0 => register.a[0] = bytes[0],
        RegisterName::A1 => register.a[1] = bytes[0],
        RegisterName::A2 => register.a[2] = bytes[0],
        RegisterName::A3 => register.a[3] = bytes[0],

        RegisterName::B0 => register.b[0] = u16::from_ne_bytes(bytes.try_into().unwrap()),
        RegisterName::B1 => register.b[1] = u16::from_ne_bytes(bytes.try_into().unwrap()),
        RegisterName::B2 => register.b[2] = u16::from_ne_bytes(bytes.try_into().unwrap()),
        RegisterName::B3 => register.b[3] = u16::from_ne_bytes(bytes.try_into().unwrap()),

        RegisterName::C0 => register.c[0] = u32::from_ne_bytes(bytes.try_into().unwrap()),
        RegisterName::C1 => register.c[1] = u32::from_ne_bytes(bytes.try_into().unwrap()),
        RegisterName::C2 => register.c[2] = u32::from_ne_bytes(bytes.try_into().unwrap()),
        RegisterName::C3 => register.c[3] = u32::from_ne_bytes(bytes.try_into().unwrap()),

        RegisterName::D0 => register.d[0] = u64::from_ne_bytes(bytes.try_into().unwrap()),
        RegisterName::D1 => register.d[1] = u64::from_ne_bytes(bytes.try_into().unwrap()),
        RegisterName::D2 => register.d[2] = u64::from_ne_bytes(bytes.try_into().unwrap()),
        RegisterName::D3 => register.d[3] = u64::from_ne_bytes(bytes.try_into().unwrap()),

        RegisterName::F0 => register.f[0] = f32::from_ne_bytes(bytes.try_into().unwrap()),
        RegisterName::F1 => register.f[1] = f32::from_ne_bytes(bytes.try_into().unwrap()),
        RegisterName::F2 => register.f[2] = f32::from_ne_bytes(bytes.try_into().unwrap()),
        RegisterName::F3 => register.f[3] = f32::from_ne_bytes(bytes.try_into().unwrap()),

        RegisterName::G0 => register.g[0] = f64::from_ne_bytes(bytes.try_into().unwrap()),
        RegisterName::G1 => register.g[1] = f64::from_ne_bytes(bytes.try_into().unwrap()),
        RegisterName::G2 => register.g[2] = f64::from_ne_bytes(bytes.try_into().unwrap()),
        RegisterName::G3 => register.g[3] = f64::from_ne_bytes(bytes.try_into().unwrap()),

        RegisterName::I0 => register.i[0] = bytes[0] != 0,
        RegisterName::I1 => register.i[1] = bytes[0] != 0,
        RegisterName::I2 => register.i[2] = bytes[0] != 0,
        RegisterName::I3 => register.i[2] = bytes[0] != 0,
    }
}

fn unload_reg(
    reg: &RegisterName,
    var_name: &str,
    variables: &[(Name, Type, usize)],
    stack: &mut [u8],
    register: &mut Register,
) -> Result<(), Str> {
    let (start, end) = find_var_idx(var_name, variables);

    if end - start != reg.size() {
        return Err("Not of same size".into());
    }

    match reg {
        RegisterName::A0 => {
            stack[start] = register.a[0];
            register.a[0] = 0;
        }
        RegisterName::A1 => {
            stack[start] = register.a[1];
            register.a[0] = 0;
        }
        RegisterName::A2 => {
            stack[start] = register.a[2];
            register.a[0] = 0;
        }
        RegisterName::A3 => {
            stack[start] = register.a[3];
            register.a[0] = 0;
        }

        RegisterName::B0 => {
            stack[start..end].copy_from_slice(&register.b[0].to_ne_bytes());
            register.b[0] = 0;
        }
        RegisterName::B1 => {
            stack[start..end].copy_from_slice(&register.b[1].to_ne_bytes());
            register.b[1] = 0;
        }
        RegisterName::B2 => {
            stack[start..end].copy_from_slice(&register.b[2].to_ne_bytes());
            register.b[2] = 0;
        }
        RegisterName::B3 => {
            stack[start..end].copy_from_slice(&register.b[3].to_ne_bytes());
            register.b[3] = 0;
        }

        RegisterName::C0 => {
            stack[start..end].copy_from_slice(&register.c[0].to_ne_bytes());
            register.c[0] = 0;
        }
        RegisterName::C1 => {
            stack[start..end].copy_from_slice(&register.c[1].to_ne_bytes());
            register.c[1] = 0;
        }
        RegisterName::C2 => {
            stack[start..end].copy_from_slice(&register.c[2].to_ne_bytes());
            register.c[2] = 0;
        }
        RegisterName::C3 => {
            stack[start..end].copy_from_slice(&register.c[3].to_ne_bytes());
            register.c[3] = 0;
        }

        RegisterName::D0 => {
            stack[start..end].copy_from_slice(&register.d[0].to_ne_bytes());
            register.d[0] = 0;
        }
        RegisterName::D1 => {
            stack[start..end].copy_from_slice(&register.d[1].to_ne_bytes());
            register.d[1] = 0;
        }
        RegisterName::D2 => {
            stack[start..end].copy_from_slice(&register.d[2].to_ne_bytes());
            register.d[2] = 0;
        }
        RegisterName::D3 => {
            stack[start..end].copy_from_slice(&register.d[3].to_ne_bytes());
            register.d[3] = 0;
        }

        RegisterName::F0 => {
            stack[start..end].copy_from_slice(&register.f[0].to_ne_bytes());
            register.f[0] = 0.0;
        }
        RegisterName::F1 => {
            stack[start..end].copy_from_slice(&register.f[1].to_ne_bytes());
            register.f[1] = 0.0;
        }
        RegisterName::F2 => {
            stack[start..end].copy_from_slice(&register.f[2].to_ne_bytes());
            register.f[2] = 0.0;
        }
        RegisterName::F3 => {
            stack[start..end].copy_from_slice(&register.f[3].to_ne_bytes());
            register.f[3] = 0.0;
        }

        RegisterName::G0 => {
            stack[start..end].copy_from_slice(&register.g[0].to_ne_bytes());
            register.g[0] = 0.0;
        }
        RegisterName::G1 => {
            stack[start..end].copy_from_slice(&register.g[1].to_ne_bytes());
            register.g[1] = 0.0;
        }
        RegisterName::G2 => {
            stack[start..end].copy_from_slice(&register.g[2].to_ne_bytes());
            register.g[2] = 0.0;
        }
        RegisterName::G3 => {
            stack[start..end].copy_from_slice(&register.g[3].to_ne_bytes());
            register.g[3] = 0.0;
        }

        RegisterName::I0 => {
            stack[start] = register.i[0] as u8;
            register.i[0] = false;
        }
        RegisterName::I1 => {
            stack[start] = register.i[1] as u8;
            register.i[1] = false;
        }
        RegisterName::I2 => {
            stack[start] = register.i[2] as u8;
            register.i[2] = false;
        }
        RegisterName::I3 => {
            stack[start] = register.i[3] as u8;
            register.i[3] = false;
        }
    };

    Ok(())
}
