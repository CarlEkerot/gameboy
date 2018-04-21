use cpu::CPU;
use instructions::Instruction;
use definition::Operand;
use errors::*;
use operations::Execute;

pub struct Reset;

impl Execute for Reset {
    fn execute(instruction: &Instruction, cpu: &mut CPU) -> Result<()> {

        let bit = instruction.get_operand(0)?;
        let src = instruction.get_operand(1)?;

        match (src, bit) {
            (&Operand::Register(r), &Operand::Bit(b)) => {
                cpu.reg[r] &= !(1u8 << b);
            },
            (&Operand::RegisterPairAddr(h, l), &Operand::Bit(b)) => {
                let addr = cpu.read_reg_addr(h, l);
                let val = cpu.ram.load(addr) & !(1u8 << b);
                cpu.ram.store(addr, val);
            },
            _ => {
                // TODO: Add error here
                println!("UNEXPECTED OPERAND {}", src);
            },
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use test_helpers::execute_all;
    use definition::Mnemonic;

    #[test]
    fn execute_res() {
        execute_all(Mnemonic::RES);
    }
}
