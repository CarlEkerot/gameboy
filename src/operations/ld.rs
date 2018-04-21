use cpu::CPU;
use instructions::Instruction;
use definition::Operand;
use errors::*;
use constants::*;
use operations::Execute;

pub struct Load;

impl Execute for Load {
    fn execute(instruction: &Instruction, cpu: &mut CPU) -> Result<()> {
        let dst = instruction.get_operand(0)?;
        let src = instruction.get_operand(1)?;

        match (dst, src) {
            (&Operand::Register(r), &Operand::Immediate(BYTE)) => {
                cpu.reg[r] = instruction.get_immediate_u8()?;
            },
            (&Operand::Register(r1), &Operand::Register(r2)) => {
                cpu.reg[r1] = cpu.reg[r2];
            },
            (&Operand::Register(r), &Operand::Address(SHORT)) => {
                let addr = instruction.get_immediate_usize()?;
                cpu.reg[r] = cpu.ram.load(addr);
            },
            (&Operand::Register(r), &Operand::RegisterPairAddr(h, l)) => {
                let addr = cpu.read_reg_addr(h, l);
                cpu.reg[r] = cpu.ram.load(addr);
            },
            (&Operand::Register(r1), &Operand::RegisterAddr(r2)) => {
                let offset = cpu.reg[r2] as usize;
                let addr = OFFSET_BASE + offset;
                cpu.reg[r1] = cpu.ram.load(addr);
            },
            (&Operand::RegisterPair(h, l), &Operand::Immediate(SHORT)) => {
                cpu.store_reg_short(h, l, instruction.get_immediate_u16()?);
            },
            (&Operand::RegisterPair(h, l), &Operand::SPOffset(BYTE)) => {
                let offset = instruction.get_immediate_i16()?;
                let sp = cpu.sp;
                let val = sp as i16 + offset;

                cpu.store_reg_short(h, l, val as u16);

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
                let addr = cpu.read_reg_addr(h, l);
                cpu.ram.store(addr, cpu.reg[r]);
            },
            (&Operand::RegisterPairAddr(h, l), &Operand::Immediate(BYTE)) => {
                let addr = cpu.read_reg_addr(h, l);
                let val = instruction.get_immediate_u8()?;
                cpu.ram.store(addr, val);
            },
            (&Operand::Address(SHORT), &Operand::SP) => {
                // Stores two bytes in memory
                let addr = instruction.get_immediate_usize()?;
                cpu.ram.store(addr, (cpu.sp & 0xff) as u8);
                cpu.ram.store(addr + 1, (cpu.sp >> 8) as u8);
            },
            (&Operand::Address(SHORT), &Operand::Register(r)) => {
                // Stores two bytes in memory
                let addr = instruction.get_immediate_usize()?;
                cpu.ram.store(addr, cpu.reg[r]);
            },
            (&Operand::SP, &Operand::Immediate(SHORT)) => {
                cpu.sp = instruction.get_immediate_u16()?;
            },
            (&Operand::SP, &Operand::RegisterPair(h, l)) => {
                cpu.sp = cpu.read_reg_short(h, l);
            }
            _ => {
                // TODO: Add error here
                println!("UNEXPECTED OPERANDS {} {}", dst, src);
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
    fn execute_ld() {
        execute_all(Mnemonic::LD);
    }
}
