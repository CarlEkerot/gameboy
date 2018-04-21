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
    use test_helpers::execute_all;
    use definition::Mnemonic;

    #[test]
    fn execute_ldh() {
        execute_all(Mnemonic::LDH);
    }
}
