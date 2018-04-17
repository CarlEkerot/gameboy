use cpu::CPU;
use instructions::{Instruction, Operand};
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
    use test_helpers::execute_all;
    use instructions::Mnemonic;

    #[test]
    fn execute_loads() {
        execute_all(Mnemonic::LDD);
    }
}
