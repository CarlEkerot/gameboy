use cpu::CPU;
use instructions::{Instruction, Operand};
use errors::*;
use constants::*;
use operations::Execute;

pub struct SubtractCarry;

impl Execute for SubtractCarry {
    fn execute(instruction: &Instruction, cpu: &mut CPU) -> Result<()> {
        let src = instruction.get_operand(1)?;

        match *src {
            Operand::Register(r) => {
                let a = cpu.reg[REG_A];
                let b = cpu.reg[r];

                // TODO: CHECK THIS
                let mut val = a - b;

                if cpu.flag_is_set(FLAG_C) {
                    val -= 1;
                }

                cpu.reg[REG_A] = val;

                cpu.flag_cond(FLAG_Z, val == 0);
                cpu.set_flag(FLAG_N);
                cpu.set_half_carry(a as usize, b as usize);
                cpu.set_carry(a as usize, b as usize);
            },
            Operand::RegisterPairAddr(h, l) => {
                let addr = cpu.read_reg_addr(h, l);
                let a = cpu.reg[REG_A];
                let b = cpu.ram.load(addr);

                let mut val = a - b;

                if cpu.flag_is_set(FLAG_C) {
                    val -= 1;
                }

                cpu.reg[REG_A] = val;

                cpu.flag_cond(FLAG_Z, val == 0);
                cpu.set_flag(FLAG_N);
                cpu.set_half_carry(a as usize, b as usize);
                cpu.set_carry(a as usize, b as usize);
            },
            Operand::Immediate(BYTE) => {
                let a = cpu.reg[REG_A];
                let b = instruction.get_immediate_u8()?;
                let mut val = a - b;

                if cpu.flag_is_set(FLAG_C) {
                    val -= 1;
                }

                cpu.reg[REG_A] = val;

                cpu.flag_cond(FLAG_Z, val == 0);
                cpu.set_flag(FLAG_N);
                cpu.set_half_carry(a as usize, b as usize);
                cpu.set_carry(a as usize, b as usize);

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
    fn execute_sbcs() {
        execute_all(Mnemonic::SBC);
    }
}
