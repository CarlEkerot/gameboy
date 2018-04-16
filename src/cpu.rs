// http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf
use memory::Memory;
use instructions::{Instruction, Mnemonic};
use errors::*;
use constants::*;
use operations::*;

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
            Mnemonic::ADD => Add::execute(instruction, self),
            Mnemonic::LD => Load::execute(instruction, self),
            Mnemonic::LDI => LoadIncrease::execute(instruction, self),
            Mnemonic::LDD => LoadDecrease::execute(instruction, self),
            Mnemonic::INC => Increase::execute(instruction, self),
            Mnemonic::DEC => Decrease::execute(instruction, self),
            Mnemonic::NOP => Nop::execute(instruction, self),
            Mnemonic::RLA => RotateALeft::execute(instruction, self),
            Mnemonic::RLCA => RotateALeftCarry::execute(instruction, self),
            _ => Ok(())
        }
    }

    pub fn read_reg_addr(&self, h: usize, l: usize) -> usize {
        ((self.reg[h] as usize) << 8) | (self.reg[l] as usize)
    }

    pub fn read_reg_short(&self, h: usize, l: usize) -> u16 {
        ((self.reg[h] as u16) << 8) | (self.reg[l] as u16)
    }

    pub fn store_reg_short(&mut self, h: usize, l: usize, val: u16) {
        self.reg[h] = (val >> 8) as u8;
        self.reg[l] = (val & 0xff) as u8;
    }

    pub fn flag_is_set(&self, flag: u8) -> bool {
        self.flag & flag != 0
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
