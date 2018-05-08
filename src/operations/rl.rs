use cpu::CPU;
use instructions::Instruction;
use definition::Operand;
use errors::*;
use constants::*;
use operations::Execute;

pub struct RotateLeft;

impl Execute for RotateLeft {
    fn execute(instruction: &Instruction, cpu: &mut CPU) -> Result<()> {
        let src = instruction.get_operand(0)?;

        match *src {
            Operand::Register(r) => {
                let val = cpu.reg[r];
                let msb = val >> 7;
                let mut res = val << 1;

                if cpu.flag_is_set(FLAG_C) {
                    res |= 1;
                }

                cpu.reg[r] = res;

                cpu.flag_cond(FLAG_Z, res == 0);
                cpu.clear_flag(FLAG_N);
                cpu.clear_flag(FLAG_H);
                cpu.flag_cond(FLAG_C, msb == 1);
            },
            Operand::RegisterPairAddr(h, l) => {
                let addr = cpu.read_reg_addr(h, l);
                let val = cpu.load_mem(addr);
                let msb = val >> 7;
                let mut res = (val << 1) | msb;

                if cpu.flag_is_set(FLAG_C) {
                    res |= 1;
                }

                cpu.store_mem(addr, res);

                cpu.flag_cond(FLAG_Z, res == 0);
                cpu.clear_flag(FLAG_N);
                cpu.clear_flag(FLAG_H);
                cpu.flag_cond(FLAG_C, msb == 1);
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
    use test_helpers::{execute_all, execute_instruction, test_cpu};
    use definition::Mnemonic;
    use constants::*;

    #[test]
    fn execute_rl() {
        execute_all(Mnemonic::RL);
    }

    #[test]
    fn test_rl_reg_no_carry() {
        let reg_codes: [(u16, usize); 7] = [
            (0xcb17, REG_A),
            (0xcb10, REG_B),
            (0xcb11, REG_C),
            (0xcb12, REG_D),
            (0xcb13, REG_E),
            (0xcb14, REG_H),
            (0xcb15, REG_L),
        ];

        for &(c, r) in reg_codes.iter() {
            let mut cpu = test_cpu();
            cpu.reg[r] = 0b0111_1111;
            execute_instruction(&mut cpu, c, None);
            assert_eq!(cpu.reg[r], 0b1111_1110);
            assert_eq!(cpu.flag, 0b0000_0000);
        }
    }

    #[test]
    fn test_rl_reg_carry() {
        let reg_codes: [(u16, usize); 7] = [
            (0xcb17, REG_A),
            (0xcb10, REG_B),
            (0xcb11, REG_C),
            (0xcb12, REG_D),
            (0xcb13, REG_E),
            (0xcb14, REG_H),
            (0xcb15, REG_L),
        ];

        for &(c, r) in reg_codes.iter() {
            let mut cpu = test_cpu();
            cpu.reg[r] = 0b1111_1111;
            execute_instruction(&mut cpu, c, None);
            assert_eq!(cpu.reg[r], 0b1111_1110);
            assert_eq!(cpu.flag, 0b0001_0000);
        }
    }

    // TODO: Test with carry flag set before RL

    #[test]
    fn test_rl_regpair_addr_no_carry() {
        let mut cpu = test_cpu();
        cpu.store_mem(0xff22, 0b0111_1111);
        cpu.reg[REG_H] = 0xff;
        cpu.reg[REG_L] = 0x22;
        execute_instruction(&mut cpu, 0xcb06, None);
        assert_eq!(cpu.load_mem(0xff22), 0b1111_1110);
        assert_eq!(cpu.flag, 0b0000_0000);
    }
}
