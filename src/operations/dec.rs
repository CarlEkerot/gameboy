use cpu::CPU;
use instructions::{Instruction, Operand};
use errors::*;
use constants::*;
use operations::Execute;

pub struct Decrease;

impl Execute for Decrease {
    fn execute(instruction: &Instruction, cpu: &mut CPU) -> Result<()> {
        let dst = instruction.definition.operands.get(0)
            .chain_err(|| "Missing operand")?;

        match *dst {
            Operand::Register(r) => {
                let val = cpu.reg[r];
                cpu.reg[r] -= 1;

                // TODO: CHECK THIS!
                cpu.set_half_carry(val as usize, 1);

                cpu.set_flag(FLAG_N);
                if cpu.reg[r] == 0 {
                    cpu.set_flag(FLAG_Z);
                }
            },
            Operand::RegisterPair(h, l) => {
                let val = ((cpu.reg[h] as u16) << 8) | (cpu.reg[l] as u16) - 1;
                cpu.reg[h] = (val >> 8) as u8;
                cpu.reg[l] = (val & 0xff) as u8;
            },
            Operand::SP => {
                cpu.sp += 1;
            },
            Operand::RegisterPairAddr(h, l) => {
                let addr = ((cpu.reg[h] as usize) << 8) | (cpu.reg[l] as usize);
                let val = cpu.ram.load(addr);
                cpu.ram.store(addr, val - 1);
            },
            _ => {
                println!("UNEXPECTED OPERANDS IN DEC");
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
    fn execute_decs() {
        execute_all(Mnemonic::DEC);
    }
}
