use cpu::CPU;
use instructions::{Instruction, Operand};
use errors::*;
use constants::*;
use operations::Execute;

pub struct Load;

impl Execute for Load {
    fn execute(instruction: &Instruction, cpu: &mut CPU) -> Result<()> {
        let dst = instruction.definition.operands.get(0)
            .chain_err(|| "Missing destination operand")?;
        let src = instruction.definition.operands.get(1)
            .chain_err(|| "Missing source operand")?;

        match (dst, src) {
            (&Operand::Register(r), &Operand::Immediate(BYTE)) => {
                cpu.reg[r] = instruction.immediate.map(|i| i as u8)
                    .chain_err(|| "Missing immediate")?
            },
            (&Operand::Register(r1), &Operand::Register(r2)) => {
                cpu.reg[r1] = cpu.reg[r2];
            },
            (&Operand::Register(r), &Operand::Address(SHORT)) => {
                let addr = instruction.immediate.map(|d| d as usize)
                    .chain_err(|| "Missing immediate")?;
                cpu.reg[r] = cpu.ram.load(addr);
            },
            (&Operand::Register(r), &Operand::RegisterPairAddr(h, l)) => {
                let addr = ((cpu.reg[h] as usize) << 8) | (cpu.reg[l] as usize);
                cpu.reg[r] = cpu.ram.load(addr);
            },
            (&Operand::Register(r1), &Operand::RegisterAddr(r2)) => {
                let offset = cpu.reg[r2] as usize;
                let addr = OFFSET_BASE + offset;
                cpu.reg[r1] = cpu.ram.load(addr);
            },
            (&Operand::RegisterPair(h, l), &Operand::Immediate(SHORT)) => {
                let val = instruction.immediate.chain_err(|| "Missing immediate")?;
                cpu.reg[h] = (val >> 8) as u8;
                cpu.reg[l] = (val & 0xff) as u8;
            },
            (&Operand::RegisterPair(h, l), &Operand::SPOffset(BYTE)) => {
                let offset = instruction.immediate.map(|d| d as i16)
                    .chain_err(|| "Missing immediate")?;
                let sp = cpu.sp.clone();
                let val = sp as i16 + offset;

                cpu.reg[h] = (val >> 8) as u8;
                cpu.reg[l] = (val & 0xff) as u8;

                cpu.clear_flag(FLAG_Z);
                cpu.clear_flag(FLAG_N);

                // TODO: Double check these. _Really_ Unsafe arithmetic.
                cpu.set_half_carry(sp as usize, offset as usize);
                cpu.set_carry(sp as usize, offset as usize);
            },
            (&Operand::RegisterAddr(r1), &Operand::Register(r2)) => {
                let offset = cpu.reg[r1] as usize;
                let addr = OFFSET_BASE + offset;
                cpu.ram.store(addr, cpu.reg[r2]);
            },
            (&Operand::RegisterPairAddr(h, l), &Operand::Register(r)) => {
                let addr = ((cpu.reg[h] as usize) << 8) | (cpu.reg[l] as usize);
                cpu.ram.store(addr, cpu.reg[r]);
            },
            (&Operand::RegisterPairAddr(h, l), &Operand::Immediate(BYTE)) => {
                let addr = ((cpu.reg[h] as usize) << 8) | (cpu.reg[l] as usize);
                let val = instruction.immediate.map(|i| i as u8).chain_err(|| "Missing immediate")?;
                cpu.ram.store(addr, val);
            },
            (&Operand::Address(SHORT), &Operand::SP) => {
                // Stores two bytes in memory
                let addr = instruction.immediate.map(|a| a as usize)
                    .chain_err(|| "Missing immediate")?;
                cpu.ram.store(addr, (cpu.sp & 0xff) as u8);
                cpu.ram.store(addr + 1, (cpu.sp >> 8) as u8);
            },
            (&Operand::Address(BYTE), &Operand::Register(r)) => {
                let offset = instruction.immediate.map(|a| a as usize)
                    .chain_err(|| "Missing immediate")?;
                let addr = OFFSET_BASE + offset;
                cpu.ram.store(addr, cpu.reg[r]);
            },
            (&Operand::Address(SHORT), &Operand::Register(r)) => {
                // Stores two bytes in memory
                let addr = instruction.immediate.map(|a| a as usize)
                    .chain_err(|| "Missing immediate")?;
                cpu.ram.store(addr, cpu.reg[r]);
            },
            (&Operand::SP, &Operand::Immediate(SHORT)) => {
                let val = instruction.immediate.chain_err(|| "Missing immediate")?;
                cpu.sp = val;
            },
            (&Operand::SP, &Operand::RegisterPair(h, l)) => {
                cpu.sp = ((cpu.reg[h] as u16) << 8) | (cpu.reg[l] as u16);
            }
            _ => {
                // TODO: Add error here
                println!("UNEXPECTED OPERANDS IN LD {} {}", dst, src);
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
    fn execute_loads() {
        execute_all(Mnemonic::LD);
    }
}
