use cpu::CPU;
use instructions::Instruction;
use errors::*;
use constants::*;
use operations::Execute;

pub struct SetCarryFlag;

impl Execute for SetCarryFlag {
    fn execute(_instruction: &Instruction, cpu: &mut CPU) -> Result<()> {
        cpu.clear_flag(FLAG_N);
        cpu.clear_flag(FLAG_H);
        cpu.set_flag(FLAG_C);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use test_helpers::execute_all;
    use instructions::Mnemonic;

    #[test]
    fn execute_scf() {
        execute_all(Mnemonic::SCF);
    }
}
