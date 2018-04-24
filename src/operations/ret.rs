use cpu::CPU;
use instructions::Instruction;
use definition::Operand;
use errors::*;
use constants::*;
use operations::Execute;

pub struct Return;

impl Execute for Return {
    fn execute(instruction: &Instruction, cpu: &mut CPU) -> Result<()> {
        let op = instruction.get_operand(0)?;

        let should_return = match *op {
            Operand::None => true,
            Operand::Zero => cpu.flag_is_set(FLAG_Z),
            Operand::NonZero => !cpu.flag_is_set(FLAG_Z),
            Operand::Carry => cpu.flag_is_set(FLAG_C),
            Operand::NonCarry => !cpu.flag_is_set(FLAG_C),
            _ => {
                println!("UNEXPECTED OPERAND {}", op);
                false
            }
        };

        if should_return {
            let hi = cpu.stack_pop() as u16;
            let lo = cpu.stack_pop() as u16;
            cpu.pc = (hi << 8) | lo;
        }

        // Accommodate for next inc of program counter
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
    fn execute_ret() {
        execute_all(Mnemonic::RET);
    }

    #[test]
    fn test_ret() {
        let mut mem = Memory::default();
        let mut cpu = CPU::new(mem);
        cpu.sp = 0x1122;
        cpu.stack_push(0x22);
        cpu.stack_push(0xff);
        execute_instruction(&mut cpu, 0xc9, None);
        assert_eq!(cpu.pc, 0xff22);
        assert_eq!(cpu.sp, 0x1122);
    }

    #[test]
    fn test_ret_flag() {
        let flag_set_codes: [(u16, u8, bool); 4] = [
            (0xc0, FLAG_Z, false),
            (0xc8, FLAG_Z, true),
            (0xd0, FLAG_C, false),
            (0xd8, FLAG_C, true),
        ];

        for &(c, f, s) in flag_set_codes.iter() {
            let mut mem = Memory::default();
            let mut cpu = CPU::new(mem);
            cpu.sp = 0x1122;
            cpu.stack_push(0x22);
            cpu.stack_push(0xff);
            cpu.flag_cond(f, s);
            execute_instruction(&mut cpu, c, None);
            assert_eq!(cpu.pc, 0xff22);
            assert_eq!(cpu.sp, 0x1122);
        }
    }
}
