use cpu::CPU;
use instructions::Instruction;
use definition::Operand;
use errors::*;
use operations::Execute;

pub struct Push;

impl Execute for Push {
    fn execute(instruction: &Instruction, cpu: &mut CPU) -> Result<()> {
        let src = instruction.get_operand(0)?;

        match *src {
            Operand::RegisterPair(h, l) => {
                let lo = cpu.reg[l];
                let hi = cpu.reg[h];
                cpu.stack_push(lo);
                cpu.stack_push(hi);
            },
            _ => {
                println!("UNEXPECTED OPERAND {}", src);
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
    fn execute_push() {
        execute_all(Mnemonic::PUSH);
    }
}
