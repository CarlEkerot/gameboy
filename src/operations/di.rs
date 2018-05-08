use cpu::CPU;
use instructions::Instruction;
use errors::*;
use operations::Execute;

pub struct DisableInterrupts;

impl Execute for DisableInterrupts {
    fn execute(_instruction: &Instruction, cpu: &mut CPU) -> Result<()> {
        cpu.disable_interrupts();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use test_helpers::{execute_all, execute_instruction, test_cpu};
    use definition::Mnemonic;

    #[test]
    fn execute_di() {
        execute_all(Mnemonic::DI);
    }

    /*
    #[test]
    fn test_disable_interrupts() {
        let mut cpu = test_cpu();
        execute_instruction(&mut cpu, 0xf3, None);
        assert_eq!(cpu.mem.borrow().interrupts, false)
    }
    */
}
