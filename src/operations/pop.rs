use cpu::CPU;
use instructions::Instruction;
use definition::Operand;
use errors::*;
use operations::Execute;

pub struct Pop;

impl Execute for Pop {
    fn execute(instruction: &Instruction, cpu: &mut CPU) -> Result<()> {
        let src = instruction.get_operand(0)?;

        match *src {
            Operand::RegisterPair(h, l) => {
                cpu.reg[h] = cpu.stack_pop();
                cpu.reg[l] = cpu.stack_pop();
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
    use test_helpers::{execute_all, execute_instruction};
    use definition::Mnemonic;
    use cpu::CPU;
    use memory::Memory;
    use constants::*;

    #[test]
    fn execute_pop() {
        execute_all(Mnemonic::POP);
    }

    #[test]
    fn test_pop_regpair_from_stack() {
        let pairs: [(u16, usize, usize); 4] = [
            (0xf1, REG_A, REG_F),
            (0xc1, REG_B, REG_C),
            (0xd1, REG_D, REG_E),
            (0xe1, REG_H, REG_L),
        ];

        for &(c, h, l) in pairs.iter() {
            let mut mem = Memory::default();
            mem.store(0xff22, 0xaa);
            mem.store(0xff23, 0xbb);
            let mut cpu = CPU::new(mem);
            cpu.sp = 0xff22;
            execute_instruction(&mut cpu, c, None);
            assert_eq!(cpu.reg[h], 0xaa);
            assert_eq!(cpu.reg[l], 0xbb);
            assert_eq!(cpu.sp, 0xff24);
        }
    }
}
