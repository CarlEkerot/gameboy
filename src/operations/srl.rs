use cpu::CPU;
use instructions::{Instruction, Operand};
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
    use test_helpers::execute_all;
    use instructions::Mnemonic;

    #[test]
    fn execute_srl() {
        execute_all(Mnemonic::SRL);
    }
}
