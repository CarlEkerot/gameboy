use std::fmt;
use definition::{Definition, Operand};
use errors::*;

#[derive(Debug, Clone)]
pub struct Instruction {
    pub definition: &'static Definition,
    pub immediate: Option<u16>,
}

impl Instruction {
    pub fn get_immediate_usize(&self) -> Result<usize> {
        self.immediate.map(|i| i as usize).chain_err(|| "Missing immediate")
    }

    pub fn get_immediate_u8(&self) -> Result<u8> {
        self.immediate.map(|i| i as u8).chain_err(|| "Missing immediate")
    }

    pub fn get_immediate_u16(&self) -> Result<u16> {
        self.immediate.chain_err(|| "Missing immediate")
    }

    pub fn get_immediate_i8(&self) -> Result<i8> {
        self.immediate.map(|i| i as i8).chain_err(|| "Missing immediate")
    }

    pub fn get_immediate_i16(&self) -> Result<i16> {
        self.immediate.map(|i| i as i16).chain_err(|| "Missing immediate")
    }

    pub fn get_operand(&self, index: usize) -> Result<&Operand> {
        self.definition.operands.get(index).chain_err(|| "Missing operand")
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a = self.definition.operands.get(0).unwrap_or(&Operand::None);
        let b = self.definition.operands.get(1).unwrap_or(&Operand::None);
        let a_str = a.as_string(self.immediate);
        let b_str = b.as_string(self.immediate);

        let s = match (a, b) {
            (&Operand::None, &Operand::None) =>
                format!("{:?}", self.definition.mnemonic),
            (_, &Operand::None) =>
                format!("{:?} {}", self.definition.mnemonic, a_str),
            (&Operand::None, _) =>
                format!("{:?} {}", self.definition.mnemonic, b_str),
            _ => {
                format!("{:?} {}, {}", self.definition.mnemonic, a_str, b_str)
            },
        };
        write!(f, "{}", s)
    }
}
