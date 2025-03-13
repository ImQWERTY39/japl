use super::{BinOperator, UnOperator};
use crate::lexer::Keyword;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RegisterName {
    A0, // 8 bits
    A1, // int registers
    A2,
    A3,

    B0, // 16 bits
    B1, // int registers
    B2,
    B3,

    C0, // 32 bits
    C1, // int registers
    C2,
    C3,

    D0, // 64 bits
    D1, // int registers
    D2,
    D3,

    F0, // 32 bit float
    F1, // registers
    F2,
    F3,

    G0, // 32 bit
    G1, // float register
    G2,
    G3,

    I0, // boolean
    I1, // registers
    I2,
    I3,
}

impl RegisterName {
    pub fn register_class(&self) -> RegisterClass {
        match self {
            RegisterName::A0 | RegisterName::A1 | RegisterName::A2 | RegisterName::A3 => {
                RegisterClass::A
            }
            RegisterName::B0 | RegisterName::B1 | RegisterName::B2 | RegisterName::B3 => {
                RegisterClass::B
            }
            RegisterName::C0 | RegisterName::C1 | RegisterName::C2 | RegisterName::C3 => {
                RegisterClass::C
            }
            RegisterName::D0 | RegisterName::D1 | RegisterName::D2 | RegisterName::D3 => {
                RegisterClass::D
            }
            RegisterName::F0 | RegisterName::F1 | RegisterName::F2 | RegisterName::F3 => {
                RegisterClass::F
            }
            RegisterName::G0 | RegisterName::G1 | RegisterName::G2 | RegisterName::G3 => {
                RegisterClass::G
            }
            RegisterName::I0 | RegisterName::I1 | RegisterName::I2 | RegisterName::I3 => {
                RegisterClass::I
            }
        }
    }

    pub fn size(&self) -> usize {
        match self {
            RegisterName::A0 | RegisterName::A1 | RegisterName::A2 | RegisterName::A3 => 1,
            RegisterName::B0 | RegisterName::B1 | RegisterName::B2 | RegisterName::B3 => 2,
            RegisterName::C0 | RegisterName::C1 | RegisterName::C2 | RegisterName::C3 => 4,
            RegisterName::D0 | RegisterName::D1 | RegisterName::D2 | RegisterName::D3 => 8,
            RegisterName::F0 | RegisterName::F1 | RegisterName::F2 | RegisterName::F3 => 4,
            RegisterName::G0 | RegisterName::G1 | RegisterName::G2 | RegisterName::G3 => 8,
            RegisterName::I0 | RegisterName::I1 | RegisterName::I2 | RegisterName::I3 => 1,
        }
    }

    pub fn index(&self) -> usize {
        match self {
            RegisterName::A0
            | RegisterName::B0
            | RegisterName::C0
            | RegisterName::D0
            | RegisterName::F0
            | RegisterName::G0
            | RegisterName::I0 => 0,

            RegisterName::A1
            | RegisterName::B1
            | RegisterName::C1
            | RegisterName::D1
            | RegisterName::F1
            | RegisterName::G1
            | RegisterName::I1 => 1,

            RegisterName::A2
            | RegisterName::B2
            | RegisterName::C2
            | RegisterName::D2
            | RegisterName::F2
            | RegisterName::G2
            | RegisterName::I2 => 2,

            RegisterName::A3
            | RegisterName::B3
            | RegisterName::C3
            | RegisterName::D3
            | RegisterName::F3
            | RegisterName::G3
            | RegisterName::I3 => 3,
        }
    }
}

impl TryFrom<Keyword> for RegisterName {
    type Error = ();

