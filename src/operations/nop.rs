use cpu::CPU;
use instructions::Instruction;
use errors::*;
use operations::Execute;

pub struct Nop;

impl Execute for Nop {
    fn execute(_instruction: &Instruction, _cpu: &mut CPU) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use test_helpers::execute_all;
    use definition::Mnemonic;

    #[test]
    fn execute_nop() {
        execute_all(Mnemonic::NOP);
    }
}
