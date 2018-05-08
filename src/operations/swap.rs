use cpu::CPU;
use instructions::Instruction;
use definition::Operand;
use errors::*;
use constants::*;
use operations::Execute;

pub struct Swap;

impl Execute for Swap {
    fn execute(instruction: &Instruction, cpu: &mut CPU) -> Result<()> {
        let src = instruction.get_operand(0)?;

        let res = match *src {
            Operand::Register(r) => {
                let hi = cpu.reg[r] >> 4;
                let lo = cpu.reg[r] & 0xf;
                let res = (lo << 4) | hi;
                cpu.reg[r] = res;
                res
            },
            Operand::RegisterPairAddr(h, l) => {
                let addr = cpu.read_reg_addr(h, l);
                let val = cpu.load_mem(addr);
                let hi = val >> 4;
                let lo = val & 0xf;
                let res = (lo << 4) | hi;
                cpu.store_mem(addr, res);
                res
            },
            _ => {
                // TODO: Add error here
                println!("UNEXPECTED OPERAND {}", src);
                0
            },
        };
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
    fn execute_swap() {
        execute_all(Mnemonic::SWAP);
    }

    #[test]
    fn test_swap_reg() {
        let reg_codes: [(u16, usize); 7] = [
            (0xcb37, REG_A),
            (0xcb30, REG_B),
            (0xcb31, REG_C),
            (0xcb32, REG_D),
            (0xcb33, REG_E),
            (0xcb34, REG_H),
            (0xcb35, REG_L),
        ];

        for &(c, r) in reg_codes.iter() {
            let mut cpu = test_cpu();
            cpu.reg[r] = 0x0f;
            execute_instruction(&mut cpu, c, None);
            assert_eq!(cpu.reg[r], 0xf0);
        }
    }

    #[test]
    fn test_swap_reg_zero() {
        let mut cpu = test_cpu();
        cpu.reg[REG_A] = 0x00;
        execute_instruction(&mut cpu, 0xcb37, None);
        assert_eq!(cpu.reg[REG_A], 0x00);
        assert_eq!(cpu.flag, 0b1000_0000);
    }

    #[test]
    fn test_swap_regpair_addr() {
        let mut cpu = test_cpu();
        cpu.store_mem(0xff22, 0xf0);
        cpu.reg[REG_H] = 0xff;
        cpu.reg[REG_L] = 0x22;
        execute_instruction(&mut cpu, 0xcb36, None);
        assert_eq!(cpu.load_mem(0xff22), 0x0f);
    }
}
