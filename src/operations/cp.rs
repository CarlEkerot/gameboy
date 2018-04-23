use cpu::CPU;
use instructions::Instruction;
use definition::Operand;
use errors::*;
use constants::*;
use operations::Execute;

pub struct Compare;

impl Execute for Compare {
    fn execute(instruction: &Instruction, cpu: &mut CPU) -> Result<()> {
        let src = instruction.get_operand(0)?;

        let a = cpu.reg[REG_A];
        let b = match *src {
            Operand::Register(r) => cpu.reg[r],
            Operand::RegisterPairAddr(h, l) => {
                let addr = cpu.read_reg_addr(h, l);
                cpu.ram.load(addr)
            },
            Operand::Immediate(BYTE) => instruction.get_immediate_u8()?,
            _ => {
                // TODO: Add error here
                println!("UNEXPECTED OPERAND {}", src);
                0
            },
        };

        cpu.flag_cond(FLAG_Z, a == b);
        cpu.set_flag(FLAG_N);
        cpu.set_half_carry(a as usize, b as usize);
        cpu.flag_cond(FLAG_C, a < b);
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
    fn execute_cps() {
        execute_all(Mnemonic::CP);
    }

    #[test]
    fn test_cp_reg_with_a() {
        let reg_codes: [(u16, usize); 7] = [
            (0xbf, REG_A),
            (0xb8, REG_B),
            (0xb9, REG_C),
            (0xba, REG_D),
            (0xbb, REG_E),
            (0xbc, REG_H),
            (0xbd, REG_L),
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
                assert_eq!(cpu.flag, 0b0100_0000);
            } else {
                assert_eq!(cpu.flag, 0b1110_0000);
            }
        }
    }

    #[test]
    fn test_cp_immediate_less_than_a() {
        let mut mem = Memory::default();
        let mut cpu = CPU::new(mem);
        cpu.reg[REG_A] = 0x9a;
        execute_instruction(&mut cpu, 0xfe, Some(0x11));
        assert_eq!(cpu.flag, 0b0100_0000);
    }

    #[test]
    fn test_cp_immediate_greater_than_a() {
        let mut mem = Memory::default();
        let mut cpu = CPU::new(mem);
        cpu.reg[REG_A] = 0x9a;
        execute_instruction(&mut cpu, 0xfe, Some(0xd1));
        assert_eq!(cpu.flag, 0b0101_0000);
    }

    #[test]
    fn test_cp_regpair_addr_with_a() {
        let mut mem = Memory::default();
        mem.store(0xff22, 0x11);
        let mut cpu = CPU::new(mem);
        cpu.reg[REG_A] = 0x9a;
        cpu.reg[REG_H] = 0xff;
        cpu.reg[REG_L] = 0x22;
        execute_instruction(&mut cpu, 0xbe, None);
        assert_eq!(cpu.flag, 0b0100_0000);
    }
}
