use cpu::CPU;
use instructions::Instruction;
use errors::*;
use operations::Execute;

pub struct EnableInterrupts;

impl Execute for EnableInterrupts {
    fn execute(_instruction: &Instruction, cpu: &mut CPU) -> Result<()> {
        cpu.enable_interrupts();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use test_helpers::{execute_all, execute_instruction, test_cpu};
    use definition::Mnemonic;

    #[test]
    fn execute_ei() {
        execute_all(Mnemonic::EI);
    }

    #[test]
    fn test_enable_interrupts() {
        let mut cpu = test_cpu();
        cpu.disable_interrupts();
        execute_instruction(&mut cpu, 0xfb, None);
        let mem = cpu.mem.borrow();
        assert_eq!(mem.interrupts, true);
    }
}
