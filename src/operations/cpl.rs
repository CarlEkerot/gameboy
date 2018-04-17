use cpu::CPU;
use instructions::Instruction;
use errors::*;
use constants::*;
use operations::Execute;

pub struct ComplementA;

impl Execute for ComplementA {
    fn execute(_instruction: &Instruction, cpu: &mut CPU) -> Result<()> {
        cpu.reg[REG_A] = !cpu.reg[REG_A];
        cpu.set_flag(FLAG_N);
        cpu.set_flag(FLAG_H);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use test_helpers::execute_all;
    use instructions::Mnemonic;

    #[test]
    fn execute_cpl() {
        execute_all(Mnemonic::CPL);
    }
}
