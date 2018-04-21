use cpu::CPU;
use instructions::Instruction;
use definition::Operand;
use errors::*;
use constants::*;
use operations::Execute;

pub struct Bit;

impl Execute for Bit {
    fn execute(instruction: &Instruction, cpu: &mut CPU) -> Result<()> {

        let bit = instruction.get_operand(0)?;
        let src = instruction.get_operand(1)?;

        match (src, bit) {
            (&Operand::Register(r), &Operand::Bit(b)) => {
                let val = cpu.reg[r];
                let test_bit = 1u8 << b;

                cpu.flag_cond(FLAG_Z, val & test_bit != 0);
                cpu.clear_flag(FLAG_N);
                cpu.set_flag(FLAG_H);
            },
            (&Operand::RegisterPairAddr(h, l), &Operand::Bit(b)) => {
                let addr = cpu.read_reg_addr(h, l);
                let val = cpu.ram.load(addr);
                let test_bit = 1u8 << b;

                cpu.flag_cond(FLAG_Z, val & test_bit != 0);
                cpu.clear_flag(FLAG_N);
                cpu.set_flag(FLAG_H);
            },
            _ => {
                // TODO: Add error here
                println!("UNEXPECTED OPERAND {}", src);
            },
        };
        let val = cpu.reg[REG_A];
        let lsb = val & 0x1;
        let res = ((val >> 1) & 0xff) | (lsb << 7);
        cpu.reg[REG_A] = res;

        cpu.flag_cond(FLAG_Z, res == 0);
        cpu.clear_flag(FLAG_N);
        cpu.clear_flag(FLAG_H);
        cpu.flag_cond(FLAG_C, lsb == 1);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use test_helpers::execute_all;
    use definition::Mnemonic;

    #[test]
    fn execute_bit() {
        execute_all(Mnemonic::BIT);
    }
}
