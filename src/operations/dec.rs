use cpu::CPU;
use instructions::Instruction;
use definition::Operand;
use errors::*;
use constants::*;
use operations::Execute;

pub struct Decrease;

impl Execute for Decrease {
    fn execute(instruction: &Instruction, cpu: &mut CPU) -> Result<()> {
        let dst = instruction.get_operand(0)?;

        match *dst {
            Operand::Register(r) => {
                let val = cpu.reg[r];
                let res = val.wrapping_sub(1);
                cpu.reg[r] = res;
                cpu.set_half_carry(val as usize, 1);
                cpu.flag_cond(FLAG_Z, res == 0);
            },
            Operand::RegisterPair(h, l) => {
                let val = cpu.read_reg_short(h, l);
                let res = val.wrapping_sub(1);
                cpu.store_reg_short(h, l, val);

                // Make sure we calculate carry on high byte
                cpu.set_half_carry((val >> 8) as usize, 1);
                cpu.flag_cond(FLAG_Z, res == 0);
            },
            Operand::SP => {
                let val = cpu.sp;
                let res = val.wrapping_sub(1);
                cpu.sp = res;
                cpu.set_half_carry((val >> 8) as usize, 1);
                cpu.flag_cond(FLAG_Z, res == 0);
            },
            Operand::RegisterPairAddr(h, l) => {
                let addr = cpu.read_reg_addr(h, l);
                let val = cpu.ram.load(addr);
                let res = val.wrapping_sub(1);
                cpu.ram.store(addr, res);
                cpu.set_half_carry(val as usize, 1);
                cpu.flag_cond(FLAG_Z, res == 0);
            },
            _ => {
                println!("UNEXPECTED OPERANDS IN DEC");
            }
        };

        cpu.clear_flag(FLAG_N);
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
    fn execute_decs() {
        execute_all(Mnemonic::DEC);
    }

    #[test]
    fn test_dec_reg() {
        let reg_codes: [(u16, usize); 7] = [
            (0x3d, REG_A),
            (0x05, REG_B),
            (0x0d, REG_C),
            (0x15, REG_D),
            (0x1d, REG_E),
            (0x25, REG_H),
            (0x2d, REG_L),
        ];

        for &(c, r) in reg_codes.iter() {
            let mut mem = Memory::default();
            let mut cpu = CPU::new(mem);
            cpu.reg[r] = 0x11;
            execute_instruction(&mut cpu, c, None);
            assert_eq!(cpu.reg[r], 0x10);
        }
    }

    #[test]
    fn test_dec_overflow() {
        let mem = Memory::default();
        let mut cpu = CPU::new(mem);
        cpu.reg[REG_A] = 0x00;
        execute_instruction(&mut cpu, 0x3d, None);
        assert_eq!(cpu.reg[REG_A], 0xff);
        assert_eq!(cpu.flag, 0b0000_0000);
    }

    #[test]
    fn test_dec_half_carry() {
        let mem = Memory::default();
        let mut cpu = CPU::new(mem);
        cpu.reg[REG_A] = 0x10;
        execute_instruction(&mut cpu, 0x3d, None);
        assert_eq!(cpu.reg[REG_A], 0x0f);
        assert_eq!(cpu.flag, 0b0010_0000);
    }

    #[test]
    fn test_dec_regpair_addr() {
        let mut mem = Memory::default();
        mem.store(0xff22, 0x11);
        let mut cpu = CPU::new(mem);
        cpu.reg[REG_H] = 0xff;
        cpu.reg[REG_L] = 0x22;
        execute_instruction(&mut cpu, 0x35, None);
        assert_eq!(cpu.ram.load(0xff22), 0x10);
    }

    #[test]
    fn test_dec_regpair() {
        let pairs: [(u16, usize, usize); 3] = [
            (0x0b, REG_B, REG_C),
            (0x1b, REG_D, REG_E),
            (0x2b, REG_H, REG_L),
        ];

        for &(c, h, l) in pairs.iter() {
            let mut mem = Memory::default();
            let mut cpu = CPU::new(mem);
            cpu.reg[h] = 0xaa;
            cpu.reg[l] = 0xbb;
            execute_instruction(&mut cpu, c, None);
            assert_eq!(cpu.reg[h], 0xaa);
            assert_eq!(cpu.reg[l], 0xba);
        }
    }

    #[test]
    fn test_dec_sp() {
        let mem = Memory::default();
        let mut cpu = CPU::new(mem);
        cpu.sp = 0xaabb;
        execute_instruction(&mut cpu, 0x3B, None);
        assert_eq!(cpu.sp, 0xaaba);
    }
}
