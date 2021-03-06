use cpu::CPU;
use instructions::Instruction;
use definition::Operand;
use errors::*;
use constants::*;
use operations::Execute;

pub struct AddCarry;

impl Execute for AddCarry {
    fn execute(instruction: &Instruction, cpu: &mut CPU) -> Result<()> {
        let src = instruction.get_operand(1)?;

        match *src {
            Operand::Register(r) => {
                let a = cpu.reg[REG_A];
                let b = cpu.reg[r];

                let mut val = a.wrapping_add(b);

                if cpu.flag_is_set(FLAG_C) {
                    val = val.wrapping_add(1);
                }

                cpu.reg[REG_A] = val;

                cpu.flag_cond(FLAG_Z, val == 0);
                cpu.clear_flag(FLAG_N);
                cpu.set_half_carry(a as usize, b as usize);
                cpu.set_carry(a as usize, b as usize);
            },
            Operand::RegisterPairAddr(h, l) => {
                let addr = cpu.read_reg_addr(h, l);
                let a = cpu.reg[REG_A];
                let b = cpu.load_mem(addr);

                let mut val = a.wrapping_add(b);

                if cpu.flag_is_set(FLAG_C) {
                    val = val.wrapping_add(1);
                }

                cpu.reg[REG_A] = val;

                cpu.flag_cond(FLAG_Z, val == 0);
                cpu.clear_flag(FLAG_N);
                cpu.set_half_carry(a as usize, b as usize);
                cpu.set_carry(a as usize, b as usize);
            },
            Operand::Immediate(BYTE) => {
                let a = cpu.reg[REG_A];
                let b = instruction.get_immediate_u8()?;
                let mut val = a.wrapping_add(b);

                if cpu.flag_is_set(FLAG_C) {
                    val = val.wrapping_add(1);
                }

                cpu.reg[REG_A] = val;

                cpu.flag_cond(FLAG_Z, val == 0);
                cpu.clear_flag(FLAG_N);
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
    use test_helpers::{execute_all, execute_instruction, test_cpu};
    use definition::Mnemonic;
    use constants::*;

    #[test]
    fn execute_adcs() {
        execute_all(Mnemonic::ADC);
    }

    #[test]
    fn test_adc_reg_to_a() {
        let reg_codes: [(u16, usize, u8); 14] = [
            (0x8f, REG_A, 1),
            (0x88, REG_B, 1),
            (0x89, REG_C, 1),
            (0x8a, REG_D, 1),
            (0x8b, REG_E, 1),
            (0x8c, REG_H, 1),
            (0x8d, REG_L, 1),
            (0x8f, REG_A, 0),
            (0x88, REG_B, 0),
            (0x89, REG_C, 0),
            (0x8a, REG_D, 0),
            (0x8b, REG_E, 0),
            (0x8c, REG_H, 0),
            (0x8d, REG_L, 0),
        ];

        for &(c, r, carry) in reg_codes.iter() {
            let mut cpu = test_cpu();
            cpu.reg[REG_A] = 0x7a;
            if r != REG_A {
                cpu.reg[r] = 0x11;
            }
            cpu.flag_cond(FLAG_C, carry == 1);
            execute_instruction(&mut cpu, c, None);
            if r != REG_A {
                assert_eq!(cpu.reg[REG_A], 0x8b + carry);
            } else {
                assert_eq!(cpu.reg[REG_A], 0xf4 + carry);
            }
        }
    }

    #[test]
    fn test_adc_reg_to_a_half_carry() {
        for carry in 0..2 {
            let mut cpu = test_cpu();
            cpu.reg[REG_A] = 0x08;
            cpu.reg[REG_B] = 0x09;
            cpu.flag_cond(FLAG_C, carry == 1);
            execute_instruction(&mut cpu, 0x88, None);
            assert_eq!(cpu.reg[REG_A], 0x11 + carry);
            assert_eq!(cpu.flag, 0b0010_0000);
        }
    }

    #[test]
    fn test_adc_reg_to_a_carry() {
        for carry in 0..2 {
            let mut cpu = test_cpu();
            cpu.reg[REG_A] = 0x80;
            cpu.reg[REG_B] = 0x81;
            cpu.flag_cond(FLAG_C, carry == 1);
            execute_instruction(&mut cpu, 0x88, None);
            assert_eq!(cpu.reg[REG_A], 0x01 + carry);
            assert_eq!(cpu.flag, 0b0001_0000);
        }
    }

    #[test]
    fn test_adc_reg_to_a_zero() {
        let mut cpu = test_cpu();
        cpu.reg[REG_A] = 0x00;
        cpu.reg[REG_B] = 0x00;
        execute_instruction(&mut cpu, 0x88, None);
        assert_eq!(cpu.reg[REG_A], 0x00);
        assert_eq!(cpu.flag, 0b1000_0000);
    }

    #[test]
    fn test_adc_immediate_to_a() {
        for carry in 0..2 {
            let mut cpu = test_cpu();
            cpu.reg[REG_A] = 0x9a;
            cpu.flag_cond(FLAG_C, carry == 1);
            execute_instruction(&mut cpu, 0xce, Some(0x11));
            assert_eq!(cpu.reg[REG_A], 0xab + carry);
        }
    }

    #[test]
    fn test_adc_regpair_addr_to_a() {
        for carry in 0..2 {
            let mut cpu = test_cpu();
            cpu.store_mem(0xff22, 0x11);
            cpu.reg[REG_A] = 0x9a;
            cpu.reg[REG_H] = 0xff;
            cpu.reg[REG_L] = 0x22;
            cpu.flag_cond(FLAG_C, carry == 1);
            execute_instruction(&mut cpu, 0x8e, None);
            assert_eq!(cpu.reg[REG_A], 0xab + carry);
        }
    }
}
