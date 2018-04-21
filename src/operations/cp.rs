use cpu::CPU;
use instructions::Instruction;
use definition::Operand;
use errors::*;
use constants::*;
use operations::Execute;

pub struct Compare;

impl Execute for Compare {
    fn execute(instruction: &Instruction, cpu: &mut CPU) -> Result<()> {
        let src = instruction.get_operand(0)?;

        let a = cpu.reg[REG_A];
        let b = match *src {
            Operand::Register(r) => cpu.reg[r],
            Operand::RegisterPairAddr(h, l) => {
                let addr = cpu.read_reg_addr(h, l);
                cpu.ram.load(addr)
            },
            Operand::Immediate(BYTE) => instruction.get_immediate_u8()?,
            _ => {
                // TODO: Add error here
                println!("UNEXPECTED OPERAND {}", src);
                0
            },
        };

        cpu.flag_cond(FLAG_Z, a == b);
        cpu.set_flag(FLAG_N);
        cpu.set_half_carry(a as usize, b as usize);
        cpu.flag_cond(FLAG_C, a < b);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use test_helpers::execute_all;
    use definition::Mnemonic;

    #[test]
    fn execute_cps() {
        execute_all(Mnemonic::CP);
    }
}
