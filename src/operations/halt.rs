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
    use test_helpers::{execute_all, execute_instruction, test_cpu};
    use definition::Mnemonic;
    use cpu::CPUState;

    #[test]
    fn execute_halt() {
        execute_all(Mnemonic::HALT);
    }

    #[test]
    fn test_halt() {
        let mut cpu = test_cpu();
        execute_instruction(&mut cpu, 0x76, None);
        assert_eq!(cpu.state, CPUState::Halted)
    }
}
