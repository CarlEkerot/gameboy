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
                cpu.reg[r] = instruction.immediate.map(|d| d as u8)
                    .chain_err(|| "Missing immediate")?
            },
            (&Operand::Register(r1), &Operand::Register(r2)) => {
                cpu.reg[r1] = cpu.reg[r2];
            },
            (&Operand::Register(r), &Operand::RegisterPair(h, l)) => {
                let addr = ((cpu.reg[h] as usize) << 8) | (cpu.reg[l] as usize);
                cpu.reg[r] = cpu.ram.load(addr);
            },
            (&Operand::Register(r), &Operand::Address(SHORT)) => {
                let addr = instruction.immediate.map(|d| d as usize)
                    .chain_err(|| "Missing immediate")?;
                cpu.reg[r] = cpu.ram.load(addr);
            },
            (&Operand::Register(r), &Operand::COffset(BYTE)) => {
                let addr = OFFSET_BASE + cpu.reg[REG_C] as usize;
                cpu.reg[r] = cpu.ram.load(addr);
            },
            (&Operand::SP, &Operand::RegisterPair(h, l)) => {
                cpu.sp = ((cpu.reg[h] as u16) << 8) | (cpu.reg[l] as u16);
            },
            (&Operand::SP, &Operand::Immediate(SHORT)) => {
                cpu.sp = instruction.immediate.chain_err(|| "Missing immediate")?;
            },
            (&Operand::RegisterPair(h, l), &Operand::Immediate(SHORT)) => {
                let val = instruction.immediate.chain_err(|| "Missing immediate")?;
                cpu.reg[h] = (val >> 8) as u8;
                cpu.reg[l] = (val & 0xff) as u8;
            },
            (&Operand::RegisterPair(h, l), &Operand::Immediate(BYTE)) => {
                let addr = ((cpu.reg[h] as usize) << 8) | (cpu.reg[l] as usize);
                let val = instruction.immediate.map(|d| d as u8)
                    .chain_err(|| "Missing immediate")?;
                cpu.ram.store(addr, val);
            },
            (&Operand::RegisterPair(h, l), &Operand::Register(r)) => {
                let addr = ((cpu.reg[h] as usize) << 8) | (cpu.reg[l] as usize);
                cpu.ram.store(addr, cpu.reg[r]);
            },
            (&Operand::RegisterPair(h, l), &Operand::SPOffset(BYTE)) => {
                let offset = instruction.immediate.map(|d| d as i8)
                    .chain_err(|| "Missing immediate")?;
                let dst_addr = ((cpu.reg[h] as usize) << 8) | (cpu.reg[l] as usize);
                let src_addr = OFFSET_BASE + offset as usize;
                let val = cpu.ram.load(src_addr);
                cpu.ram.store(dst_addr, val);

                cpu.clear_flag(FLAG_Z);
                cpu.clear_flag(FLAG_N);

                // TODO: Double check these. Unsafe arithmetic.
                cpu.set_half_carry(OFFSET_BASE, offset as usize);
                cpu.set_carry(OFFSET_BASE, offset as usize);
            },
            (&Operand::Address(SHORT), &Operand::SP) => {
                let addr = instruction.immediate.map(|d| d as usize)
                    .chain_err(|| "Missing immediate")?;
                let val = cpu.ram.load(cpu.sp as usize);
                cpu.ram.store(addr, val);
            },
            (&Operand::Address(SHORT), &Operand::Register(r)) => {
                let addr = instruction.immediate.map(|d| d as usize)
                    .chain_err(|| "Missing immediate")?;
                let val = cpu.ram.load(cpu.reg[r] as usize);
                cpu.ram.store(addr, val);
            },
            (&Operand::COffset(BYTE), &Operand::Register(r)) => {
                let addr = OFFSET_BASE + cpu.reg[REG_C] as usize;
                cpu.ram.store(addr, cpu.reg[r])
            },
            _ => {
                // TODO: Add error here
                println!("UNEXPECTED OPERANDS IN LD");
            },
        };
        Ok(())
    }
}