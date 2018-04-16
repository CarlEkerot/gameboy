use cpu::CPU;
use instructions::{Instruction, Operand};
use errors::*;
use operations::Execute;

pub struct LoadIncrease;

impl Execute for LoadIncrease {
    fn execute(instruction: &Instruction, cpu: &mut CPU) -> Result<()> {
        let dst = instruction.definition.operands.get(0)
            .chain_err(|| "Missing destination operand")?;
        let src = instruction.definition.operands.get(1)
            .chain_err(|| "Missing source operand")?;


        match (dst, src) {
            (&Operand::RegisterPairAddr(h, l), &Operand::Register(r)) => {
                let mut addr = ((cpu.reg[h] as usize) << 8) | (cpu.reg[l] as usize);
                cpu.ram.store(addr, cpu.reg[r]);

                // Increase
                addr += 1;
                cpu.reg[h] = (addr >> 8) as u8;
                cpu.reg[l] = (addr & 0xff) as u8;
            },
            (&Operand::Register(r), &Operand::RegisterPairAddr(h, l)) => {
                let mut addr = ((cpu.reg[h] as usize) << 8) | (cpu.reg[l] as usize);
                cpu.reg[r] = cpu.ram.load(addr);

                // Increase
                addr += 1;
                cpu.reg[h] = (addr >> 8) as u8;
                cpu.reg[l] = (addr & 0xff) as u8;
            },
            _ => {
                println!("UNEXPECTED OPERANDS IN LDI");
            }
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use test_helpers::execute_all;
    use instructions::Mnemonic;

    #[test]
    fn execute_ldis() {
        execute_all(Mnemonic::LDI);
    }
}
