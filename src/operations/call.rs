use cpu::CPU;
use instructions::Instruction;
use definition::Operand;
use errors::*;
use constants::*;
use operations::Execute;

pub struct Call;

impl Execute for Call {
    fn execute(instruction: &Instruction, cpu: &mut CPU) -> Result<()> {
        let op1 = instruction.get_operand(0)?;
        let op2 = instruction.get_operand(1)?;

        let should_call = match (op1, op2) {
            (&Operand::Address(SHORT), &Operand::None) => true,
            (&Operand::Zero, &Operand::Address(SHORT)) => cpu.flag_is_set(FLAG_Z),
            (&Operand::NonZero, &Operand::Address(SHORT)) => !cpu.flag_is_set(FLAG_Z),
            (&Operand::Carry, &Operand::Address(SHORT)) => cpu.flag_is_set(FLAG_C),
            (&Operand::NonCarry, &Operand::Address(SHORT)) => !cpu.flag_is_set(FLAG_C),
            _ => {
                println!("UNEXPECTED OPERAND {} {}", op1, op2);
                false
            }
        };

        if should_call {
            let next_addr = cpu.pc + 1;
            cpu.stack_push((next_addr & 0xf) as u8);
            cpu.stack_push((next_addr >> 8) as u8);
            cpu.pc = instruction.get_immediate_u16()?;
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
    fn execute_call() {
        execute_all(Mnemonic::CALL);
    }
}
