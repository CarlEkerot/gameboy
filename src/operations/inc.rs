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

        let val = match *dst {
            Operand::Register(r) => {
                let v = cpu.reg[r];
                cpu.reg[r] = v.wrapping_add(1);
                v as usize
            },
            Operand::RegisterPair(h, l) => {
                let v = cpu.read_reg_short(h, l);
                cpu.store_reg_short(h, l, v.wrapping_add(1));

                // Make sure we calculate carry on high byte
                (v >> 8) as usize
            },
            Operand::SP => {
                let v = cpu.sp;
                cpu.sp = v.wrapping_add(1);
                (v >> 8) as usize
            },
            Operand::RegisterPairAddr(h, l) => {
                let addr = cpu.read_reg_addr(h, l);
                let v = cpu.ram.load(addr);
                cpu.ram.store(addr, v.wrapping_add(1));
                v as usize
            },
            _ => {
                println!("UNEXPECTED OPERANDS IN INC");
                0
            }
        };
        cpu.set_half_carry(val, 1);
        cpu.clear_flag(FLAG_N);
        cpu.flag_cond(FLAG_Z, val == 0);
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
            let mut mem = Memory::default();
            let mut cpu = CPU::new(mem);
            cpu.reg[r] = 0x11;
            execute_instruction(&mut cpu, c, None);
            assert_eq!(cpu.reg[r], 0x12);
        }
    }

    #[test]
    fn test_inc_overflow() {
        let mut mem = Memory::default();
        let mut cpu = CPU::new(mem);
        cpu.reg[REG_A] = 0xff;
        execute_instruction(&mut cpu, 0x3c, None);
        assert_eq!(cpu.reg[REG_A], 0x00);
        assert_eq!(cpu.flag, 0b1000_0000);
    }

    #[test]
    fn test_inc_half_carry() {
        let mut mem = Memory::default();
        let mut cpu = CPU::new(mem);
        cpu.reg[REG_A] = 0x0f;
        execute_instruction(&mut cpu, 0x3c, None);
        assert_eq!(cpu.reg[REG_A], 0x10);
        assert_eq!(cpu.flag, 0b0010_0000);
    }

    #[test]
    fn test_inc_regpair_addr() {
        let mut mem = Memory::default();
        mem.store(0xff22, 0x11);
        let mut cpu = CPU::new(mem);
        cpu.reg[REG_H] = 0xff;
        cpu.reg[REG_L] = 0x22;
        execute_instruction(&mut cpu, 0x34, None);
        assert_eq!(cpu.ram.load(0xff22), 0x12);
    }

    #[test]
    fn test_inc_regpair() {
        let pairs: [(u16, usize, usize); 3] = [
            (0x03, REG_B, REG_C),
            (0x13, REG_D, REG_E),
            (0x23, REG_H, REG_L),
        ];

        for &(c, h, l) in pairs.iter() {
            let mut mem = Memory::default();
            let mut cpu = CPU::new(mem);
            cpu.reg[h] = 0xaa;
            cpu.reg[l] = 0xbb;
            execute_instruction(&mut cpu, c, None);
            assert_eq!(cpu.reg[h], 0xaa);
            assert_eq!(cpu.reg[l], 0xbc);
        }
    }

    #[test]
    fn test_inc_sp() {
        let mut mem = Memory::default();
        let mut cpu = CPU::new(mem);
        cpu.sp = 0xaabb;
        execute_instruction(&mut cpu, 0x33, None);
        assert_eq!(cpu.sp, 0xaabc);
    }
}
