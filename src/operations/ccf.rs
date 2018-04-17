use cpu::CPU;
use instructions::Instruction;
use errors::*;
use constants::*;
use operations::Execute;

pub struct ComplementCarryFlag;

impl Execute for ComplementCarryFlag {
    fn execute(_instruction: &Instruction, cpu: &mut CPU) -> Result<()> {
        let is_carry = cpu.flag_is_set(FLAG_C);
        cpu.clear_flag(FLAG_N);
        cpu.clear_flag(FLAG_H);
        cpu.flag_cond(FLAG_C, !is_carry);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use test_helpers::execute_all;
    use instructions::Mnemonic;

    #[test]
    fn execute_ccf() {
        execute_all(Mnemonic::CCF);
    }
}
