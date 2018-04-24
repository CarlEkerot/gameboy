use cpu::CPU;
use instructions::Instruction;
use definition::Operand;
use errors::*;
use constants::*;
use operations::Execute;

pub struct Jump;

impl Execute for Jump {
    fn execute(instruction: &Instruction, cpu: &mut CPU) -> Result<()> {
        let op1 = instruction.get_operand(0)?;
        let op2 = instruction.get_operand(1)?;

        match (op1, op2) {
            (&Operand::Address(SHORT), &Operand::None) => {
                cpu.pc = instruction.get_immediate_u16()?;
            },
            (&Operand::RegisterPairAddr(h, l), &Operand::None) => {
                let a = cpu.read_reg_addr(h, l);
                cpu.pc = a as u16;
            },
            (&Operand::Zero, &Operand::Address(SHORT))  => {
                if cpu.flag_is_set(FLAG_Z) {
                    cpu.pc = instruction.get_immediate_u16()?;
                }
            },
            (&Operand::NonZero, &Operand::Address(SHORT)) => {
                if !cpu.flag_is_set(FLAG_Z) {
                    cpu.pc = instruction.get_immediate_u16()?;
                }
            },
            (&Operand::Carry, &Operand::Address(SHORT)) => {
                if cpu.flag_is_set(FLAG_C) {
                    cpu.pc = instruction.get_immediate_u16()?;
                }
            },
            (&Operand::NonCarry, &Operand::Address(SHORT)) => {
                if !cpu.flag_is_set(FLAG_C) {
                    cpu.pc = instruction.get_immediate_u16()?;
                }
            },
            _ => {
                println!("UNEXPECTED OPERAND {} {}", op1, op2);
            }
        };

        cpu.pc = cpu.pc.wrapping_sub(instruction.definition.length as u16);

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
    fn execute_jp() {
        execute_all(Mnemonic::JP);
    }

    #[test]
    fn test_jp_immediate_addr() {
        let mut mem = Memory::default();
        let mut cpu = CPU::new(mem);
        execute_instruction(&mut cpu, 0xc3, Some(0xff22));
        assert_eq!(cpu.pc, 0xff22);
    }

    #[test]
    fn test_jp_addr_flag() {
        let flag_set_codes: [(u16, u8, bool); 4] = [
            (0xc2, FLAG_Z, false),
            (0xca, FLAG_Z, true),
            (0xd2, FLAG_C, false),
            (0xda, FLAG_C, true),
        ];

        for &(c, f, s) in flag_set_codes.iter() {
            let mut mem = Memory::default();
            let mut cpu = CPU::new(mem);
            cpu.flag_cond(f, s);
            execute_instruction(&mut cpu, c, Some(0xff22));
            assert_eq!(cpu.pc, 0xff22);
        }
    }

    #[test]
    fn test_jp_regpair_addr() {
        let mut mem = Memory::default();
        let mut cpu = CPU::new(mem);
        cpu.reg[REG_H] = 0xff;
        cpu.reg[REG_L] = 0x22;
        execute_instruction(&mut cpu, 0xe9, None);
        assert_eq!(cpu.pc, 0xff22);
    }
}
