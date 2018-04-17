use cpu::CPU;
use instructions::{Instruction, Operand};
use errors::*;
use operations::Execute;

pub struct Restart;

impl Execute for Restart {
    fn execute(instruction: &Instruction, cpu: &mut CPU) -> Result<()> {
        let offset = instruction.get_operand(0)?;

        if let Operand::RSTOffset(o) = *offset {
            // TODO: Push present address onto stack?!
            let addr = cpu.pc;
            cpu.stack_push((addr & 0xf) as u8);
            cpu.stack_push((addr >> 8) as u8);
            cpu.pc = o as u16;
        } else {
            println!("UNEXPECTED OPERAND {}", offset);
        }

        // TODO: Accommodate for next inc of program counter?

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use test_helpers::execute_all;
    use instructions::Mnemonic;

    #[test]
    fn execute_rst() {
        execute_all(Mnemonic::RST);
    }
}
