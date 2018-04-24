use cpu::CPU;
use instructions::Instruction;
use definition::Operand;
use errors::*;
use constants::*;
use operations::Execute;

pub struct RotateLeftCarry;

impl Execute for RotateLeftCarry {
    fn execute(instruction: &Instruction, cpu: &mut CPU) -> Result<()> {
        let src = instruction.get_operand(0)?;

        match *src {
            Operand::Register(r) => {
                let val = cpu.reg[r];
                let msb = val >> 7;
                let res = ((val << 1) & 0xff) | msb;
                cpu.reg[r] = res;

                cpu.flag_cond(FLAG_Z, res == 0);
                cpu.clear_flag(FLAG_N);
                cpu.clear_flag(FLAG_H);
                cpu.flag_cond(FLAG_C, msb == 1);
            },
            Operand::RegisterPairAddr(h, l) => {
                let addr = cpu.read_reg_addr(h, l);
                let val = cpu.ram.load(addr);
                let msb = val >> 7;
                let res = ((val << 1) & 0xff) | msb;
                cpu.ram.store(addr, res);

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
    use test_helpers::{execute_all, execute_instruction};
    use definition::Mnemonic;
    use cpu::CPU;
    use memory::Memory;
    use constants::*;

    #[test]
    fn execute_rlc() {
        execute_all(Mnemonic::RLC);
    }

    #[test]
    fn test_rlc_reg_no_carry() {
        let reg_codes: [(u16, usize); 7] = [
            (0xcb07, REG_A),
            (0xcb00, REG_B),
            (0xcb01, REG_C),
            (0xcb02, REG_D),
            (0xcb03, REG_E),
            (0xcb04, REG_H),
            (0xcb05, REG_L),
        ];

        for &(c, r) in reg_codes.iter() {
            let mut mem = Memory::default();
            let mut cpu = CPU::new(mem);
            cpu.reg[r] = 0b0111_1111;
            execute_instruction(&mut cpu, c, None);
            assert_eq!(cpu.reg[r], 0b1111_1110);
            assert_eq!(cpu.flag, 0b0000_0000);
        }
    }

    #[test]
    fn test_rlc_reg_carry() {
        let reg_codes: [(u16, usize); 7] = [
            (0xcb07, REG_A),
            (0xcb00, REG_B),
            (0xcb01, REG_C),
            (0xcb02, REG_D),
            (0xcb03, REG_E),
            (0xcb04, REG_H),
            (0xcb05, REG_L),
        ];

        for &(c, r) in reg_codes.iter() {
            let mut mem = Memory::default();
            let mut cpu = CPU::new(mem);
            cpu.reg[r] = 0b1111_1111;
            execute_instruction(&mut cpu, c, None);
            assert_eq!(cpu.reg[r], 0b1111_1111);
            assert_eq!(cpu.flag, 0b0001_0000);
        }
    }

    #[test]
    fn test_rlc_regpair_addr_no_carry() {
        let mut mem = Memory::default();
        mem.store(0xff22, 0b0111_1111);
        let mut cpu = CPU::new(mem);
        cpu.reg[REG_H] = 0xff;
        cpu.reg[REG_L] = 0x22;
        execute_instruction(&mut cpu, 0xcb06, None);
        assert_eq!(cpu.ram.load(0xff22), 0b1111_1110);
        assert_eq!(cpu.flag, 0b0000_0000);
    }
}
