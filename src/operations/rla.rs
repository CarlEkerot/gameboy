use cpu::CPU;
use instructions::Instruction;
use errors::*;
use constants::*;
use operations::Execute;

pub struct RotateALeft;

impl Execute for RotateALeft {
    fn execute(_instruction: &Instruction, cpu: &mut CPU) -> Result<()> {
        let val = cpu.reg[REG_A];
        let msb = val >> 7;
        let mut res = val << 1;

        if cpu.flag_is_set(FLAG_C) {
            res |= 1;
        }
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
    use test_helpers::{execute_all, execute_instruction, test_cpu};
    use definition::Mnemonic;
    use constants::*;

    #[test]
    fn execute_rla() {
        execute_all(Mnemonic::RLA);
    }

    #[test]
    fn test_rla_no_carry() {
        let mut cpu = test_cpu();
        cpu.reg[REG_A] = 0b0111_1111;
        execute_instruction(&mut cpu, 0x17, None);
        assert_eq!(cpu.reg[REG_A], 0b1111_1110);
        assert_eq!(cpu.flag, 0b0000_0000);
    }

    #[test]
    fn test_rla_carry() {
        let mut cpu = test_cpu();
        cpu.reg[REG_A] = 0b1111_1111;
        execute_instruction(&mut cpu, 0x17, None);
        assert_eq!(cpu.reg[REG_A], 0b1111_1110);
        assert_eq!(cpu.flag, 0b0001_0000);
    }
}
