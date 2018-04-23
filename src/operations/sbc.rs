use cpu::CPU;
use instructions::Instruction;
use definition::Operand;
use errors::*;
use constants::*;
use operations::Execute;

pub struct SubtractCarry;

impl Execute for SubtractCarry {
    fn execute(instruction: &Instruction, cpu: &mut CPU) -> Result<()> {
        let src = instruction.get_operand(1)?;

        match *src {
            Operand::Register(r) => {
                let a = cpu.reg[REG_A];
                let b = cpu.reg[r];

                // TODO: CHECK THIS
                let mut val = a.wrapping_sub(b);

                if cpu.flag_is_set(FLAG_C) {
                    val = val.wrapping_sub(1);
                }

                cpu.reg[REG_A] = val;

                cpu.flag_cond(FLAG_Z, val == 0);
                cpu.set_flag(FLAG_N);
                cpu.set_half_carry(a as usize, b as usize);
                cpu.set_carry(a as usize, b as usize);
            },
            Operand::RegisterPairAddr(h, l) => {
                let addr = cpu.read_reg_addr(h, l);
                let a = cpu.reg[REG_A];
                let b = cpu.ram.load(addr);

                let mut val = a.wrapping_sub(b);

                if cpu.flag_is_set(FLAG_C) {
                    val = val.wrapping_sub(1);
                }

                cpu.reg[REG_A] = val;

                cpu.flag_cond(FLAG_Z, val == 0);
                cpu.set_flag(FLAG_N);
                cpu.set_half_carry(a as usize, b as usize);
                cpu.set_carry(a as usize, b as usize);
            },
            Operand::Immediate(BYTE) => {
                let a = cpu.reg[REG_A];
                let b = instruction.get_immediate_u8()?;
                let mut val = a.wrapping_sub(b);

                if cpu.flag_is_set(FLAG_C) {
                    val = val.wrapping_sub(1);
                }

                cpu.reg[REG_A] = val;

                cpu.flag_cond(FLAG_Z, val == 0);
                cpu.set_flag(FLAG_N);
                cpu.set_half_carry(a as usize, b as usize);
                cpu.set_carry(a as usize, b as usize);

            },
            _ => {
                // TODO: Add error here
                println!("UNEXPECTED OPERANDS {}", src);
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
    fn execute_sbcs() {
        execute_all(Mnemonic::SBC);
    }

    #[test]
    fn test_sub_reg_from_a() {
        let reg_codes: [(u16, usize, u8); 14] = [
            (0x9f, REG_A, 1),
            (0x98, REG_B, 1),
            (0x99, REG_C, 1),
            (0x9a, REG_D, 1),
            (0x9b, REG_E, 1),
            (0x9c, REG_H, 1),
            (0x9d, REG_L, 1),
            (0x9f, REG_A, 0),
            (0x98, REG_B, 0),
            (0x99, REG_C, 0),
            (0x9a, REG_D, 0),
            (0x9b, REG_E, 0),
            (0x9c, REG_H, 0),
            (0x9d, REG_L, 0),
        ];

        for &(c, r, carry) in reg_codes.iter() {
            let mut mem = Memory::default();
            let mut cpu = CPU::new(mem);
            cpu.flag_cond(FLAG_C, carry == 1);
            cpu.reg[REG_A] = 0x7a;
            if r != REG_A {
                cpu.reg[r] = 0x11;
            }
            execute_instruction(&mut cpu, c, None);
            if r != REG_A {
                assert_eq!(cpu.reg[REG_A], 0x69u8.wrapping_sub(carry));
            } else {
                assert_eq!(cpu.reg[REG_A], 0x00u8.wrapping_sub(carry));
            }
        }
    }

    #[test]
    fn test_sbc_reg_from_a_half_carry() {
        for carry in 0..2 {
            let mut mem = Memory::default();
            let mut cpu = CPU::new(mem);
            cpu.reg[REG_A] = 0x7f;
            cpu.reg[REG_B] = 0x01;
            cpu.flag_cond(FLAG_C, carry == 1);
            execute_instruction(&mut cpu, 0x98, None);
            assert_eq!(cpu.reg[REG_A], 0x7e - carry);
            assert_eq!(cpu.flag, 0b0110_0000);
        }
    }

    #[test]
    fn test_sbc_reg_from_a_carry() {
        for carry in 0..2 {
            let mut mem = Memory::default();
            let mut cpu = CPU::new(mem);
            cpu.reg[REG_A] = 0x80;
            cpu.reg[REG_B] = 0x81;
            cpu.flag_cond(FLAG_C, carry == 1);
            execute_instruction(&mut cpu, 0x98, None);
            assert_eq!(cpu.reg[REG_A], 0xff - carry);
            assert_eq!(cpu.flag, 0b0101_0000);
        }
    }

    #[test]
    fn test_sbc_reg_from_a_zero() {
        let mut mem = Memory::default();
        let mut cpu = CPU::new(mem);
        cpu.reg[REG_A] = 0x01;
        cpu.reg[REG_B] = 0x00;
        cpu.set_flag(FLAG_C);
        execute_instruction(&mut cpu, 0x98, None);
        assert_eq!(cpu.reg[REG_A], 0x00);
        assert_eq!(cpu.flag, 0b1100_0000);
    }

    #[test]
    fn test_sbc_immediate_from_a() {
        for carry in 0..2 {
            let mut mem = Memory::default();
            let mut cpu = CPU::new(mem);
            cpu.reg[REG_A] = 0x9a;
            cpu.flag_cond(FLAG_C, carry == 1);
            execute_instruction(&mut cpu, 0xde, Some(0x11));
            assert_eq!(cpu.reg[REG_A], 0x89 - carry);
        }
    }

    #[test]
    fn test_sbc_regpair_addr_from_a() {
        for carry in 0..2 {
            let mut mem = Memory::default();
            mem.store(0xff22, 0x11);
            let mut cpu = CPU::new(mem);
            cpu.reg[REG_A] = 0x9a;
            cpu.reg[REG_H] = 0xff;
            cpu.reg[REG_L] = 0x22;
            cpu.flag_cond(FLAG_C, carry == 1);
            execute_instruction(&mut cpu, 0x9e, None);
            assert_eq!(cpu.reg[REG_A], 0x89 - carry);
        }
    }
}
