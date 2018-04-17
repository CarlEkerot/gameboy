use cpu::CPU;
use instructions::{Instruction, Operand};
use errors::*;
use operations::Execute;

pub struct Pop;

impl Execute for Pop {
    fn execute(instruction: &Instruction, cpu: &mut CPU) -> Result<()> {
        let src = instruction.get_operand(0)?;

        match *src {
            Operand::RegisterPair(h, l) => {
                cpu.reg[h] = cpu.stack_pop();
                cpu.reg[l] = cpu.stack_pop();
            },
            _ => {
                println!("UNEXPECTED OPERAND {}", src);
            }
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use test_helpers::execute_all;
    use instructions::Mnemonic;

    #[test]
    fn execute_pop() {
        execute_all(Mnemonic::POP);
    }
}
