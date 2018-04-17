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
    use test_helpers::execute_all;
    use instructions::Mnemonic;

    #[test]
    fn execute_stop() {
        execute_all(Mnemonic::STOP);
    }
}
