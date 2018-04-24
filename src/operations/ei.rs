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
    use test_helpers::{execute_all, execute_instruction};
    use definition::Mnemonic;
    use cpu::{CPU, CPUState};
    use memory::Memory;
    use constants::*;

    #[test]
    fn execute_ei() {
        execute_all(Mnemonic::EI);
    }

    #[test]
    fn test_enable_interrupts() {
        let mut mem = Memory::default();
        let mut cpu = CPU::new(mem);
        cpu.interrupts = false;
        execute_instruction(&mut cpu, 0xfb, None);
        assert_eq!(cpu.interrupts, true)
    }
}
