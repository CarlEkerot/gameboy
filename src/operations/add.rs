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
                let a = cpu.reg[r1];
                let b = cpu.reg[r2];
                let val = a.wrapping_add(b);
                cpu.reg[r1] = val;

                cpu.flag_cond(FLAG_Z, val == 0);
                cpu.clear_flag(FLAG_N);
                cpu.set_half_carry(a as usize, b as usize);
                cpu.set_carry(a as usize, b as usize);
            },
            (&Operand::Register(r), &Operand::RegisterPairAddr(h, l)) => {
                let a = cpu.reg[r];
                let addr = cpu.read_reg_addr(h, l);
                let b = cpu.ram.load(addr);
                let val = a.wrapping_add(b);
                cpu.reg[r] = val;

                cpu.flag_cond(FLAG_Z, val == 0);
                cpu.clear_flag(FLAG_N);
                cpu.set_half_carry(a as usize, b as usize);
                cpu.set_carry(a as usize, b as usize);
            },
            (&Operand::Register(r), &Operand::Immediate(BYTE)) => {
                let a = cpu.reg[r];
                let b = instruction.get_immediate_u8()?;
                let val = a.wrapping_add(b);
                cpu.reg[r] = val;

                cpu.flag_cond(FLAG_Z, val == 0);
                cpu.clear_flag(FLAG_N);
                cpu.set_half_carry(a as usize, b as usize);
                cpu.set_carry(a as usize, b as usize);
            },
            (&Operand::RegisterPair(h1, l1), &Operand::RegisterPair(h2, l2)) => {
                let a = cpu.read_reg_short(h1, l1);
                let b = cpu.read_reg_short(h2, l2);
                cpu.store_reg_short(h1, l1, a.wrapping_add(b));

                cpu.clear_flag(FLAG_N);
                cpu.set_half_carry(a as usize, b as usize);
                cpu.set_carry(a as usize, b as usize);
            },
            (&Operand::RegisterPair(h, l), &Operand::SP) => {
                let a = cpu.read_reg_short(h, l);
                let b = cpu.sp;
                cpu.store_reg_short(h, l, a.wrapping_add(b));

                cpu.clear_flag(FLAG_N);
                cpu.set_half_carry(a as usize, b as usize);
                cpu.set_carry(a as usize, b as usize);
            },
            (&Operand::SP, &Operand::Offset(BYTE)) => {
                let a = cpu.sp;
                let b = instruction.get_immediate_i8()?;

                // TODO: Check this. Danger danger
                cpu.sp = (a as i32 + b as i32) as u16;
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
        let mut mem = Memory::default();
        let mut cpu = CPU::new(mem);
        cpu.reg[REG_A] = 0x08;
        cpu.reg[REG_B] = 0x09;
        execute_instruction(&mut cpu, 0x80, None);
        assert_eq!(cpu.reg[REG_A], 0x11);
        assert_eq!(cpu.flag, 0b0010_0000);
    }

    #[test]
    fn test_add_reg_to_a_carry() {
        let mut mem = Memory::default();
        let mut cpu = CPU::new(mem);
        cpu.reg[REG_A] = 0x80;
        cpu.reg[REG_B] = 0x81;
        execute_instruction(&mut cpu, 0x80, None);
        assert_eq!(cpu.reg[REG_A], 0x01);
        assert_eq!(cpu.flag, 0b0001_0000);
    }

    #[test]
    fn test_add_reg_to_a_zero() {
        let mut mem = Memory::default();
        let mut cpu = CPU::new(mem);
        cpu.reg[REG_A] = 0x00;
        cpu.reg[REG_B] = 0x00;
        execute_instruction(&mut cpu, 0x80, None);
        assert_eq!(cpu.reg[REG_A], 0x00);
        assert_eq!(cpu.flag, 0b1000_0000);
    }

    #[test]
    fn test_add_immediate_to_a() {
        let mut mem = Memory::default();
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
}
