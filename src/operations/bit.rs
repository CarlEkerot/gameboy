use cpu::CPU;
use instructions::Instruction;
use definition::Operand;
use errors::*;
use constants::*;
use operations::Execute;

pub struct Bit;

impl Execute for Bit {
    fn execute(instruction: &Instruction, cpu: &mut CPU) -> Result<()> {

        let bit = instruction.get_operand(0)?;
        let src = instruction.get_operand(1)?;

        match (src, bit) {
            (&Operand::Register(r), &Operand::Bit(b)) => {
                let val = cpu.reg[r];
                let test_bit = 1u8 << b;

                // Set if zero
                cpu.flag_cond(FLAG_Z, (val & test_bit) == 0);
                cpu.clear_flag(FLAG_N);
                cpu.set_flag(FLAG_H);
            },
            (&Operand::RegisterPairAddr(h, l), &Operand::Bit(b)) => {
                let addr = cpu.read_reg_addr(h, l);
                let val = cpu.ram.load(addr);
                let test_bit = 1u8 << b;

                cpu.flag_cond(FLAG_Z, val & test_bit == 0);
                cpu.clear_flag(FLAG_N);
                cpu.set_flag(FLAG_H);
            },
            _ => {
                // TODO: Add error here
                println!("UNEXPECTED OPERAND {}", src);
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
    fn execute_bit() {
        execute_all(Mnemonic::BIT);
    }

    #[test]
    fn test_bit_reg() {
        let reg_codes: [(u16, usize); 7] = [
            (0xcb47, REG_A),
            (0xcb40, REG_B),
            (0xcb41, REG_C),
            (0xcb42, REG_D),
            (0xcb43, REG_E),
            (0xcb44, REG_H),
            (0xcb45, REG_L),
        ];

        for bit in 0..8 {
            for &(c, r) in reg_codes.iter() {
                let mut mem = Memory::default();
                let mut cpu = CPU::new(mem);
                cpu.reg[r] = 1u8 << bit;
                execute_instruction(&mut cpu, c + 8 * bit, None);
                assert_eq!(cpu.flag, 0b0010_0000);
                cpu.reg[r] = 0;
                execute_instruction(&mut cpu, c + 8 * bit, None);
                assert_eq!(cpu.flag, 0b1010_0000);
            }
        }
    }

    #[test]
    fn test_bit_regpair_addr() {
        for bit in 0..8 {
            let mut mem = Memory::default();
            mem.store(0xff22, 1u8 << bit);
            let mut cpu = CPU::new(mem);
            cpu.reg[REG_H] = 0xff;
            cpu.reg[REG_L] = 0x22;
            execute_instruction(&mut cpu, 0xcb46 + 8 * bit, None);
            assert_eq!(cpu.flag, 0b0010_0000);
            cpu.ram.store(0xff22, 0);
            execute_instruction(&mut cpu, 0xcb46 + 8 * bit, None);
            assert_eq!(cpu.flag, 0b1010_0000);
        }
    }
}
