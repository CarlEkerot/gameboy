// http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf
use memory::Memory;
use instructions::{Instruction, Mnemonic};
use errors::*;
use constants::*;
use operations::{Execute, Increase, Load, LoadIncrease, LoadDecrease};

// Allow dead code for now...
#[allow(dead_code)]
pub struct CPU {
    pub reg: [u8; 8],
    pub sp: u16,
    pc: u16,
    pub flag: u8,
    pub ram: Memory,
    cycles: usize,
}

impl CPU {
    pub fn new(ram: Memory) -> CPU {
        CPU {
            reg: [0; 8],
            sp: 0xfffe,
            pc: 0x100,
            flag: 0,
            ram,
            cycles: 0,
        }
    }

    pub fn execute(&mut self, instruction: &Instruction) -> Result<()> {
        // NOTE: When other cycle count?
        self.cycles += instruction.definition.cycles[0];
        match instruction.definition.mnemonic {
            Mnemonic::LD => Load::execute(instruction, self),
            Mnemonic::LDI => LoadIncrease::execute(instruction, self),
            Mnemonic::LDD => LoadDecrease::execute(instruction, self),
            Mnemonic::INC => Increase::execute(instruction, self),
            _ => Ok(())
        }
    }

    pub fn set_flag(&mut self, flag: u8) {
        self.flag |= flag;
    }

    pub fn clear_flag(&mut self, flag: u8) {
        self.flag &= !flag;
    }

    pub fn is_carry(a: usize, b: usize) -> bool {
        (((a & 0xff) + (b & 0xff)) & 0x100) == 0x100
    }

    pub fn is_half_carry(a: usize, b: usize) -> bool {
        (((a & 0xf) + (b & 0xf)) & 0x10) == 0x10
    }

    pub fn set_carry(&mut self, a: usize, b: usize) {
        if CPU::is_carry(a, b) {
            self.set_flag(FLAG_C);
        } else {
            self.clear_flag(FLAG_C);
        }
    }

    pub fn set_half_carry(&mut self, a: usize, b: usize) {
        if CPU::is_half_carry(a, b) {
            self.set_flag(FLAG_H);
        } else {
            self.clear_flag(FLAG_H);
        }
    }
}
