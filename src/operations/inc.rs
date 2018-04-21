use cpu::CPU;
use instructions::Instruction;
use definition::Operand;
use errors::*;
use constants::*;
use operations::Execute;

pub struct Increase;

impl Execute for Increase {
    fn execute(instruction: &Instruction, cpu: &mut CPU) -> Result<()> {
        let dst = instruction.get_operand(0)?;

        match *dst {
            Operand::Register(r) => {
                let val = cpu.reg[r] + 1;
                cpu.reg[r] = val;
                cpu.set_half_carry(val as usize, 1);
                cpu.clear_flag(FLAG_N);
                cpu.flag_cond(FLAG_Z, val == 0)
            },
            Operand::RegisterPair(h, l) => {
                let val = cpu.read_reg_short(h, l) + 1;
                cpu.store_reg_short(h, l, val);
            },
            Operand::SP => {
                cpu.sp += 1;
            },
            Operand::RegisterPairAddr(h, l) => {
                let addr = cpu.read_reg_addr(h, l);
                let val = cpu.ram.load(addr);
                cpu.ram.store(addr, val + 1);
            },
            _ => {
                println!("UNEXPECTED OPERANDS IN INC");
            }
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use test_helpers::execute_all;
    use definition::Mnemonic;

    #[test]
    fn execute_incs() {
        execute_all(Mnemonic::INC);
    }
}
