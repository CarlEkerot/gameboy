use cpu::CPU;
use instructions::Instruction;
use definition::Operand;
use errors::*;
use constants::*;
use operations::Execute;

pub struct Xor;

impl Execute for Xor {
    fn execute(instruction: &Instruction, cpu: &mut CPU) -> Result<()> {
        let src = instruction.get_operand(0)?;

        match *src {
            Operand::Register(r) => {
                cpu.reg[REG_A] ^= cpu.reg[r];
            },
            Operand::RegisterPairAddr(h, l) => {
                let addr = cpu.read_reg_addr(h, l);
                cpu.reg[REG_A] ^= cpu.load_mem(addr);
            },
            Operand::Immediate(BYTE) => {
                cpu.reg[REG_A] ^= instruction.get_immediate_u8()?;
            }
            _ => {
                // TODO: Add error here
                println!("UNEXPECTED OPERAND {}", src);
            },
        };

        let res = cpu.reg[REG_A];
        cpu.flag_cond(FLAG_Z, res == 0);
        cpu.clear_flag(FLAG_N);
        cpu.clear_flag(FLAG_H);
        cpu.clear_flag(FLAG_C);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use test_helpers::{execute_all, execute_instruction, test_cpu};
    use definition::Mnemonic;
    use constants::*;

    #[test]
    fn execute_xors() {
        execute_all(Mnemonic::XOR);
    }

    #[test]
    fn test_xor_reg_with_a() {
        let reg_codes: [(u16, usize); 7] = [
            (0xaf, REG_A),
            (0xa8, REG_B),
            (0xa9, REG_C),
            (0xaa, REG_D),
            (0xab, REG_E),
            (0xac, REG_H),
            (0xad, REG_L),
        ];

        for &(c, r) in reg_codes.iter() {
            let mut cpu = test_cpu();
            cpu.reg[REG_A] = 0b0001_1100;
            if r != REG_A {
                cpu.reg[r] = 0b0011_1000;
            }
            execute_instruction(&mut cpu, c, None);
            if r != REG_A {
                assert_eq!(cpu.reg[REG_A], 0b0010_0100);
                assert_eq!(cpu.flag, 0b0000_0000);
            } else {
                assert_eq!(cpu.reg[REG_A], 0b0000_0000);
                assert_eq!(cpu.flag, 0b1000_0000);
            }
        }
    }

    #[test]
    fn test_xor_immediate_with_a() {
        let mut cpu = test_cpu();
        cpu.reg[REG_A] = 0b0001_1100;
        execute_instruction(&mut cpu, 0xee, Some(0b0011_1000));
        assert_eq!(cpu.reg[REG_A], 0b0010_0100);
        assert_eq!(cpu.flag, 0b0000_0000);
    }

    #[test]
    fn test_xor_regpair_addr_with_a() {
        let mut cpu = test_cpu();
        cpu.store_mem(0xff22, 0b0011_1000);
        cpu.reg[REG_A] = 0b0001_1100;
        cpu.reg[REG_H] = 0xff;
        cpu.reg[REG_L] = 0x22;
        execute_instruction(&mut cpu, 0xae, None);
        assert_eq!(cpu.reg[REG_A], 0b0010_0100);
        assert_eq!(cpu.flag, 0b0000_0000);
    }
}
