use crate::error::JAPLError;

#[derive(Debug, PartialEq)]
pub enum Literal {
    Boolean(bool),
    Float(f64),
    Integer(u64),
}

impl TryFrom<&str> for Literal {
    type Error = JAPLError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(i) = value.parse::<u64>() {
            return Ok(Self::Integer(i));
        }

        if let Ok(i) = value.parse::<i64>() {
            return Ok(Self::Integer(unsafe { std::mem::transmute::<i64, u64>(i) }));
        }

        if let Ok(i) = value.parse::<f64>() {
            return Ok(Self::Float(i));
        }

        if value == "true" {
            return Ok(Self::Boolean(true));
        }

        if value == "false" {
            return Ok(Self::Boolean(false));
        }

        if value.starts_with('\'') && value.ends_with('\'') {
            todo!("Characters not supported yet");
        }

        if value.starts_with('"') && value.ends_with('"') {
            todo!("Strings not supported yet");
        }

        Err(JAPLError::InvalidIdentifier(value.into()))
    }
}

impl Literal {
    pub fn as_bytes(&self, size: usize) -> Vec<u8> {
        match self {
            Literal::Boolean(i) => {
                if *i {
                    vec![1]
                } else {
                    vec![0]
                }
            }
            Literal::Float(i) => {
                if size == 4 {
                    (*i as f32).to_ne_bytes().to_vec()
                } else {
                    i.to_ne_bytes().to_vec()
                }
            }
            Literal::Integer(i) => match size {
                1 => (*i as u8).to_ne_bytes().to_vec(),
                2 => (*i as u16).to_ne_bytes().to_vec(),
                4 => (*i as u32).to_ne_bytes().to_vec(),
                _ => i.to_ne_bytes().to_vec(),
            },
        }
    }
}
