use cpu::CPU;
use instructions::Instruction;
use errors::*;
use operations::Execute;

pub struct ReturnEnableInterrupts;

impl Execute for ReturnEnableInterrupts {
    fn execute(_instruction: &Instruction, cpu: &mut CPU) -> Result<()> {
        let hi = cpu.stack_pop() as u16;
        let lo = cpu.stack_pop() as u16;
        cpu.pc = (hi << 8) | lo;
        cpu.enable_interrupts();

        // TODO: Accommodate for next inc of program counter?

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use test_helpers::execute_all;
    use definition::Mnemonic;

    #[test]
    fn execute_reti() {
        execute_all(Mnemonic::RETI);
    }
}
