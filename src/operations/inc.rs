use cpu::CPU;
use instructions::Instruction;
use definition::Operand;
use errors::*;
use constants::*;
use operations::Execute;

pub struct Increase;

impl Execute for Increase {
    fn execute(instruction: &Instruction, cpu: &mut CPU) -> Result<()> {
        let dst = instruction.get_operand(0)?;

        match *dst {
            Operand::Register(r) => {
                let val = cpu.reg[r];
                let res = val.wrapping_add(1);
                cpu.reg[r] = res;
                cpu.set_half_carry(val as usize, 1);
                cpu.flag_cond(FLAG_Z, res == 0);
            },
            Operand::RegisterPair(h, l) => {
                let val = cpu.read_reg_short(h, l);
                let res = val.wrapping_add(1);
                cpu.store_reg_short(h, l, res);

                // Make sure we calculate carry on high byte
                cpu.set_half_carry((val >> 8) as usize, 1);
                cpu.flag_cond(FLAG_Z, res == 0);
            },
            Operand::SP => {
                let val = cpu.sp;
                let res = val.wrapping_add(1);
                cpu.sp = res;
                cpu.set_half_carry((val >> 8) as usize, 1);
                cpu.flag_cond(FLAG_Z, res == 0);
            },
            Operand::RegisterPairAddr(h, l) => {
                let addr = cpu.read_reg_addr(h, l);
                let val = cpu.load_mem(addr);
                let res = val.wrapping_add(1);
                cpu.store_mem(addr, res);
                cpu.set_half_carry(val as usize, 1);
                cpu.flag_cond(FLAG_Z, res == 0);
            },
            _ => {
                println!("UNEXPECTED OPERANDS IN INC");
            }
        };

        cpu.clear_flag(FLAG_N);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use test_helpers::{execute_all, execute_instruction, test_cpu};
    use definition::Mnemonic;
    use constants::*;

    #[test]
    fn execute_incs() {
        execute_all(Mnemonic::INC);
    }

    #[test]
    fn test_inc_reg() {
        let reg_codes: [(u16, usize); 7] = [
            (0x3c, REG_A),
            (0x04, REG_B),
            (0x0c, REG_C),
            (0x14, REG_D),
            (0x1c, REG_E),
            (0x24, REG_H),
            (0x2c, REG_L),
        ];

        for &(c, r) in reg_codes.iter() {
            let mut cpu = test_cpu();
            cpu.reg[r] = 0x11;
            execute_instruction(&mut cpu, c, None);
            assert_eq!(cpu.reg[r], 0x12);
        }
    }

    #[test]
    fn test_inc_overflow() {
        let mut cpu = test_cpu();
        cpu.reg[REG_A] = 0xff;
        execute_instruction(&mut cpu, 0x3c, None);
        assert_eq!(cpu.reg[REG_A], 0x00);
        assert_eq!(cpu.flag, 0b1010_0000);
    }

    #[test]
    fn test_inc_half_carry() {
        let mut cpu = test_cpu();
        cpu.reg[REG_A] = 0x0f;
        execute_instruction(&mut cpu, 0x3c, None);
        assert_eq!(cpu.reg[REG_A], 0x10);
        assert_eq!(cpu.flag, 0b0010_0000);
    }

    #[test]
    fn test_inc_regpair_addr() {
        let mut cpu = test_cpu();
        cpu.store_mem(0xff22, 0x11);
        cpu.reg[REG_H] = 0xff;
        cpu.reg[REG_L] = 0x22;
        execute_instruction(&mut cpu, 0x34, None);
        assert_eq!(cpu.load_mem(0xff22), 0x12);
    }

    #[test]
    fn test_inc_regpair() {
        let pairs: [(u16, usize, usize); 3] = [
            (0x03, REG_B, REG_C),
            (0x13, REG_D, REG_E),
            (0x23, REG_H, REG_L),
        ];

        for &(c, h, l) in pairs.iter() {
            let mut cpu = test_cpu();
            cpu.reg[h] = 0xaa;
            cpu.reg[l] = 0xbb;
            execute_instruction(&mut cpu, c, None);
            assert_eq!(cpu.reg[h], 0xaa);
            assert_eq!(cpu.reg[l], 0xbc);
        }
    }

    #[test]
    fn test_inc_sp() {
        let mut cpu = test_cpu();
        cpu.sp = 0xaabb;
        execute_instruction(&mut cpu, 0x33, None);
        assert_eq!(cpu.sp, 0xaabc);
    }
}
