use cpu::CPU;
use instructions::Instruction;
use definition::Operand;
use errors::*;
use constants::*;
use operations::Execute;

pub struct And;

impl Execute for And {
    fn execute(instruction: &Instruction, cpu: &mut CPU) -> Result<()> {
        let src = instruction.get_operand(0)?;

        match *src {
            Operand::Register(r) => {
                cpu.reg[REG_A] &= cpu.reg[r];
            },
            Operand::RegisterPairAddr(h, l) => {
                let addr = cpu.read_reg_addr(h, l);
                cpu.reg[REG_A] &= cpu.ram.load(addr);
            },
            Operand::Immediate(BYTE) => {
                cpu.reg[REG_A] &= instruction.get_immediate_u8()?;
            }
            _ => {
                // TODO: Add error here
                println!("UNEXPECTED OPERAND {}", src);
            },
        };

        let res = cpu.reg[REG_A];
        cpu.flag_cond(FLAG_Z, res == 0);
        cpu.clear_flag(FLAG_N);
        cpu.set_flag(FLAG_H);
        cpu.clear_flag(FLAG_C);
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
    fn execute_ands() {
        execute_all(Mnemonic::AND);
    }

    #[test]
    fn test_and_reg_with_a() {
        let reg_codes: [(u16, usize); 7] = [
            (0xa7, REG_A),
            (0xa0, REG_B),
            (0xa1, REG_C),
            (0xa2, REG_D),
            (0xa3, REG_E),
            (0xa4, REG_H),
            (0xa5, REG_L),
        ];

        for &(c, r) in reg_codes.iter() {
            let mut mem = Memory::default();
            let mut cpu = CPU::new(mem);
            cpu.reg[REG_A] = 0b0001_1100;
            if r != REG_A {
                cpu.reg[r] = 0b0011_1000;
            }
            execute_instruction(&mut cpu, c, None);
            if r != REG_A {
                assert_eq!(cpu.reg[REG_A], 0b0001_1000);
            } else {
                assert_eq!(cpu.reg[REG_A], 0b0001_1100);
            }
            assert_eq!(cpu.flag, 0b0010_0000);
        }
    }

    #[test]
    fn test_and_zero_with_a() {
        let mut mem = Memory::default();
        let mut cpu = CPU::new(mem);
        cpu.reg[REG_A] = 0b0001_1100;
        cpu.reg[REG_B] = 0b0000_0000;
        execute_instruction(&mut cpu, 0xa0, None);
        assert_eq!(cpu.reg[REG_A], 0b0000_0000);
        assert_eq!(cpu.flag, 0b1010_0000);
    }

    #[test]
    fn test_and_immediate_with_a() {
        let mut mem = Memory::default();
        let mut cpu = CPU::new(mem);
        cpu.reg[REG_A] = 0b0001_1100;
        execute_instruction(&mut cpu, 0xe6, Some(0b0011_1000));
        assert_eq!(cpu.reg[REG_A], 0b0001_1000);
        assert_eq!(cpu.flag, 0b0010_0000);
    }

    #[test]
    fn test_and_regpair_addr_with_a() {
        let mut mem = Memory::default();
        mem.store(0xff22, 0b0011_1000);
        let mut cpu = CPU::new(mem);
        cpu.reg[REG_A] = 0b0001_1100;
        cpu.reg[REG_H] = 0xff;
        cpu.reg[REG_L] = 0x22;
        execute_instruction(&mut cpu, 0xa6, None);
        assert_eq!(cpu.reg[REG_A], 0b0001_1000);
        assert_eq!(cpu.flag, 0b0010_0000);
    }
}
