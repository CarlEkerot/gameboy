use cpu::CPU;
use instructions::Instruction;
use errors::*;
use constants::*;
use operations::Execute;

pub struct ComplementCarryFlag;

impl Execute for ComplementCarryFlag {
    fn execute(_instruction: &Instruction, cpu: &mut CPU) -> Result<()> {
        let is_carry = cpu.flag_is_set(FLAG_C);
        cpu.clear_flag(FLAG_N);
        cpu.clear_flag(FLAG_H);
        cpu.flag_cond(FLAG_C, !is_carry);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use test_helpers::{execute_all, execute_instruction};
    use definition::Mnemonic;
    use cpu::CPU;
    use memory::Memory;
    use constants::*;

    #[test]
    fn execute_ccf() {
        execute_all(Mnemonic::CCF);
    }

    #[test]
    fn test_ccf_no_carry() {
        let mem = Memory::default();
        let mut cpu = CPU::new(mem);
        execute_instruction(&mut cpu, 0x3f, None);
        assert_eq!(cpu.flag, 0b0001_0000);
    }

    #[test]
    fn test_ccf_carry() {
        let mem = Memory::default();
        let mut cpu = CPU::new(mem);
        cpu.set_flag(FLAG_C);
        execute_instruction(&mut cpu, 0x3f, None);
        assert_eq!(cpu.flag, 0b0000_0000);
    }
}
