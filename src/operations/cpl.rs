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
    use test_helpers::{execute_all, execute_instruction, test_cpu};
    use definition::Mnemonic;
    use constants::*;

    #[test]
    fn execute_cpl() {
        execute_all(Mnemonic::CPL);
    }

    #[test]
    fn test_cpl() {
        let mut cpu = test_cpu();
        cpu.reg[REG_A] = 0b0101_0101;
        execute_instruction(&mut cpu, 0x2f, None);
        assert_eq!(cpu.reg[REG_A], 0b1010_1010);
        assert_eq!(cpu.flag, 0b0110_0000);
    }
}
