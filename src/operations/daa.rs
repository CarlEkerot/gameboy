use cpu::CPU;
use instructions::Instruction;
use errors::*;
use constants::*;
use operations::Execute;

pub struct DecimalAdjustA;

impl Execute for DecimalAdjustA {
    fn execute(_instruction: &Instruction, cpu: &mut CPU) -> Result<()> {
        let lo = cpu.reg[REG_A] & 0xf;
        let hi = cpu.reg[REG_A] >> 4;

        // if the least significant four bits of A contain a non-BCD digit (i. e.
        // it is greater than 9) or the H flag is set: $06 is added to the register.
        if lo > 9 || cpu.flag_is_set(FLAG_H) {
            cpu.reg[REG_A] = cpu.reg[REG_A].wrapping_add(0x6);
        }


        // If the more significant digit also happens to be greater than 9 or the
        // C flag is set: $60 is added
        if (lo > 9 && hi > 9) || cpu.flag_is_set(FLAG_C) {
            cpu.reg[REG_A] = cpu.reg[REG_A].wrapping_add(0x60);
            cpu.set_flag(FLAG_C);
        } else {
            cpu.clear_flag(FLAG_C);
        }

        let res = cpu.reg[REG_A];
        cpu.flag_cond(FLAG_Z, res == 0);
        cpu.clear_flag(FLAG_H);

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
    fn execute_daa() {
        execute_all(Mnemonic::DAA);
    }

    #[test]
    fn test_daa_adjusted() {
        let mem = Memory::default();
        let mut cpu = CPU::new(mem);
        cpu.reg[REG_A] = 0b1010_0111;
        execute_instruction(&mut cpu, 0x27, None);
        assert_eq!(cpu.reg[REG_A], 0b1010_0111);
        assert_eq!(cpu.flag, 0b0000_0000);
    }

    #[test]
    fn test_daa_low() {
        let mem = Memory::default();
        let mut cpu = CPU::new(mem);
        cpu.reg[REG_A] = 0b0000_1111;
        execute_instruction(&mut cpu, 0x27, None);
        assert_eq!(cpu.reg[REG_A], 0b0001_0101);
        assert_eq!(cpu.flag, 0b0000_0000);
    }

    #[test]
    fn test_daa_high() {
        let mem = Memory::default();
        let mut cpu = CPU::new(mem);
        cpu.reg[REG_A] = 0b1010_1010;
        execute_instruction(&mut cpu, 0x27, None);
        assert_eq!(cpu.reg[REG_A], 0b0001_0000);
    }

}
