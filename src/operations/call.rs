use cpu::CPU;
use instructions::Instruction;
use definition::Operand;
use errors::*;
use constants::*;
use operations::Execute;

pub struct Call;

impl Execute for Call {
    fn execute(instruction: &Instruction, cpu: &mut CPU) -> Result<()> {
        let op1 = instruction.get_operand(0)?;
        let op2 = instruction.get_operand(1)?;

        let should_call = match (op1, op2) {
            (&Operand::Address(SHORT), &Operand::None) => true,
            (&Operand::Zero, &Operand::Address(SHORT)) => cpu.flag_is_set(FLAG_Z),
            (&Operand::NonZero, &Operand::Address(SHORT)) => !cpu.flag_is_set(FLAG_Z),
            (&Operand::Carry, &Operand::Address(SHORT)) => cpu.flag_is_set(FLAG_C),
            (&Operand::NonCarry, &Operand::Address(SHORT)) => !cpu.flag_is_set(FLAG_C),
            _ => {
                println!("UNEXPECTED OPERAND {} {}", op1, op2);
                false
            }
        };

        let len = instruction.definition.length as u16;

        if should_call {
            let next_addr = cpu.pc.wrapping_add(len);
            cpu.stack_push((next_addr & 0xff) as u8);
            cpu.stack_push((next_addr >> 8) as u8);
            cpu.pc = instruction.get_immediate_u16()?;
        }

        // Accommodate for this instruction
        cpu.pc = cpu.pc.wrapping_sub(len);

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
    fn execute_call() {
        execute_all(Mnemonic::CALL);
    }

    #[test]
    fn test_call_immediate_addr() {
        let mem = Memory::default();
        let mut cpu = CPU::new(mem);
        cpu.pc = 0x2233;
        cpu.sp = 0x1122;
        execute_instruction(&mut cpu, 0xcd, Some(0xff22));
        assert_eq!(cpu.pc, 0xff22);
        assert_eq!(cpu.ram.load(0x1120), 0x22);
        assert_eq!(cpu.ram.load(0x1121), 0x36);
    }

    #[test]
    fn test_call_addr_flag() {
        let flag_set_codes: [(u16, u8, bool); 4] = [
            (0xc4, FLAG_Z, false),
            (0xcc, FLAG_Z, true),
            (0xd4, FLAG_C, false),
            (0xdc, FLAG_C, true),
        ];

        for &(c, f, s) in flag_set_codes.iter() {
            let mut mem = Memory::default();
            let mut cpu = CPU::new(mem);
            cpu.pc = 0x2233;
            cpu.sp = 0x1122;
            cpu.flag_cond(f, s);
            execute_instruction(&mut cpu, c, Some(0xff22));
            assert_eq!(cpu.pc, 0xff22);
            assert_eq!(cpu.ram.load(0x1120), 0x22);
            assert_eq!(cpu.ram.load(0x1121), 0x36);
        }
    }
}
