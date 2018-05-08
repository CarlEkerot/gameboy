use cpu::CPU;
use instructions::Instruction;
use errors::*;
use operations::Execute;

pub struct ReturnEnableInterrupts;

impl Execute for ReturnEnableInterrupts {
    fn execute(_instruction: &Instruction, cpu: &mut CPU) -> Result<()> {
        let hi = cpu.stack_pop() as u16;
        let lo = cpu.stack_pop() as u16;
        cpu.pc = (hi << 8) | lo;
        cpu.enable_interrupts();

        // Accommodate for next inc of program counter
        cpu.pc = cpu.pc.wrapping_sub(_instruction.definition.length as u16);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use test_helpers::{execute_all, execute_instruction, test_cpu};
    use definition::Mnemonic;

    #[test]
    fn execute_reti() {
        execute_all(Mnemonic::RETI);
    }

    /*
    #[test]
    fn test_ret() {
        let mut cpu = test_cpu();
        let mut mem = cpu.mem.borrow();
        cpu.sp = 0x1122;
        cpu.disable_interrupts();
        cpu.stack_push(0x22);
        cpu.stack_push(0xff);
        execute_instruction(&mut cpu, 0xd9, None);
        assert_eq!(cpu.pc, 0xff22);
        assert_eq!(cpu.sp, 0x1122);
        assert_eq!(mem.interrupts, true);
    }
    */
}
