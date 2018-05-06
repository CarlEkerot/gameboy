use cpu::CPU;
use instructions::Instruction;
use definition::Operand;
use errors::*;
use constants::*;
use operations::Execute;

pub struct Load;

impl Execute for Load {
    fn execute(instruction: &Instruction, cpu: &mut CPU) -> Result<()> {
        let dst = instruction.get_operand(0)?;
        let src = instruction.get_operand(1)?;

        match (dst, src) {
            (&Operand::Register(r), &Operand::Immediate(BYTE)) => {
                cpu.reg[r] = instruction.get_immediate_u8()?;
            },
            (&Operand::Register(r1), &Operand::Register(r2)) => {
                cpu.reg[r1] = cpu.reg[r2];
            },
            (&Operand::Register(r), &Operand::Address(SHORT)) => {
                let addr = instruction.get_immediate_usize()?;
                cpu.reg[r] = cpu.ram.load(addr);
            },
            (&Operand::Register(r), &Operand::RegisterPairAddr(h, l)) => {
                let addr = cpu.read_reg_addr(h, l);
                cpu.reg[r] = cpu.ram.load(addr);
            },
            (&Operand::Register(r1), &Operand::RegisterAddr(r2)) => {
                let offset = cpu.reg[r2] as usize;
                let addr = OFFSET_BASE + offset;
                cpu.reg[r1] = cpu.ram.load(addr);
            },
            (&Operand::RegisterPair(h, l), &Operand::Immediate(SHORT)) => {
                cpu.store_reg_short(h, l, instruction.get_immediate_u16()?);
            },
            (&Operand::RegisterPair(h, l), &Operand::SPOffset(BYTE)) => {
                let offset = instruction.get_immediate_i16()?;
                let sp = cpu.sp;
                let val = sp as i16 + offset;

                cpu.store_reg_short(h, l, val as u16);

                cpu.clear_flag(FLAG_Z);
                cpu.clear_flag(FLAG_N);

                // TODO: Double check these. _Really_ Unsafe arithmetic.
                cpu.set_half_carry(sp as usize, offset as usize);
                cpu.set_carry(sp as usize, offset as usize);
            },
            (&Operand::RegisterAddr(r1), &Operand::Register(r2)) => {
                let offset = cpu.reg[r1] as usize;
                let addr = OFFSET_BASE + offset;
                cpu.ram.store(addr, cpu.reg[r2]);
            },
            (&Operand::RegisterPairAddr(h, l), &Operand::Register(r)) => {
                let addr = cpu.read_reg_addr(h, l);
                cpu.ram.store(addr, cpu.reg[r]);
            },
            (&Operand::RegisterPairAddr(h, l), &Operand::Immediate(BYTE)) => {
                let addr = cpu.read_reg_addr(h, l);
                let val = instruction.get_immediate_u8()?;
                cpu.ram.store(addr, val);
            },
            (&Operand::Address(SHORT), &Operand::SP) => {
                // Stores two bytes in memory
                let addr = instruction.get_immediate_usize()?;
                cpu.ram.store(addr, (cpu.sp & 0xff) as u8);
                cpu.ram.store(addr + 1, (cpu.sp >> 8) as u8);
            },
            (&Operand::Address(SHORT), &Operand::Register(r)) => {
                // Stores two bytes in memory
                let addr = instruction.get_immediate_usize()?;
                cpu.ram.store(addr, cpu.reg[r]);
            },
            (&Operand::SP, &Operand::Immediate(SHORT)) => {
                cpu.sp = instruction.get_immediate_u16()?;
            },
            (&Operand::SP, &Operand::RegisterPair(h, l)) => {
                cpu.sp = cpu.read_reg_short(h, l);
            }
            _ => {
                // TODO: Add error here
                println!("UNEXPECTED OPERANDS {} {}", dst, src);
            },
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
    fn execute_ld() {
        execute_all(Mnemonic::LD);
    }

    #[test]
    fn test_ld_immediate() {
        let reg_codes: [(u16, usize); 7] = [
            (0x3e, REG_A),
            (0x06, REG_B),
            (0x0e, REG_C),
            (0x16, REG_D),
            (0x1e, REG_E),
            (0x26, REG_H),
            (0x2e, REG_L),
        ];
        for &(c, r) in reg_codes.iter() {
            let mut mem = Memory::default();
            let mut cpu = CPU::new(mem);
            execute_instruction(&mut cpu, c, Some(0xab));
            assert_eq!(cpu.reg[r], 0xab);
        }
    }

    #[test]
    fn test_ld_reg_reg() {
        let reg_codes: [(u16, usize, usize); 49] = [
            (0x7f, REG_A, REG_A),
            (0x78, REG_A, REG_B),
            (0x79, REG_A, REG_C),
            (0x7a, REG_A, REG_D),
            (0x7b, REG_A, REG_E),
            (0x7c, REG_A, REG_H),
            (0x7d, REG_A, REG_L),
            (0x47, REG_B, REG_A),
            (0x40, REG_B, REG_B),
            (0x41, REG_B, REG_C),
            (0x42, REG_B, REG_D),
            (0x43, REG_B, REG_E),
            (0x44, REG_B, REG_H),
            (0x45, REG_B, REG_L),
            (0x4f, REG_C, REG_A),
            (0x48, REG_C, REG_B),
            (0x49, REG_C, REG_C),
            (0x4a, REG_C, REG_D),
            (0x4b, REG_C, REG_E),
            (0x4c, REG_C, REG_H),
            (0x4d, REG_C, REG_L),
            (0x57, REG_D, REG_A),
            (0x50, REG_D, REG_B),
            (0x51, REG_D, REG_C),
            (0x52, REG_D, REG_D),
            (0x53, REG_D, REG_E),
            (0x54, REG_D, REG_H),
            (0x55, REG_D, REG_L),
            (0x5f, REG_E, REG_A),
            (0x58, REG_E, REG_B),
            (0x59, REG_E, REG_C),
            (0x5a, REG_E, REG_D),
            (0x5b, REG_E, REG_E),
            (0x5c, REG_E, REG_H),
            (0x5d, REG_E, REG_L),
            (0x67, REG_H, REG_A),
            (0x60, REG_H, REG_B),
            (0x61, REG_H, REG_C),
            (0x62, REG_H, REG_D),
            (0x63, REG_H, REG_E),
            (0x64, REG_H, REG_H),
            (0x65, REG_H, REG_L),
            (0x6f, REG_L, REG_A),
            (0x68, REG_L, REG_B),
            (0x69, REG_L, REG_C),
            (0x6a, REG_L, REG_D),
            (0x6b, REG_L, REG_E),
            (0x6c, REG_L, REG_H),
            (0x6d, REG_L, REG_L),
        ];
        for &(c, dst, src) in reg_codes.iter() {
            let mut mem = Memory::default();
            let mut cpu = CPU::new(mem);
            cpu.reg[src] = 0xab;
            execute_instruction(&mut cpu, c, None);
            assert_eq!(cpu.reg[dst], 0xab);
        }
    }

    #[test]
    fn test_ld_hl_addr_to_reg() {
        let reg_codes: [(u16, usize); 7] = [
            (0x7e, REG_A),
            (0x46, REG_B),
            (0x4e, REG_C),
            (0x56, REG_D),
            (0x5e, REG_E),
            (0x66, REG_H),
            (0x6e, REG_L),
        ];

        for &(c, r) in reg_codes.iter() {
            let mut mem = Memory::default();
            mem.store(0xff22, 0xab);
            let mut cpu = CPU::new(mem);
            cpu.reg[REG_H] = 0xff;
            cpu.reg[REG_L] = 0x22;
            execute_instruction(&mut cpu, c, None);
            assert_eq!(cpu.reg[r], 0xab);
        }
    }

    #[test]
    fn test_ld_reg_to_regpair_addr() {
        let reg_codes: [(u16, usize); 6] = [
            (0x70, REG_B),
            (0x71, REG_C),
            (0x72, REG_D),
            (0x73, REG_E),
            (0x74, REG_H),
            (0x75, REG_L),
        ];

        for &(c, r) in reg_codes.iter() {
            let mut mem = Memory::default();
            let mut cpu = CPU::new(mem);
            let ex = match r {
                REG_H => 0xff,
                REG_L => 0x22,
                _ => 0xab,
            };
            cpu.reg[r] = ex;
            cpu.reg[REG_H] = 0xff;
            cpu.reg[REG_L] = 0x22;
            execute_instruction(&mut cpu, c, None);
            assert_eq!(cpu.ram.load(0xff22), ex);
        }
    }

    #[test]
    fn test_ld_byte_to_regpair_addr() {
        let mem = Memory::default();
        let mut cpu = CPU::new(mem);
        cpu.reg[REG_H] = 0xff;
        cpu.reg[REG_L] = 0x22;
        execute_instruction(&mut cpu, 0x36, Some(0xab));
        assert_eq!(cpu.ram.load(0xff22), 0xab);
    }

    #[test]
    fn test_ld_regpair_addr_to_a() {
        let pairs: [(u16, usize, usize); 2] = [
            (0x0a, REG_B, REG_C),
            (0x1a, REG_D, REG_E),
        ];

        for &(c, h, l) in pairs.iter() {
            let mut mem = Memory::default();
            mem.store(0xff22, 0xab);
            let mut cpu = CPU::new(mem);
            cpu.reg[h] = 0xff;
            cpu.reg[l] = 0x22;
            execute_instruction(&mut cpu, c, None);
            assert_eq!(cpu.reg[REG_A], 0xab);
        }
    }

    #[test]
    fn test_ld_immediate_addr_to_a() {
        let mut mem = Memory::default();
        mem.store(0xff22, 0xab);
        let mut cpu = CPU::new(mem);
        execute_instruction(&mut cpu, 0xfa, Some(0xff22));
        assert_eq!(cpu.reg[REG_A], 0xab);
    }

    #[test]
    fn test_ld_a_to_regpair() {
        let pairs: [(u16, usize, usize); 3] = [
            (0x02, REG_B, REG_C),
            (0x12, REG_D, REG_E),
            (0x77, REG_H, REG_L),
        ];

        for &(c, h, l) in pairs.iter() {
            let mut mem = Memory::default();
            let mut cpu = CPU::new(mem);
            cpu.reg[h] = 0xff;
            cpu.reg[l] = 0x22;
            cpu.reg[REG_A] = 0xab;
            execute_instruction(&mut cpu, c, None);
            assert_eq!(cpu.ram.load(0xff22), 0xab);
        }
    }

    #[test]
    fn test_ld_a_to_immediate_addr() {
        let mem = Memory::default();
        let mut cpu = CPU::new(mem);
        cpu.reg[REG_A] = 0xab;
        execute_instruction(&mut cpu, 0xea, Some(0xff22));
        assert_eq!(cpu.ram.load(0xff22), 0xab);
    }

    #[test]
    fn test_ld_c_addr_offset_to_a() {
        let mut mem = Memory::default();
        mem.store(0xff22, 0xab);
        let mut cpu = CPU::new(mem);
        cpu.reg[REG_C] = 0x22;
        execute_instruction(&mut cpu, 0xf2, None);
        assert_eq!(cpu.reg[REG_A], 0xab);
    }

    #[test]
    fn test_ld_a_to_c_addr_offset() {
        let mem = Memory::default();
        let mut cpu = CPU::new(mem);
        cpu.reg[REG_A] = 0xab;
        cpu.reg[REG_C] = 0x22;
        execute_instruction(&mut cpu, 0xe2, None);
        assert_eq!(cpu.ram.load(0xff22), 0xab);
    }

    #[test]
    fn test_ld_short_to_regpair() {
        let pairs: [(u16, usize, usize); 3] = [
            (0x01, REG_B, REG_C),
            (0x11, REG_D, REG_E),
            (0x21, REG_H, REG_L),
        ];

        for &(c, h, l) in pairs.iter() {
            let mut mem = Memory::default();
            let mut cpu = CPU::new(mem);
            execute_instruction(&mut cpu, c, Some(0xaabb));
            assert_eq!(cpu.reg[h], 0xaa);
            assert_eq!(cpu.reg[l], 0xbb);
        }
    }

    #[test]
    fn test_ld_short_to_sp() {
        let mem = Memory::default();
        let mut cpu = CPU::new(mem);
        execute_instruction(&mut cpu, 0x31, Some(0xaabb));
        assert_eq!(cpu.sp, 0xaabb);
    }

    #[test]
    fn test_ld_hl_to_sp() {
        let mem = Memory::default();
        let mut cpu = CPU::new(mem);
        cpu.reg[REG_H] = 0xaa;
        cpu.reg[REG_L] = 0xbb;
        execute_instruction(&mut cpu, 0xf9, None);
        assert_eq!(cpu.sp, 0xaabb);
    }

    #[test]
    fn test_ld_sp_offset_to_hl() {
        let mem = Memory::default();
        let mut cpu = CPU::new(mem);
        cpu.sp = 0xff00;
        execute_instruction(&mut cpu, 0xf8, Some(0x22));
        assert_eq!(cpu.reg[REG_H], 0xff);
        assert_eq!(cpu.reg[REG_L], 0x22);
    }

    #[test]
    fn test_ld_sp_to_addr() {
        let mem = Memory::default();
        let mut cpu = CPU::new(mem);
        cpu.sp = 0xaabb;
        execute_instruction(&mut cpu, 0x08, Some(0xff22));
        assert_eq!(cpu.ram.load(0xff22), 0xbb);
        assert_eq!(cpu.ram.load(0xff23), 0xaa);
    }
}
