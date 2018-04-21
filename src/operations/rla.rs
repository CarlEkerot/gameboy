use cpu::CPU;
use instructions::Instruction;
use errors::*;
use constants::*;
use operations::Execute;

pub struct RotateALeft;

impl Execute for RotateALeft {
    fn execute(_instruction: &Instruction, cpu: &mut CPU) -> Result<()> {
        let val = cpu.reg[REG_A];
        let msb = val >> 7;
        let mut res = (val << 1) & 0xff;

        if cpu.flag_is_set(FLAG_C) {
            res |= 1;
        }
        cpu.reg[REG_A] = res;

        cpu.flag_cond(FLAG_Z, res == 0);
        cpu.clear_flag(FLAG_N);
        cpu.clear_flag(FLAG_H);
        cpu.flag_cond(FLAG_C, msb == 1);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use test_helpers::execute_all;
    use definition::Mnemonic;

    #[test]
    fn execute_rla() {
        execute_all(Mnemonic::RLA);
    }
}
