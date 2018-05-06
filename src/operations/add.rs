use cpu::CPU;
use instructions::Instruction;
use definition::Operand;
use errors::*;
use constants::*;
use operations::Execute;

pub struct Add;

impl Execute for Add {
    fn execute(instruction: &Instruction, cpu: &mut CPU) -> Result<()> {
        let dst = instruction.get_operand(0)?;
        let src = instruction.get_operand(1)?;

        match (dst, src) {
            (&Operand::Register(r1), &Operand::Register(r2)) => {
                let op1 = cpu.reg[r1];
                let op2 = cpu.reg[r2];
                let val = op1.wrapping_add(op2);
                cpu.reg[r1] = val;

                cpu.flag_cond(FLAG_Z, val == 0);
                cpu.clear_flag(FLAG_N);
                cpu.set_half_carry(op1 as usize, op2 as usize);
                cpu.set_carry(op1 as usize, op2 as usize);
            },
            (&Operand::Register(r), &Operand::RegisterPairAddr(h, l)) => {
                let op1 = cpu.reg[r];
                let addr = cpu.read_reg_addr(h, l);
                let op2 = cpu.ram.load(addr);
                let val = op1.wrapping_add(op2);
                cpu.reg[r] = val;

                cpu.flag_cond(FLAG_Z, val == 0);
                cpu.clear_flag(FLAG_N);
                cpu.set_half_carry(op1 as usize, op2 as usize);
                cpu.set_carry(op1 as usize, op2 as usize);
            },
            (&Operand::Register(r), &Operand::Immediate(BYTE)) => {
                let op1 = cpu.reg[r];
                let op2 = instruction.get_immediate_u8()?;
                let val = op1.wrapping_add(op2);
                cpu.reg[r] = val;

                cpu.flag_cond(FLAG_Z, val == 0);
                cpu.clear_flag(FLAG_N);
                cpu.set_half_carry(op1 as usize, op2 as usize);
                cpu.set_carry(op1 as usize, op2 as usize);
            },
            (&Operand::RegisterPair(h1, l1), &Operand::RegisterPair(h2, l2)) => {
                let op1 = cpu.read_reg_short(h1, l1);
                let op2 = cpu.read_reg_short(h2, l2);
                cpu.store_reg_short(h1, l1, op1.wrapping_add(op2));

                cpu.clear_flag(FLAG_N);

                // 16 bit carry
                cpu.set_half_carry((op1 >> 8) as usize, (op2 >> 8) as usize);
                cpu.set_carry((op1 >> 8) as usize, (op2 >> 8) as usize);
            },
            (&Operand::RegisterPair(h, l), &Operand::SP) => {
                let op1 = cpu.read_reg_short(h, l);
                let op2 = cpu.sp;
                cpu.store_reg_short(h, l, op1.wrapping_add(op2));

                cpu.clear_flag(FLAG_N);
                cpu.set_half_carry((op1 >> 8) as usize, (op2 >> 8) as usize);
                cpu.set_carry((op1 >> 8) as usize, (op2 >> 8) as usize);
            },
            (&Operand::SP, &Operand::Offset(BYTE)) => {
                let op1 = cpu.sp;
                let op2 = instruction.get_immediate_i8()?;

                // TODO: Check this. Danger danger
                cpu.sp = (op1 as i32 + op2 as i32) as u16;
                cpu.clear_flag(FLAG_Z);
                cpu.clear_flag(FLAG_N);
            },
            _ => {
                // TODO: Add error here
                println!("UNEXPECTED OPERANDS {} {}", dst, src);
            },
        };
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
    fn execute_adds() {
        execute_all(Mnemonic::ADD);
    }

    #[test]
    fn test_add_reg_to_a() {
        let reg_codes: [(u16, usize); 7] = [
            (0x87, REG_A),
            (0x80, REG_B),
            (0x81, REG_C),
            (0x82, REG_D),
            (0x83, REG_E),
            (0x84, REG_H),
            (0x85, REG_L),
        ];

        for &(c, r) in reg_codes.iter() {
            let mut mem = Memory::default();
            let mut cpu = CPU::new(mem);
            cpu.reg[REG_A] = 0x7a;
            if r != REG_A {
                cpu.reg[r] = 0x11;
            }
            execute_instruction(&mut cpu, c, None);
            if r != REG_A {
                assert_eq!(cpu.reg[REG_A], 0x8b);
            } else {
                assert_eq!(cpu.reg[REG_A], 0xf4);
            }
        }
    }

    #[test]
    fn test_add_reg_to_a_half_carry() {
        let mem = Memory::default();
        let mut cpu = CPU::new(mem);
        cpu.reg[REG_A] = 0x08;
        cpu.reg[REG_B] = 0x09;
        execute_instruction(&mut cpu, 0x80, None);
        assert_eq!(cpu.reg[REG_A], 0x11);
        assert_eq!(cpu.flag, 0b0010_0000);
    }

    #[test]
    fn test_add_reg_to_a_carry() {
        let mem = Memory::default();
        let mut cpu = CPU::new(mem);
        cpu.reg[REG_A] = 0x80;
        cpu.reg[REG_B] = 0x81;
        execute_instruction(&mut cpu, 0x80, None);
        assert_eq!(cpu.reg[REG_A], 0x01);
        assert_eq!(cpu.flag, 0b0001_0000);
    }

    #[test]
    fn test_add_reg_to_a_zero() {
        let mem = Memory::default();
        let mut cpu = CPU::new(mem);
        cpu.reg[REG_A] = 0x00;
        cpu.reg[REG_B] = 0x00;
        execute_instruction(&mut cpu, 0x80, None);
        assert_eq!(cpu.reg[REG_A], 0x00);
        assert_eq!(cpu.flag, 0b1000_0000);
    }

    #[test]
    fn test_add_immediate_to_a() {
        let mem = Memory::default();
        let mut cpu = CPU::new(mem);
        cpu.reg[REG_A] = 0x9a;
        execute_instruction(&mut cpu, 0xc6, Some(0x11));
        assert_eq!(cpu.reg[REG_A], 0xab);
    }

    #[test]
    fn test_add_regpair_addr_to_a() {
        let mut mem = Memory::default();
        mem.store(0xff22, 0x11);
        let mut cpu = CPU::new(mem);
        cpu.reg[REG_A] = 0x9a;
        cpu.reg[REG_H] = 0xff;
        cpu.reg[REG_L] = 0x22;
        execute_instruction(&mut cpu, 0x86, None);
        assert_eq!(cpu.reg[REG_A], 0xab);
    }

    #[test]
    fn test_add_regpair_to_hl() {
        let pairs: [(u16, usize, usize); 3] = [
            (0x09, REG_B, REG_C),
            (0x19, REG_D, REG_E),
            (0x29, REG_H, REG_L),
        ];

        for &(c, h, l) in pairs.iter() {
            let mut mem = Memory::default();
            let mut cpu = CPU::new(mem);
            cpu.reg[REG_H] = 0x01;
            cpu.reg[REG_L] = 0x02;
            if h != REG_H {
                cpu.reg[h] = 0x03;
                cpu.reg[l] = 0x04;
            }
            execute_instruction(&mut cpu, c, None);
            if h != REG_H {
                assert_eq!(cpu.reg[REG_H], 0x04);
                assert_eq!(cpu.reg[REG_L], 0x06);
            } else {
                assert_eq!(cpu.reg[REG_H], 0x02);
                assert_eq!(cpu.reg[REG_L], 0x04);
            }
        }
    }

    #[test]
    fn test_add_regpair_to_hl_half_carry() {
        let mem = Memory::default();
        let mut cpu = CPU::new(mem);
        cpu.reg[REG_H] = 0x08;
        cpu.reg[REG_L] = 0x02;
        cpu.reg[REG_B] = 0x09;
        cpu.reg[REG_C] = 0x04;
        execute_instruction(&mut cpu, 0x09, None);
        assert_eq!(cpu.reg[REG_H], 0x11);
        assert_eq!(cpu.reg[REG_L], 0x06);
        assert_eq!(cpu.flag, 0b0010_0000);
    }

    #[test]
    fn test_add_regpair_to_hl_carry() {
        let mem = Memory::default();
        let mut cpu = CPU::new(mem);
        cpu.reg[REG_H] = 0x80;
        cpu.reg[REG_L] = 0x02;
        cpu.reg[REG_B] = 0x81;
        cpu.reg[REG_C] = 0x04;
        execute_instruction(&mut cpu, 0x09, None);
        assert_eq!(cpu.reg[REG_H], 0x01);
        assert_eq!(cpu.reg[REG_L], 0x06);
        assert_eq!(cpu.flag, 0b0001_0000);
    }

    #[test]
    fn test_add_sp_to_hl() {
        let mem = Memory::default();
        let mut cpu = CPU::new(mem);
        cpu.reg[REG_H] = 0x01;
        cpu.reg[REG_L] = 0x02;
        cpu.sp = 0x03;
        execute_instruction(&mut cpu, 0x39, None);
        assert_eq!(cpu.reg[REG_H], 0x01);
        assert_eq!(cpu.reg[REG_L], 0x05);
    }
}
