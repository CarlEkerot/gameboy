use cpu::CPU;
use instructions::Instruction;
use definition::Operand;
use errors::*;
use constants::*;
use operations::Execute;

pub struct LoadOffset;

impl Execute for LoadOffset {
    fn execute(instruction: &Instruction, cpu: &mut CPU) -> Result<()> {
        let dst = instruction.get_operand(0)?;
        let src = instruction.get_operand(1)?;

        match (dst, src) {
            (&Operand::Address(BYTE), &Operand::Register(REG_A)) => {
                let offset = instruction.get_immediate_usize()?;
                let addr = OFFSET_BASE + offset;
                cpu.ram.store(addr, cpu.reg[REG_A]);
            },
            (&Operand::Register(REG_A), &Operand::Address(BYTE)) => {
                let offset = instruction.get_immediate_usize()?;
                let addr = OFFSET_BASE + offset;
                cpu.reg[REG_A] = cpu.ram.load(addr);
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
    fn execute_ldh() {
        execute_all(Mnemonic::LDH);
    }

    #[test]
    fn test_ldh_a_to_immediate_offset_addr() {
        let mem = Memory::default();
        let mut cpu = CPU::new(mem);
        cpu.reg[REG_A] = 0xab;
        execute_instruction(&mut cpu, 0xe0, Some(0x22));
        assert_eq!(cpu.ram.load(0xff22), 0xab);
    }

    #[test]
    fn test_ldhimmediate_offset_addr_to_a() {
        let mut mem = Memory::default();
        mem.store(0xff22, 0xab);
        let mut cpu = CPU::new(mem);
        execute_instruction(&mut cpu, 0xf0, Some(0x22));
        assert_eq!(cpu.reg[REG_A], 0xab);
    }
}