    fn try_from(value: Keyword) -> Result<Self, ()> {
        match value {
            Keyword::RegisterA0 => Ok(Self::A0),
            Keyword::RegisterA1 => Ok(Self::A1),
            Keyword::RegisterA2 => Ok(Self::A2),
            Keyword::RegisterA3 => Ok(Self::A3),

            Keyword::RegisterB0 => Ok(Self::B0),
            Keyword::RegisterB1 => Ok(Self::B1),
            Keyword::RegisterB2 => Ok(Self::B2),
            Keyword::RegisterB3 => Ok(Self::B3),

            Keyword::RegisterC0 => Ok(Self::C0),
            Keyword::RegisterC1 => Ok(Self::C1),
            Keyword::RegisterC2 => Ok(Self::C2),
            Keyword::RegisterC3 => Ok(Self::C3),

            Keyword::RegisterD0 => Ok(Self::D0),
            Keyword::RegisterD1 => Ok(Self::D1),
            Keyword::RegisterD2 => Ok(Self::D2),
            Keyword::RegisterD3 => Ok(Self::D3),

            Keyword::RegisterF0 => Ok(Self::F0),
            Keyword::RegisterF1 => Ok(Self::F1),
            Keyword::RegisterF2 => Ok(Self::F2),
            Keyword::RegisterF3 => Ok(Self::F3),

            Keyword::RegisterG0 => Ok(Self::G0),
            Keyword::RegisterG1 => Ok(Self::G1),
            Keyword::RegisterG2 => Ok(Self::G2),
            Keyword::RegisterG3 => Ok(Self::G3),

            Keyword::RegisterI0 => Ok(Self::I0),
            Keyword::RegisterI1 => Ok(Self::I1),
            Keyword::RegisterI2 => Ok(Self::I2),
            Keyword::RegisterI3 => Ok(Self::I3),

            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RegisterClass {
    A,
    B,
    C,
    D,
    F,
    G,
    I,
}

impl TryFrom<Keyword> for RegisterClass {
    type Error = ();

    fn try_from(value: Keyword) -> Result<Self, ()> {
        match value {
            Keyword::RegClassA => Ok(Self::A),
            Keyword::RegClassB => Ok(Self::B),
            Keyword::RegClassC => Ok(Self::C),
            Keyword::RegClassD => Ok(Self::D),
            Keyword::RegClassF => Ok(Self::F),
            Keyword::RegClassG => Ok(Self::G),
            Keyword::RegClassI => Ok(Self::I),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Default)]
pub struct Register {
    pub a: [u8; 4],
    pub b: [u16; 4],
    pub c: [u32; 4],
    pub d: [u64; 4],
    pub f: [f32; 4],
    pub g: [f64; 4],
    pub i: [bool; 4],
}

impl Register {
    pub fn un_operate(
        &mut self,
        op: UnOperator,
        src: RegisterName,
        dst: RegisterName,
    ) -> Result<(), ()> {
        match (op, dst.register_class(), src.register_class()) {
            (UnOperator::Not, RegisterClass::A, RegisterClass::A) => {
                self.a[dst.index()] = !self.a[src.index()];
            }
            (UnOperator::Not, RegisterClass::B, RegisterClass::B) => {
                self.b[dst.index()] = !self.b[src.index()];
            }
            (UnOperator::Not, RegisterClass::C, RegisterClass::C) => {
                self.c[dst.index()] = !self.c[src.index()];
            }
            (UnOperator::Not, RegisterClass::D, RegisterClass::D) => {
                self.d[dst.index()] = !self.d[src.index()];
            }
            (UnOperator::Not, RegisterClass::I, RegisterClass::I) => {
                self.i[dst.index()] = !self.i[src.index()];
            }
            (UnOperator::Increment, RegisterClass::A, RegisterClass::A) => {
                self.a[dst.index()] = self.a[src.index()] + 1;
            }
            (UnOperator::Increment, RegisterClass::B, RegisterClass::B) => {
                self.b[dst.index()] = self.b[src.index()] + 1;
            }
            (UnOperator::Increment, RegisterClass::C, RegisterClass::C) => {
                self.c[dst.index()] = self.c[src.index()] + 1;
            }
            (UnOperator::Increment, RegisterClass::D, RegisterClass::D) => {
                self.d[dst.index()] = self.d[src.index()] + 1;
            }
            (UnOperator::Increment, RegisterClass::F, RegisterClass::F) => {
                self.f[dst.index()] = self.f[src.index()] + 1.0;
            }
            (UnOperator::Increment, RegisterClass::G, RegisterClass::G) => {
                self.g[dst.index()] = self.g[src.index()] + 1.0;
            }
            (UnOperator::Decrement, RegisterClass::A, RegisterClass::A) => {
                self.a[dst.index()] = self.a[src.index()] - 1;
            }
            (UnOperator::Decrement, RegisterClass::B, RegisterClass::B) => {
                self.b[dst.index()] = self.b[src.index()] - 1;
            }
            (UnOperator::Decrement, RegisterClass::C, RegisterClass::C) => {
                self.c[dst.index()] = self.c[src.index()] - 1;
            }
            (UnOperator::Decrement, RegisterClass::D, RegisterClass::D) => {
                self.d[dst.index()] = self.d[src.index()] - 1;
            }
            (UnOperator::Decrement, RegisterClass::F, RegisterClass::F) => {
                self.f[dst.index()] = self.f[src.index()] - 1.0;
            }
            (UnOperator::Decrement, RegisterClass::G, RegisterClass::G) => {
                self.g[dst.index()] = self.g[src.index()] - 1.0;
            }
            _ => return Err(()),
        }

        Ok(())
    }

    pub fn bin_operate(
        &mut self,
        op: BinOperator,
        src1: RegisterName,
        src2: RegisterName,
        dst: RegisterName,
    ) -> Result<(), ()> {
        match (
            op,
            src1.register_class(),
            src2.register_class(),
            dst.register_class(),
        ) {
            (BinOperator::Add, RegisterClass::A, RegisterClass::A, RegisterClass::A) => {
                self.a[dst.index()] = self.a[src1.index()] + self.a[src2.index()]
            }
            (BinOperator::Add, RegisterClass::B, RegisterClass::B, RegisterClass::B) => {
                self.b[dst.index()] = self.b[src1.index()] + self.b[src2.index()]
            }
            (BinOperator::Add, RegisterClass::C, RegisterClass::C, RegisterClass::C) => {
                self.c[dst.index()] = self.c[src1.index()] + self.c[src2.index()]
            }
            (BinOperator::Add, RegisterClass::D, RegisterClass::D, RegisterClass::D) => {
                self.d[dst.index()] = self.d[src1.index()] + self.d[src2.index()]
            }
            (BinOperator::Add, RegisterClass::F, RegisterClass::F, RegisterClass::F) => {
                self.f[dst.index()] = self.f[src1.index()] + self.f[src2.index()]
            }
            (BinOperator::Add, RegisterClass::G, RegisterClass::G, RegisterClass::G) => {
                self.g[dst.index()] = self.g[src1.index()] + self.g[src2.index()]
            }

            (BinOperator::Subtract, RegisterClass::A, RegisterClass::A, RegisterClass::A) => {
                self.a[dst.index()] = self.a[src1.index()] - self.a[src2.index()]
            }
            (BinOperator::Subtract, RegisterClass::B, RegisterClass::B, RegisterClass::B) => {
                self.b[dst.index()] = self.b[src1.index()] - self.b[src2.index()]
            }
            (BinOperator::Subtract, RegisterClass::C, RegisterClass::C, RegisterClass::C) => {
                self.c[dst.index()] = self.c[src1.index()] - self.c[src2.index()]
            }
            (BinOperator::Subtract, RegisterClass::D, RegisterClass::D, RegisterClass::D) => {
                self.d[dst.index()] = self.d[src1.index()] - self.d[src2.index()]
            }
            (BinOperator::Subtract, RegisterClass::F, RegisterClass::F, RegisterClass::F) => {
                self.f[dst.index()] = self.f[src1.index()] - self.f[src2.index()]
            }
            (BinOperator::Subtract, RegisterClass::G, RegisterClass::G, RegisterClass::G) => {
                self.g[dst.index()] = self.g[src1.index()] - self.g[src2.index()]
            }

            (BinOperator::Multiply, RegisterClass::A, RegisterClass::A, RegisterClass::A) => {
                self.a[dst.index()] = self.a[src1.index()] * self.a[src2.index()]
            }
            (BinOperator::Multiply, RegisterClass::B, RegisterClass::B, RegisterClass::B) => {
                self.b[dst.index()] = self.b[src1.index()] * self.b[src2.index()]
            }
            (BinOperator::Multiply, RegisterClass::C, RegisterClass::C, RegisterClass::C) => {
                self.c[dst.index()] = self.c[src1.index()] * self.c[src2.index()]
            }
            (BinOperator::Multiply, RegisterClass::D, RegisterClass::D, RegisterClass::D) => {
                self.d[dst.index()] = self.d[src1.index()] * self.d[src2.index()]
            }
            (BinOperator::Multiply, RegisterClass::F, RegisterClass::F, RegisterClass::F) => {
                self.f[dst.index()] = self.f[src1.index()] * self.f[src2.index()]
            }
            (BinOperator::Multiply, RegisterClass::G, RegisterClass::G, RegisterClass::G) => {
                self.g[dst.index()] = self.g[src1.index()] * self.g[src2.index()]
            }

            (BinOperator::Divide, RegisterClass::A, RegisterClass::A, RegisterClass::A) => {
                self.a[dst.index()] = self.a[src1.index()] / self.a[src2.index()]
            }
            (BinOperator::Divide, RegisterClass::B, RegisterClass::B, RegisterClass::B) => {
                self.b[dst.index()] = self.b[src1.index()] / self.b[src2.index()]
            }
            (BinOperator::Divide, RegisterClass::C, RegisterClass::C, RegisterClass::C) => {
                self.c[dst.index()] = self.c[src1.index()] / self.c[src2.index()]
            }
            (BinOperator::Divide, RegisterClass::D, RegisterClass::D, RegisterClass::D) => {
                self.d[dst.index()] = self.d[src1.index()] / self.d[src2.index()]
            }
            (BinOperator::Divide, RegisterClass::F, RegisterClass::F, RegisterClass::F) => {
                self.f[dst.index()] = self.f[src1.index()] / self.f[src2.index()]
            }
            (BinOperator::Divide, RegisterClass::G, RegisterClass::G, RegisterClass::G) => {
                self.g[dst.index()] = self.g[src1.index()] / self.g[src2.index()]
            }

            (BinOperator::Modulus, RegisterClass::A, RegisterClass::A, RegisterClass::A) => {
                self.a[dst.index()] = self.a[src1.index()] % self.a[src2.index()]
            }
            (BinOperator::Modulus, RegisterClass::B, RegisterClass::B, RegisterClass::B) => {
                self.b[dst.index()] = self.b[src1.index()] % self.b[src2.index()]
            }
            (BinOperator::Modulus, RegisterClass::C, RegisterClass::C, RegisterClass::C) => {
                self.c[dst.index()] = self.c[src1.index()] % self.c[src2.index()]
            }
            (BinOperator::Modulus, RegisterClass::D, RegisterClass::D, RegisterClass::D) => {
                self.d[dst.index()] = self.d[src1.index()] % self.d[src2.index()]
            }

            (BinOperator::LeftShift, RegisterClass::A, RegisterClass::A, RegisterClass::A) => {
                self.a[dst.index()] = self.a[src1.index()] << self.a[src2.index()]
            }
            (BinOperator::LeftShift, RegisterClass::B, RegisterClass::B, RegisterClass::B) => {
                self.b[dst.index()] = self.b[src1.index()] << self.b[src2.index()]
            }
            (BinOperator::LeftShift, RegisterClass::C, RegisterClass::C, RegisterClass::C) => {
                self.c[dst.index()] = self.c[src1.index()] << self.c[src2.index()]
            }
            (BinOperator::LeftShift, RegisterClass::D, RegisterClass::D, RegisterClass::D) => {
                self.d[dst.index()] = self.d[src1.index()] << self.d[src2.index()]
            }

            (BinOperator::RightShift, RegisterClass::A, RegisterClass::A, RegisterClass::A) => {
                self.a[dst.index()] = self.a[src1.index()] >> self.a[src2.index()]
            }
            (BinOperator::RightShift, RegisterClass::B, RegisterClass::B, RegisterClass::B) => {
                self.b[dst.index()] = self.b[src1.index()] >> self.b[src2.index()]
            }
            (BinOperator::RightShift, RegisterClass::C, RegisterClass::C, RegisterClass::C) => {
                self.c[dst.index()] = self.c[src1.index()] >> self.c[src2.index()]
            }
            (BinOperator::RightShift, RegisterClass::D, RegisterClass::D, RegisterClass::D) => {
                self.d[dst.index()] = self.d[src1.index()] >> self.d[src2.index()]
            }

            (BinOperator::And, RegisterClass::A, RegisterClass::A, RegisterClass::A) => {
                self.a[dst.index()] = self.a[src1.index()] & self.a[src2.index()]
            }
            (BinOperator::And, RegisterClass::B, RegisterClass::B, RegisterClass::B) => {
                self.b[dst.index()] = self.b[src1.index()] & self.b[src2.index()]
            }
            (BinOperator::And, RegisterClass::C, RegisterClass::C, RegisterClass::C) => {
                self.c[dst.index()] = self.c[src1.index()] & self.c[src2.index()]
            }
            (BinOperator::And, RegisterClass::D, RegisterClass::D, RegisterClass::D) => {
                self.d[dst.index()] = self.d[src1.index()] & self.d[src2.index()]
            }
            (BinOperator::And, RegisterClass::I, RegisterClass::I, RegisterClass::I) => {
                self.i[dst.index()] = self.i[src1.index()] && self.i[src2.index()]
            }

            (BinOperator::Or, RegisterClass::A, RegisterClass::A, RegisterClass::A) => {
                self.a[dst.index()] = self.a[src1.index()] | self.a[src2.index()]
            }
            (BinOperator::Or, RegisterClass::B, RegisterClass::B, RegisterClass::B) => {
                self.b[dst.index()] = self.b[src1.index()] | self.b[src2.index()]
            }
            (BinOperator::Or, RegisterClass::C, RegisterClass::C, RegisterClass::C) => {
                self.c[dst.index()] = self.c[src1.index()] | self.c[src2.index()]
            }
            (BinOperator::Or, RegisterClass::D, RegisterClass::D, RegisterClass::D) => {
                self.d[dst.index()] = self.d[src1.index()] | self.d[src2.index()]
            }
            (BinOperator::Or, RegisterClass::I, RegisterClass::I, RegisterClass::I) => {
                self.i[dst.index()] = self.i[src1.index()] || self.i[src2.index()]
            }

            (BinOperator::Xor, RegisterClass::A, RegisterClass::A, RegisterClass::A) => {
                self.a[dst.index()] = self.a[src1.index()] ^ self.a[src2.index()]
            }
            (BinOperator::Xor, RegisterClass::B, RegisterClass::B, RegisterClass::B) => {
                self.a[dst.index()] = self.a[src1.index()] ^ self.a[src2.index()]
            }
            (BinOperator::Xor, RegisterClass::C, RegisterClass::C, RegisterClass::C) => {
                self.a[dst.index()] = self.a[src1.index()] ^ self.a[src2.index()]
            }
            (BinOperator::Xor, RegisterClass::D, RegisterClass::D, RegisterClass::D) => {
                self.a[dst.index()] = self.a[src1.index()] ^ self.a[src2.index()]
            }
            (BinOperator::Xor, RegisterClass::I, RegisterClass::I, RegisterClass::I) => {
                self.a[dst.index()] = self.a[src1.index()] ^ self.a[src2.index()]
            }

            (BinOperator::Equals, RegisterClass::A, RegisterClass::A, RegisterClass::I) => {
                self.i[dst.index()] = self.a[src1.index()] == self.a[src2.index()]
            }
            (BinOperator::Equals, RegisterClass::B, RegisterClass::B, RegisterClass::I) => {
                self.i[dst.index()] = self.b[src1.index()] == self.b[src2.index()]
            }
            (BinOperator::Equals, RegisterClass::C, RegisterClass::C, RegisterClass::I) => {
                self.i[dst.index()] = self.c[src1.index()] == self.c[src2.index()]
            }
            (BinOperator::Equals, RegisterClass::D, RegisterClass::D, RegisterClass::I) => {
                self.i[dst.index()] = self.d[src1.index()] == self.d[src2.index()]
            }
            (BinOperator::Equals, RegisterClass::F, RegisterClass::F, RegisterClass::I) => {
                self.i[dst.index()] = self.f[src1.index()] == self.f[src2.index()]
            }
            (BinOperator::Equals, RegisterClass::G, RegisterClass::G, RegisterClass::I) => {
                self.i[dst.index()] = self.g[src1.index()] == self.g[src2.index()]
            }
            (BinOperator::Equals, RegisterClass::I, RegisterClass::I, RegisterClass::I) => {
                self.i[dst.index()] = self.i[src1.index()] == self.i[src2.index()]
            }

            (BinOperator::NotEquals, RegisterClass::A, RegisterClass::A, RegisterClass::I) => {
                self.i[dst.index()] = self.a[src1.index()] != self.a[src2.index()]
            }
            (BinOperator::NotEquals, RegisterClass::B, RegisterClass::B, RegisterClass::I) => {
                self.i[dst.index()] = self.b[src1.index()] != self.b[src2.index()]
            }
            (BinOperator::NotEquals, RegisterClass::C, RegisterClass::C, RegisterClass::I) => {
                self.i[dst.index()] = self.c[src1.index()] != self.c[src2.index()]
            }
            (BinOperator::NotEquals, RegisterClass::D, RegisterClass::D, RegisterClass::I) => {
                self.i[dst.index()] = self.d[src1.index()] != self.d[src2.index()]
            }
            (BinOperator::NotEquals, RegisterClass::F, RegisterClass::F, RegisterClass::I) => {
                self.i[dst.index()] = self.f[src1.index()] != self.f[src2.index()]
            }
            (BinOperator::NotEquals, RegisterClass::G, RegisterClass::G, RegisterClass::I) => {
                self.i[dst.index()] = self.g[src1.index()] != self.g[src2.index()]
            }
            (BinOperator::NotEquals, RegisterClass::I, RegisterClass::I, RegisterClass::I) => {
                self.i[dst.index()] = self.i[src1.index()] != self.i[src2.index()]
            }

            (BinOperator::LessThan, RegisterClass::A, RegisterClass::A, RegisterClass::I) => {
                self.i[dst.index()] = self.a[src1.index()] < self.a[src2.index()]
            }
            (BinOperator::LessThan, RegisterClass::B, RegisterClass::B, RegisterClass::I) => {
                self.i[dst.index()] = self.b[src1.index()] < self.b[src2.index()]
            }
            (BinOperator::LessThan, RegisterClass::C, RegisterClass::C, RegisterClass::I) => {
                self.i[dst.index()] = self.c[src1.index()] < self.c[src2.index()]
            }
            (BinOperator::LessThan, RegisterClass::D, RegisterClass::D, RegisterClass::I) => {
                self.i[dst.index()] = self.d[src1.index()] < self.d[src2.index()]
            }
            (BinOperator::LessThan, RegisterClass::F, RegisterClass::F, RegisterClass::I) => {
                self.i[dst.index()] = self.f[src1.index()] < self.f[src2.index()]
            }
            (BinOperator::LessThan, RegisterClass::G, RegisterClass::G, RegisterClass::I) => {
                self.i[dst.index()] = self.g[src1.index()] < self.g[src2.index()]
            }
            (BinOperator::LessThan, RegisterClass::I, RegisterClass::I, RegisterClass::I) => {
                self.i[dst.index()] = self.i[src1.index()] < self.i[src2.index()]
            }

            (BinOperator::GreaterThan, RegisterClass::A, RegisterClass::A, RegisterClass::I) => {
                self.i[dst.index()] = self.a[src1.index()] > self.a[src2.index()]
            }
            (BinOperator::GreaterThan, RegisterClass::B, RegisterClass::B, RegisterClass::I) => {
                self.i[dst.index()] = self.b[src1.index()] > self.b[src2.index()]
            }
            (BinOperator::GreaterThan, RegisterClass::C, RegisterClass::C, RegisterClass::I) => {
                self.i[dst.index()] = self.c[src1.index()] > self.c[src2.index()]
            }
            (BinOperator::GreaterThan, RegisterClass::D, RegisterClass::D, RegisterClass::I) => {
                self.i[dst.index()] = self.d[src1.index()] > self.d[src2.index()]
            }
            (BinOperator::GreaterThan, RegisterClass::F, RegisterClass::F, RegisterClass::I) => {
                self.i[dst.index()] = self.f[src1.index()] > self.f[src2.index()]
            }
            (BinOperator::GreaterThan, RegisterClass::G, RegisterClass::G, RegisterClass::I) => {
                self.i[dst.index()] = self.g[src1.index()] > self.g[src2.index()]
            }
            (BinOperator::GreaterThan, RegisterClass::I, RegisterClass::I, RegisterClass::I) => {
                self.i[dst.index()] = self.i[src1.index()] > self.i[src2.index()]
            }

            (
                BinOperator::LessThanEqualTo,
                RegisterClass::A,
                RegisterClass::A,
                RegisterClass::I,
            ) => self.i[dst.index()] = self.a[src1.index()] <= self.a[src2.index()],
            (
                BinOperator::LessThanEqualTo,
                RegisterClass::B,
                RegisterClass::B,
                RegisterClass::I,
            ) => self.i[dst.index()] = self.b[src1.index()] <= self.b[src2.index()],
            (
                BinOperator::LessThanEqualTo,
                RegisterClass::C,
                RegisterClass::C,
                RegisterClass::I,
            ) => self.i[dst.index()] = self.c[src1.index()] <= self.c[src2.index()],
            (
                BinOperator::LessThanEqualTo,
                RegisterClass::D,
                RegisterClass::D,
                RegisterClass::I,
            ) => self.i[dst.index()] = self.d[src1.index()] <= self.d[src2.index()],
            (
                BinOperator::LessThanEqualTo,
                RegisterClass::F,
                RegisterClass::F,
                RegisterClass::I,
            ) => self.i[dst.index()] = self.f[src1.index()] <= self.f[src2.index()],
            (
                BinOperator::LessThanEqualTo,
                RegisterClass::G,
                RegisterClass::G,
                RegisterClass::I,
            ) => self.i[dst.index()] = self.g[src1.index()] <= self.g[src2.index()],
            (
                BinOperator::LessThanEqualTo,
                RegisterClass::I,
                RegisterClass::I,
                RegisterClass::I,
            ) => self.i[dst.index()] = self.i[src1.index()] <= self.i[src2.index()],

            (
                BinOperator::GreaterThanEqualTo,
                RegisterClass::A,
                RegisterClass::A,
                RegisterClass::I,
            ) => self.i[dst.index()] = self.a[src1.index()] >= self.a[src2.index()],
            (
                BinOperator::GreaterThanEqualTo,
                RegisterClass::B,
                RegisterClass::B,
                RegisterClass::I,
            ) => self.i[dst.index()] = self.b[src1.index()] >= self.b[src2.index()],
            (
                BinOperator::GreaterThanEqualTo,
                RegisterClass::C,
                RegisterClass::C,
                RegisterClass::I,
            ) => self.i[dst.index()] = self.c[src1.index()] >= self.c[src2.index()],
            (
                BinOperator::GreaterThanEqualTo,
                RegisterClass::D,
                RegisterClass::D,
                RegisterClass::I,
            ) => self.i[dst.index()] = self.d[src1.index()] >= self.d[src2.index()],
            (
                BinOperator::GreaterThanEqualTo,
                RegisterClass::F,
                RegisterClass::F,
                RegisterClass::I,
            ) => self.i[dst.index()] = self.f[src1.index()] >= self.f[src2.index()],
            (
                BinOperator::GreaterThanEqualTo,
                RegisterClass::G,
                RegisterClass::G,
                RegisterClass::I,
            ) => self.i[dst.index()] = self.g[src1.index()] >= self.g[src2.index()],
            (
                BinOperator::GreaterThanEqualTo,
                RegisterClass::I,
                RegisterClass::I,
                RegisterClass::I,
            ) => self.i[dst.index()] = self.i[src1.index()] >= self.i[src2.index()],

            _ => return Err(()),
        };

        Ok(())
    }
}
