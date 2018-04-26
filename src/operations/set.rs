use cpu::CPU;
use instructions::Instruction;
use definition::Operand;
use errors::*;
use operations::Execute;

pub struct Set;

impl Execute for Set {
    fn execute(instruction: &Instruction, cpu: &mut CPU) -> Result<()> {

        let bit = instruction.get_operand(0)?;
        let src = instruction.get_operand(1)?;

        match (src, bit) {
            (&Operand::Register(r), &Operand::Bit(b)) => {
                cpu.reg[r] |= 1u8 << b;
            },
            (&Operand::RegisterPairAddr(h, l), &Operand::Bit(b)) => {
                let addr = cpu.read_reg_addr(h, l);
                let val = cpu.ram.load(addr) | 1u8 << b;
                cpu.ram.store(addr, val);
            },
            _ => {
                // TODO: Add error here
                println!("UNEXPECTED OPERAND {}", src);
            },
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use test_helpers::execute_instruction;
    use cpu::CPU;
    use memory::Memory;
    use constants::*;

    #[test]
    fn test_set_reg() {
        let reg_codes: [(u16, usize); 7] = [
            (0xcbc7, REG_A),
            (0xcbc0, REG_B),
            (0xcbc1, REG_C),
            (0xcbc2, REG_D),
            (0xcbc3, REG_E),
            (0xcbc4, REG_H),
            (0xcbc5, REG_L),
        ];

        for bit in 0..8 {
            for &(c, r) in reg_codes.iter() {
                let mut mem = Memory::default();
                let mut cpu = CPU::new(mem);
                execute_instruction(&mut cpu, c + 8 * bit, None);
                assert_eq!(cpu.reg[r], 1 << bit);
            }
        }
    }

    #[test]
    fn test_set_regpair_addr() {
        for bit in 0..8 {
            let mut mem = Memory::default();
            let mut cpu = CPU::new(mem);
            cpu.reg[REG_H] = 0xff;
            cpu.reg[REG_L] = 0x22;
            execute_instruction(&mut cpu, 0xcbc6 + 8 * bit, None);
            assert_eq!(cpu.ram.load(0xff22), 1 << bit);
        }
    }
}
