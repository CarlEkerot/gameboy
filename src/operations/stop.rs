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
    use test_helpers::{execute_all, execute_instruction, test_cpu};
    use definition::Mnemonic;
    use cpu::CPUState;

    #[test]
    fn execute_stop() {
        execute_all(Mnemonic::STOP);
    }

    #[test]
    fn test_stop() {
        let mut cpu = test_cpu();
        execute_instruction(&mut cpu, 0x10, None);
        assert_eq!(cpu.state, CPUState::Stopped)
    }
}
