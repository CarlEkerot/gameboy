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
    use test_helpers::execute_all;
    use definition::Mnemonic;

    #[test]
    fn execute_ei() {
        execute_all(Mnemonic::EI);
    }
}
