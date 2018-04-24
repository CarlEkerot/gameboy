use cpu::CPU;
use instructions::Instruction;
use definition::Operand;
use errors::*;
use constants::*;
use operations::Execute;

pub struct ShiftRightLogical;

impl Execute for ShiftRightLogical {
    fn execute(instruction: &Instruction, cpu: &mut CPU) -> Result<()> {
        let src = instruction.get_operand(0)?;

        match *src {
            Operand::Register(r) => {
                let val = cpu.reg[r];
                let lsb = val & 0x1;
                let res = (val >> 1) & 0xff;

                cpu.reg[r] = res;

                cpu.flag_cond(FLAG_Z, res == 0);
                cpu.clear_flag(FLAG_N);
                cpu.clear_flag(FLAG_H);
                cpu.flag_cond(FLAG_C, lsb == 1);
            },
            Operand::RegisterPairAddr(h, l) => {
                let addr = cpu.read_reg_addr(h, l);
                let val = cpu.ram.load(addr);
                let lsb = val & 0x1;
                let res = (val >> 1) & 0xff;

                cpu.ram.store(addr, res);

                cpu.flag_cond(FLAG_Z, res == 0);
                cpu.clear_flag(FLAG_N);
                cpu.clear_flag(FLAG_H);
                cpu.flag_cond(FLAG_C, lsb == 1);
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
    fn execute_srl() {
        execute_all(Mnemonic::SRL);
    }

    #[test]
    fn test_srl_reg_no_carry() {
        let reg_codes: [(u16, usize); 7] = [
            (0xcb3f, REG_A),
            (0xcb38, REG_B),
            (0xcb39, REG_C),
            (0xcb3a, REG_D),
            (0xcb3b, REG_E),
            (0xcb3c, REG_H),
            (0xcb3d, REG_L),
        ];

        for &(c, r) in reg_codes.iter() {
            let mut mem = Memory::default();
            let mut cpu = CPU::new(mem);
            cpu.reg[r] = 0b1111_1110;
            execute_instruction(&mut cpu, c, None);
            assert_eq!(cpu.reg[r], 0b0111_1111);
            assert_eq!(cpu.flag, 0b0000_0000);
        }
    }

    #[test]
    fn test_srl_reg_carry() {
        let reg_codes: [(u16, usize); 7] = [
            (0xcb3f, REG_A),
            (0xcb38, REG_B),
            (0xcb39, REG_C),
            (0xcb3a, REG_D),
            (0xcb3b, REG_E),
            (0xcb3c, REG_H),
            (0xcb3d, REG_L),
        ];

        for &(c, r) in reg_codes.iter() {
            let mut mem = Memory::default();
            let mut cpu = CPU::new(mem);
            cpu.reg[r] = 0b1111_1111;
            execute_instruction(&mut cpu, c, None);
            assert_eq!(cpu.reg[r], 0b0111_1111);
            assert_eq!(cpu.flag, 0b0001_0000);
        }
    }

    #[test]
    fn test_srl_regpair_addr_no_carry() {
        let mut mem = Memory::default();
        mem.store(0xff22, 0b1111_1110);
        let mut cpu = CPU::new(mem);
        cpu.reg[REG_H] = 0xff;
        cpu.reg[REG_L] = 0x22;
        execute_instruction(&mut cpu, 0xcb3e, None);
        assert_eq!(cpu.ram.load(0xff22), 0b0111_1111);
        assert_eq!(cpu.flag, 0b0000_0000);
    }
}
