use cpu::CPU;
use instructions::{Instruction, Operand};
use errors::*;
use constants::*;
use operations::Execute;

pub struct Jump;

impl Execute for Jump {
    fn execute(instruction: &Instruction, cpu: &mut CPU) -> Result<()> {
        let op1 = instruction.get_operand(0)?;
        let op2 = instruction.get_operand(1)?;

        match (op1, op2) {
            (&Operand::Address(SHORT), &Operand::None) => {
                cpu.pc = instruction.get_immediate_u16()?;
            },
            (&Operand::RegisterPairAddr(h, l), &Operand::None) => {
                let a = cpu.read_reg_addr(h, l);
                cpu.pc = a as u16;
            },
            (&Operand::Zero, &Operand::Address(SHORT))  => {
                if cpu.flag_is_set(FLAG_Z) {
                    cpu.pc = instruction.get_immediate_u16()?;
                }
            },
            (&Operand::NonZero, &Operand::Address(SHORT)) => {
                if !cpu.flag_is_set(FLAG_Z) {
                    cpu.pc = instruction.get_immediate_u16()?;
                }
            },
            (&Operand::Carry, &Operand::Address(SHORT)) => {
                if cpu.flag_is_set(FLAG_C) {
                    cpu.pc = instruction.get_immediate_u16()?;
                }
            },
            (&Operand::NonCarry, &Operand::Address(SHORT)) => {
                if !cpu.flag_is_set(FLAG_C) {
                    cpu.pc = instruction.get_immediate_u16()?;
                }
            },
            _ => {
                println!("UNEXPECTED OPERAND {} {}", op1, op2);
            }
        };

        // TODO: Accommodate for next inc of program counter?

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use test_helpers::execute_all;
    use instructions::Mnemonic;

    #[test]
    fn execute_jp() {
        execute_all(Mnemonic::JP);
    }
}
