use cpu::CPU;
use instructions::Instruction;
use definition::Operand;
use errors::*;
use constants::*;
use operations::Execute;

pub struct Return;

impl Execute for Return {
    fn execute(instruction: &Instruction, cpu: &mut CPU) -> Result<()> {
        let op = instruction.get_operand(0)?;

        let should_return = match *op {
            Operand::None => true,
            Operand::Zero => cpu.flag_is_set(FLAG_Z),
            Operand::NonZero => !cpu.flag_is_set(FLAG_Z),
            Operand::Carry => cpu.flag_is_set(FLAG_C),
            Operand::NonCarry => !cpu.flag_is_set(FLAG_C),
            _ => {
                println!("UNEXPECTED OPERAND {}", op);
                false
            }
        };

        if should_return {
            let hi = cpu.stack_pop() as u16;
            let lo = cpu.stack_pop() as u16;
            cpu.pc = (hi << 8) | lo;
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
    fn execute_ret() {
        execute_all(Mnemonic::RET);
    }
}
