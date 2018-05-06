use cpu::CPU;
use instructions::Instruction;
use definition::Operand;
use errors::*;
use constants::*;
use operations::Execute;

pub struct Subtract;

impl Execute for Subtract {
    fn execute(instruction: &Instruction, cpu: &mut CPU) -> Result<()> {
        let src = instruction.get_operand(0)?;

        match *src {
            Operand::Register(r) => {
                let a = cpu.reg[REG_A];
                let b = cpu.reg[r];

                // TODO: CHECK THIS
                let val = a.wrapping_sub(b);
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

                let val = a.wrapping_sub(b);
                cpu.reg[REG_A] = val;

                cpu.flag_cond(FLAG_Z, val == 0);
                cpu.set_flag(FLAG_N);
                cpu.set_half_carry(a as usize, b as usize);
                cpu.set_carry(a as usize, b as usize);
            },
            Operand::Immediate(BYTE) => {
                let a = cpu.reg[REG_A];
                let b = instruction.get_immediate_u8()?;

                let val = a.wrapping_sub(b);
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
    fn execute_subs() {
        execute_all(Mnemonic::SUB);
    }

    #[test]
    fn test_sub_reg_from_a() {
        let reg_codes: [(u16, usize); 7] = [
            (0x97, REG_A),
            (0x90, REG_B),
            (0x91, REG_C),
            (0x92, REG_D),
            (0x93, REG_E),
            (0x94, REG_H),
            (0x95, REG_L),
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
                assert_eq!(cpu.reg[REG_A], 0x69);
            } else {
                assert_eq!(cpu.reg[REG_A], 0x00);
            }
        }
    }

    #[test]
    fn test_sub_reg_from_a_half_carry() {
        let mem = Memory::default();
        let mut cpu = CPU::new(mem);
        cpu.reg[REG_A] = 0x7f;
        cpu.reg[REG_B] = 0x01;
        execute_instruction(&mut cpu, 0x90, None);
        assert_eq!(cpu.reg[REG_A], 0x7e);
        assert_eq!(cpu.flag, 0b0110_0000);
    }

    #[test]
    fn test_sub_reg_from_a_carry() {
        let mem = Memory::default();
        let mut cpu = CPU::new(mem);
        cpu.reg[REG_A] = 0x80;
        cpu.reg[REG_B] = 0x81;
        execute_instruction(&mut cpu, 0x90, None);
        assert_eq!(cpu.reg[REG_A], 0xff);
        assert_eq!(cpu.flag, 0b0101_0000);
    }

    #[test]
    fn test_sub_reg_from_a_zero() {
        let mem = Memory::default();
        let mut cpu = CPU::new(mem);
        cpu.reg[REG_A] = 0x00;
        cpu.reg[REG_B] = 0x00;
        execute_instruction(&mut cpu, 0x90, None);
        assert_eq!(cpu.reg[REG_A], 0x00);
        assert_eq!(cpu.flag, 0b1100_0000);
    }

    #[test]
    fn test_sub_immediate_from_a() {
        let mem = Memory::default();
        let mut cpu = CPU::new(mem);
        cpu.reg[REG_A] = 0x9a;
        execute_instruction(&mut cpu, 0xd6, Some(0x11));
        assert_eq!(cpu.reg[REG_A], 0x89);
    }

    #[test]
    fn test_sub_regpair_addr_from_a() {
        let mut mem = Memory::default();
        mem.store(0xff22, 0x11);
        let mut cpu = CPU::new(mem);
        cpu.reg[REG_A] = 0x9a;
        cpu.reg[REG_H] = 0xff;
        cpu.reg[REG_L] = 0x22;
        execute_instruction(&mut cpu, 0x96, None);
        assert_eq!(cpu.reg[REG_A], 0x89);
    }
}
