use cpu::CPU;
use instructions::Instruction;
use definition::Operand;
use errors::*;
use constants::*;
use operations::Execute;

pub struct JumpRelative;

impl Execute for JumpRelative {
    fn execute(instruction: &Instruction, cpu: &mut CPU) -> Result<()> {
        let op1 = instruction.get_operand(0)?;
        let op2 = instruction.get_operand(1)?;

        let should_jump = match (op1, op2) {
            (&Operand::Offset(BYTE), &Operand::None) => true,
            (&Operand::Zero, &Operand::Offset(BYTE)) => cpu.flag_is_set(FLAG_Z),
            (&Operand::NonZero, &Operand::Offset(BYTE)) => !cpu.flag_is_set(FLAG_Z),
            (&Operand::Carry, &Operand::Offset(BYTE)) => cpu.flag_is_set(FLAG_C),
            (&Operand::NonCarry, &Operand::Offset(BYTE)) => !cpu.flag_is_set(FLAG_C),
            _ => {
                println!("UNEXPECTED OPERAND {} {}", op1, op2);
                false
            }
        };

        if should_jump {
            let offset = instruction.get_immediate_i8()?;
            cpu.pc = ((cpu.pc as i32) + offset as i32) as u16;
        }

        // TODO: Accommodate for next inc of program counter?

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
    fn execute_jr() {
        execute_all(Mnemonic::JR);
    }

    #[test]
    fn test_jr_immediate_positive_addr() {
        let mem = Memory::default();
        let mut cpu = CPU::new(mem);
        cpu.pc= 0xff22;
        execute_instruction(&mut cpu, 0x18, Some(0x10));
        assert_eq!(cpu.pc, 0xff32 + 2);
    }

    #[test]
    fn test_jr_immediate_negative_addr() {
        let mem = Memory::default();
        let mut cpu = CPU::new(mem);
        cpu.pc= 0xff22;
        execute_instruction(&mut cpu, 0x18, Some(-0x10i8 as u16));
        assert_eq!(cpu.pc, 0xff12 + 2);
    }

    #[test]
    fn test_jr_addr_flag() {
        let flag_set_codes: [(u16, u8, bool); 4] = [
            (0x20, FLAG_Z, false),
            (0x28, FLAG_Z, true),
            (0x30, FLAG_C, false),
            (0x38, FLAG_C, true),
        ];

        for &(c, f, s) in flag_set_codes.iter() {
            let mut mem = Memory::default();
            let mut cpu = CPU::new(mem);
            cpu.pc= 0xff22;
            cpu.flag_cond(f, s);
            execute_instruction(&mut cpu, c, Some(0x10));
            assert_eq!(cpu.pc, 0xff32 + 2);
        }
    }
}
