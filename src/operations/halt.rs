use cpu::{CPU, CPUState};
use instructions::Instruction;
use errors::*;
use operations::Execute;

pub struct Halt;

impl Execute for Halt {
    fn execute(_instruction: &Instruction, cpu: &mut CPU) -> Result<()> {
        cpu.set_state(CPUState::Halted);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use test_helpers::{execute_all, execute_instruction};
    use definition::Mnemonic;
    use cpu::{CPU, CPUState};
    use memory::Memory;

    #[test]
    fn execute_halt() {
        execute_all(Mnemonic::HALT);
    }

    #[test]
    fn test_halt() {
        let mem = Memory::default();
        let mut cpu = CPU::new(mem);
        execute_instruction(&mut cpu, 0x76, None);
        assert_eq!(cpu.state, CPUState::Halted)
    }
}
