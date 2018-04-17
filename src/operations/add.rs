use cpu::CPU;
use instructions::{Instruction, Operand};
use errors::*;
use constants::*;
use operations::Execute;

pub struct Add;

impl Execute for Add {
    fn execute(instruction: &Instruction, cpu: &mut CPU) -> Result<()> {
        let dst = instruction.get_operand(0)?;
        let src = instruction.get_operand(1)?;

        match (dst, src) {
            (&Operand::Register(r1), &Operand::Register(r2)) => {
                let a = cpu.reg[r1];
                let b = cpu.reg[r2];
                let val = a + b;
                cpu.reg[r1] = val;

                cpu.flag_cond(FLAG_Z, val == 0);
                cpu.clear_flag(FLAG_N);
                cpu.set_half_carry(a as usize, b as usize);
                cpu.set_carry(a as usize, b as usize);
            },
            (&Operand::Register(r), &Operand::RegisterPairAddr(h, l)) => {
                let a = cpu.reg[r];
                let addr = cpu.read_reg_addr(h, l);
                let b = cpu.ram.load(addr);
                let val = a + b;
                cpu.reg[r] = val;

                cpu.flag_cond(FLAG_Z, val == 0);
                cpu.clear_flag(FLAG_N);
                cpu.set_half_carry(a as usize, b as usize);
                cpu.set_carry(a as usize, b as usize);
            },
            (&Operand::Register(r), &Operand::Immediate(BYTE)) => {
                let a = cpu.reg[r];
                let b = instruction.get_immediate_u8()?;
                let val = a + b;
                cpu.reg[r] = val;

                cpu.flag_cond(FLAG_Z, val == 0);
                cpu.clear_flag(FLAG_N);
                cpu.set_half_carry(a as usize, b as usize);
                cpu.set_carry(a as usize, b as usize);
            },
            (&Operand::RegisterPair(h1, l1), &Operand::RegisterPair(h2, l2)) => {
                let a = cpu.read_reg_short(h1, l1);
                let b = cpu.read_reg_short(h2, l2);
                cpu.store_reg_short(h1, l1, a + b);

                cpu.clear_flag(FLAG_N);
                cpu.set_half_carry(a as usize, b as usize);
                cpu.set_carry(a as usize, b as usize);
            },
            (&Operand::RegisterPair(h, l), &Operand::SP) => {
                let a = cpu.read_reg_short(h, l);
                let b = cpu.sp;
                cpu.store_reg_short(h, l, a + b);

                cpu.clear_flag(FLAG_N);
                cpu.set_half_carry(a as usize, b as usize);
                cpu.set_carry(a as usize, b as usize);
            },
            (&Operand::SP, &Operand::Offset(BYTE)) => {
                let a = cpu.sp;
                let b = instruction.get_immediate_i8()?;

                // TODO: Check this. Danger danger
                cpu.sp = (a as i32 + b as i32) as u16;
                cpu.clear_flag(FLAG_Z);
                cpu.clear_flag(FLAG_N);
            },
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
    use instructions::Mnemonic;

    #[test]
    fn execute_adds() {
        execute_all(Mnemonic::ADD);
    }
}
