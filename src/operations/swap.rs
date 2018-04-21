use cpu::CPU;
use instructions::Instruction;
use definition::Operand;
use errors::*;
use constants::*;
use operations::Execute;

pub struct Swap;

impl Execute for Swap {
    fn execute(instruction: &Instruction, cpu: &mut CPU) -> Result<()> {
        let src = instruction.get_operand(0)?;

        let res = match *src {
            Operand::Register(r) => {
                let hi = cpu.reg[r] >> 4;
                let lo = cpu.reg[r] & 0xf;
                let res = (lo << 4) | hi;
                cpu.reg[r] = res;
                res
            },
            Operand::RegisterPairAddr(h, l) => {
                let addr = cpu.read_reg_addr(h, l);
                let val = cpu.ram.load(addr);
                let hi = val >> 4;
                let lo = val & 0xf;
                let res = (lo << 4) | hi;
                cpu.ram.store(addr, res);
                res
            },
            _ => {
                // TODO: Add error here
                println!("UNEXPECTED OPERAND {}", src);
                0
            },
        };
        cpu.flag_cond(FLAG_Z, res == 0);
        cpu.clear_flag(FLAG_N);
        cpu.clear_flag(FLAG_H);
        cpu.clear_flag(FLAG_C);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use test_helpers::execute_all;
    use definition::Mnemonic;

    #[test]
    fn execute_swap() {
        execute_all(Mnemonic::SWAP);
    }
}
