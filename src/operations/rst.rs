use cpu::CPU;
use instructions::Instruction;
use definition::Operand;
use errors::*;
use operations::Execute;

pub struct Restart;

impl Execute for Restart {
    fn execute(instruction: &Instruction, cpu: &mut CPU) -> Result<()> {
        let offset = instruction.get_operand(0)?;

        if let Operand::RSTOffset(o) = *offset {
            let addr = cpu.pc;
            cpu.stack_push((addr & 0xff) as u8);
            cpu.stack_push((addr >> 8) as u8);
            cpu.pc = o as u16;
        } else {
            println!("UNEXPECTED OPERAND {}", offset);
        }

        // Accommodate for next inc of program counter
        cpu.pc = cpu.pc.wrapping_sub(instruction.definition.length as u16);


        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use test_helpers::{execute_all, execute_instruction, test_cpu};
    use definition::Mnemonic;

    #[test]
    fn execute_rst() {
        execute_all(Mnemonic::RST);
    }

    #[test]
    fn test_rst() {
        let rst_offset_codes: [(u16, u16); 8] = [
            (0xc7, 0x00),
            (0xcf, 0x08),
            (0xd7, 0x10),
            (0xdf, 0x18),
            (0xe7, 0x20),
            (0xef, 0x28),
            (0xf7, 0x30),
            (0xff, 0x38),
        ];

        for &(c, o) in rst_offset_codes.iter() {
            let mut cpu = test_cpu();
            cpu.pc = 0x2233;
            cpu.sp = 0x1122;
            execute_instruction(&mut cpu, c, None);
            assert_eq!(cpu.pc, o);
            assert_eq!(cpu.load_mem(0x1121), 0x33);
            assert_eq!(cpu.load_mem(0x1120), 0x22);
        }
    }
}
