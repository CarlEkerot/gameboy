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
    use test_helpers::{execute_all, execute_instruction};
    use definition::Mnemonic;
    use cpu::CPU;
    use memory::Memory;
    use constants::*;

    #[test]
    fn execute_push() {
        execute_all(Mnemonic::PUSH);
    }

    #[test]
    fn test_push_regpair_to_stack() {
        let pairs: [(u16, usize, usize); 4] = [
            (0xf5, REG_A, REG_F),
            (0xc5, REG_B, REG_C),
            (0xd5, REG_D, REG_E),
            (0xe5, REG_H, REG_L),
        ];

        for &(c, h, l) in pairs.iter() {
            let mut mem = Memory::default();
            let mut cpu = CPU::new(mem);
            cpu.sp = 0xff22;
            cpu.reg[h] = 0xaa;
            cpu.reg[l] = 0xbb;
            execute_instruction(&mut cpu, c, None);
            assert_eq!(cpu.ram.load(0xff22), 0xbb);
            assert_eq!(cpu.ram.load(0xff21), 0xaa);
            assert_eq!(cpu.sp, 0xff20);
        }
    }
}
