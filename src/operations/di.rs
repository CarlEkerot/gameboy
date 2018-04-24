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
    use test_helpers::{execute_all, execute_instruction};
    use definition::Mnemonic;
    use cpu::{CPU, CPUState};
    use memory::Memory;
    use constants::*;

    #[test]
    fn execute_di() {
        execute_all(Mnemonic::DI);
    }

    #[test]
    fn test_disable_interrupts() {
        let mut mem = Memory::default();
        let mut cpu = CPU::new(mem);
        execute_instruction(&mut cpu, 0xf3, None);
        assert_eq!(cpu.interrupts, false)
    }
}
