use cpu::CPU;
use instructions::Instruction;
use definition::Operand;
use errors::*;
use constants::*;
use operations::Execute;

pub struct JumpRelative;

impl Execute for JumpRelative {
    fn execute(instruction: &Instruction, cpu: &mut CPU) -> Result<()> {
        let op1 = instruction.get_operand(0)?;
        let op2 = instruction.get_operand(1)?;

        let should_jump = match (op1, op2) {
            (&Operand::Offset(BYTE), &Operand::None) => true,
            (&Operand::Zero, &Operand::Offset(BYTE)) => cpu.flag_is_set(FLAG_Z),
            (&Operand::NonZero, &Operand::Offset(BYTE)) => !cpu.flag_is_set(FLAG_Z),
            (&Operand::Carry, &Operand::Offset(BYTE)) => cpu.flag_is_set(FLAG_C),
            (&Operand::NonCarry, &Operand::Offset(BYTE)) => !cpu.flag_is_set(FLAG_C),
            _ => {
                println!("UNEXPECTED OPERAND {} {}", op1, op2);
                false
            }
        };

        if should_jump {
            let offset = instruction.get_immediate_i8()?;
            cpu.pc = ((cpu.pc as i32) + offset as i32) as u16;
        }

        // TODO: Accommodate for next inc of program counter?

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use test_helpers::execute_all;
    use definition::Mnemonic;

    #[test]
    fn execute_jr() {
        execute_all(Mnemonic::JR);
    }
}
