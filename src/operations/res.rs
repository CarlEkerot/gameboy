use cpu::CPU;
use instructions::Instruction;
use definition::Operand;
use errors::*;
use operations::Execute;

pub struct Reset;

impl Execute for Reset {
    fn execute(instruction: &Instruction, cpu: &mut CPU) -> Result<()> {

        let bit = instruction.get_operand(0)?;
        let src = instruction.get_operand(1)?;

        match (src, bit) {
            (&Operand::Register(r), &Operand::Bit(b)) => {
                cpu.reg[r] &= !(1u8 << b);
            },
            (&Operand::RegisterPairAddr(h, l), &Operand::Bit(b)) => {
                let addr = cpu.read_reg_addr(h, l);
                let val = cpu.load_mem(addr) & !(1u8 << b);
                cpu.store_mem(addr, val);
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
    use test_helpers::{execute_all, execute_instruction, test_cpu};
    use definition::Mnemonic;
    use constants::*;

    #[test]
    fn execute_res() {
        execute_all(Mnemonic::RES);
    }

    #[test]
    fn test_res_reg() {
        let reg_codes: [(u16, usize); 7] = [
            (0xcb87, REG_A),
            (0xcb80, REG_B),
            (0xcb81, REG_C),
            (0xcb82, REG_D),
            (0xcb83, REG_E),
            (0xcb84, REG_H),
            (0xcb85, REG_L),
        ];

        for bit in 0..8 {
            for &(c, r) in reg_codes.iter() {
                let mut cpu = test_cpu();
                cpu.reg[r] = 1 << bit;
                execute_instruction(&mut cpu, c + 8 * bit, None);
                assert_eq!(cpu.reg[r], 0);
            }
        }
    }

    #[test]
    fn test_res_regpair_addr() {
        for bit in 0..8 {
            let mut cpu = test_cpu();
            cpu.store_mem(0xff22, 1u8 << bit);
            cpu.reg[REG_H] = 0xff;
            cpu.reg[REG_L] = 0x22;
            execute_instruction(&mut cpu, 0xcb86 + 8 * bit, None);
            assert_eq!(cpu.load_mem(0xff22), 0);
        }
    }
}
