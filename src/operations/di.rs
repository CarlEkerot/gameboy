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
    use test_helpers::execute_all;
    use definition::Mnemonic;

    #[test]
    fn execute_di() {
        execute_all(Mnemonic::DI);
    }
}
