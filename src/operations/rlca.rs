use cpu::CPU;
use instructions::Instruction;
use errors::*;
use constants::*;
use operations::Execute;

pub struct RotateALeftCarry;

impl Execute for RotateALeftCarry {
    fn execute(instruction: &Instruction, cpu: &mut CPU) -> Result<()> {
        let val = cpu.reg[REG_A];
        let msb = val >> 7;
        cpu.reg[REG_A] = ((val << 1) & 0xff) | msb;

        // TODO: Set if result is zero?
        cpu.clear_flag(FLAG_Z);
        cpu.clear_flag(FLAG_N);
        cpu.clear_flag(FLAG_H);
        if msb == 1 {
            cpu.set_flag(FLAG_C);
        } else {
            cpu.clear_flag(FLAG_C);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use test_helpers::execute_all;
    use instructions::Mnemonic;

    #[test]
    fn execute_rlca() {
        execute_all(Mnemonic::RLCA);
    }
}
