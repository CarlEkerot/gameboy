use cpu::CPU;
use instructions::Instruction;
use errors::*;
use constants::*;
use operations::Execute;

pub struct RotateALeftCarry;

impl Execute for RotateALeftCarry {
    fn execute(_instruction: &Instruction, cpu: &mut CPU) -> Result<()> {
        let val = cpu.reg[REG_A];
        let msb = val >> 7;
        let res = ((val << 1) & 0xff) | msb;
        cpu.reg[REG_A] = res;

        cpu.flag_cond(FLAG_Z, res == 0);
        cpu.clear_flag(FLAG_N);
        cpu.clear_flag(FLAG_H);
        cpu.flag_cond(FLAG_C, msb == 1);

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
    fn execute_rlca() {
        execute_all(Mnemonic::RLCA);
    }

    #[test]
    fn test_rlca_no_carry() {
        let mut mem = Memory::default();
        let mut cpu = CPU::new(mem);
        cpu.reg[REG_A] = 0b0111_1111;
        execute_instruction(&mut cpu, 0x07, None);
        assert_eq!(cpu.reg[REG_A], 0b1111_1110);
        assert_eq!(cpu.flag, 0b0000_0000);
    }

    #[test]
    fn test_rlca_carry() {
        let mut mem = Memory::default();
        let mut cpu = CPU::new(mem);
        cpu.reg[REG_A] = 0b1111_1111;
        execute_instruction(&mut cpu, 0x07, None);
        assert_eq!(cpu.reg[REG_A], 0b1111_1111);
        assert_eq!(cpu.flag, 0b0001_0000);
    }
}
