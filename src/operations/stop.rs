use cpu::{CPU, CPUState};
use instructions::Instruction;
use errors::*;
use operations::Execute;

pub struct Stop;

impl Execute for Stop {
    fn execute(_instruction: &Instruction, cpu: &mut CPU) -> Result<()> {
        cpu.set_state(CPUState::Stopped);
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
    fn execute_stop() {
        execute_all(Mnemonic::STOP);
    }

    #[test]
    fn test_stop() {
        let mem = Memory::default();
        let mut cpu = CPU::new(mem);
        execute_instruction(&mut cpu, 0x10, None);
        assert_eq!(cpu.state, CPUState::Stopped)
    }
}
