use cpu::CPU;
use instructions::Instruction;
use definition::Operand;
use errors::*;
use operations::Execute;

pub struct LoadIncrease;

impl Execute for LoadIncrease {
    fn execute(instruction: &Instruction, cpu: &mut CPU) -> Result<()> {
        let dst = instruction.get_operand(0)?;
        let src = instruction.get_operand(1)?;

        match (dst, src) {
            (&Operand::RegisterPairAddr(h, l), &Operand::Register(r)) => {
                let mut addr = cpu.read_reg_addr(h, l);
                cpu.store_mem(addr, cpu.reg[r]);
                addr += 1;
                cpu.store_reg_short(h, l, addr as u16);
            },
            (&Operand::Register(r), &Operand::RegisterPairAddr(h, l)) => {
                let mut addr = cpu.read_reg_addr(h, l);
                cpu.reg[r] = cpu.load_mem(addr);
                addr += 1;
                cpu.store_reg_short(h, l, addr as u16);
            },
            _ => {
                println!("UNEXPECTED OPERANDS {} {}", src, dst);
            }
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
    fn execute_ldi() {
        execute_all(Mnemonic::LDI);
    }

    #[test]
    fn test_ldi_hl_addr_to_a() {
        let mut cpu = test_cpu();
        cpu.store_mem(0xff22, 0xab);
        cpu.reg[REG_H] = 0xff;
        cpu.reg[REG_L] = 0x22;
        execute_instruction(&mut cpu, 0x2a, None);
        assert_eq!(cpu.reg[REG_A], 0xab);
        assert_eq!(cpu.reg[REG_H], 0xff);
        assert_eq!(cpu.reg[REG_L], 0x23);
    }

    #[test]
    fn test_ldi_a_to_hl_addr() {
        let mut cpu = test_cpu();
        cpu.reg[REG_A] = 0xab;
        cpu.reg[REG_H] = 0xff;
        cpu.reg[REG_L] = 0x22;
        execute_instruction(&mut cpu, 0x22, None);
        assert_eq!(cpu.load_mem(0xff22), 0xab);
        assert_eq!(cpu.reg[REG_H], 0xff);
        assert_eq!(cpu.reg[REG_L], 0x23);
    }
}
