use cpu::CPU;
use instructions::Instruction;
use definition::Operand;
use errors::*;
use operations::Execute;

pub struct LoadDecrease;

impl Execute for LoadDecrease {
    fn execute(instruction: &Instruction, cpu: &mut CPU) -> Result<()> {
        let dst = instruction.get_operand(0)?;
        let src = instruction.get_operand(1)?;

        match (dst, src) {
            (&Operand::RegisterPairAddr(h, l), &Operand::Register(r)) => {
                let mut addr = cpu.read_reg_addr(h, l);
                cpu.ram.store(addr, cpu.reg[r]);
                addr -= 1;
                cpu.store_reg_short(h, l, addr as u16);
            },
            (&Operand::Register(r), &Operand::RegisterPairAddr(h, l)) => {
                let mut addr = cpu.read_reg_addr(h, l);
                cpu.reg[r] = cpu.ram.load(addr);
                addr -= 1;
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
    use test_helpers::{execute_all, execute_instruction};
    use definition::Mnemonic;
    use cpu::CPU;
    use memory::Memory;
    use constants::*;

    #[test]
    fn execute_ldd() {
        execute_all(Mnemonic::LDD);
    }

    #[test]
    fn test_ldd_hl_addr_to_a() {
        let mut mem = Memory::default();
        mem.store(0xff22, 0xab);
        let mut cpu = CPU::new(mem);
        cpu.reg[REG_H] = 0xff;
        cpu.reg[REG_L] = 0x22;
        execute_instruction(&mut cpu, 0x3a, None);
        assert_eq!(cpu.reg[REG_A], 0xab);
        assert_eq!(cpu.reg[REG_H], 0xff);
        assert_eq!(cpu.reg[REG_L], 0x21);
    }

    #[test]
    fn test_ldd_a_to_hl_addr() {
        let mut mem = Memory::default();
        let mut cpu = CPU::new(mem);
        cpu.reg[REG_A] = 0xab;
        cpu.reg[REG_H] = 0xff;
        cpu.reg[REG_L] = 0x22;
        execute_instruction(&mut cpu, 0x32, None);
        assert_eq!(cpu.ram.load(0xff22), 0xab);
        assert_eq!(cpu.reg[REG_H], 0xff);
        assert_eq!(cpu.reg[REG_L], 0x21);
    }
}
