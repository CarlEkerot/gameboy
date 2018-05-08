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
    use test_helpers::{execute_all, execute_instruction, test_cpu};
    use definition::Mnemonic;
    use constants::*;

    #[test]
    fn execute_scf() {
        execute_all(Mnemonic::SCF);
    }

    #[test]
    fn test_scf_no_carry() {
        let mut cpu = test_cpu();
        execute_instruction(&mut cpu, 0x37, None);
        assert_eq!(cpu.flag, 0b0001_0000);
    }

    #[test]
    fn test_scf_carry() {
        let mut cpu = test_cpu();
        cpu.set_flag(FLAG_C);
        execute_instruction(&mut cpu, 0x37, None);
        assert_eq!(cpu.flag, 0b0001_0000);
    }
}
