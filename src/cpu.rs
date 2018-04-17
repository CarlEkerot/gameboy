// http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf
use memory::Memory;
use instructions::{Instruction, Mnemonic};
use errors::*;
use constants::*;
use operations::*;

pub enum CPUState {
    Running,
    Halted,
    Stopped,
}

// Allow dead code for now...
#[allow(dead_code)]
pub struct CPU {
    pub reg: [u8; 8],
    pub sp: u16,
    pub pc: u16,
    pub flag: u8,
    pub ram: Memory,
    cycles: usize,
    state: CPUState,
    interrupts: bool,
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
            state: CPUState::Running,
            interrupts: true,
        }
    }

    pub fn execute(&mut self, instruction: &Instruction) -> Result<()> {
        // NOTE: When other cycle count?
        self.cycles += instruction.definition.cycles[0];
        match instruction.definition.mnemonic {
            Mnemonic::ADC => AddCarry::execute(instruction, self),
            Mnemonic::ADD => Add::execute(instruction, self),
            Mnemonic::AND => And::execute(instruction, self),
            Mnemonic::BIT => Bit::execute(instruction, self),
            Mnemonic::CALL => Call::execute(instruction, self),
            Mnemonic::CCF => ComplementCarryFlag::execute(instruction, self),
            Mnemonic::CP => Compare::execute(instruction, self),
            Mnemonic::CPL => ComplementA::execute(instruction, self),
            Mnemonic::DAA => DecimalAdjustA::execute(instruction, self),
            Mnemonic::DEC => Decrease::execute(instruction, self),
            Mnemonic::DI => DisableInterrupts::execute(instruction, self),
            Mnemonic::EI => EnableInterrupts::execute(instruction, self),
            Mnemonic::HALT => Halt::execute(instruction, self),
            Mnemonic::INC => Increase::execute(instruction, self),
            Mnemonic::JP => Jump::execute(instruction, self),
            Mnemonic::JR => JumpRelative::execute(instruction, self),
            Mnemonic::LD => Load::execute(instruction, self),
            Mnemonic::LDD => LoadDecrease::execute(instruction, self),
            Mnemonic::LDH => LoadOffset::execute(instruction, self),
            Mnemonic::LDI => LoadIncrease::execute(instruction, self),
            Mnemonic::NOP => Nop::execute(instruction, self),
            Mnemonic::OR => Or::execute(instruction, self),
            Mnemonic::POP => Pop::execute(instruction, self),
            Mnemonic::PUSH => Push::execute(instruction, self),
            Mnemonic::RES => Reset::execute(instruction, self),
            Mnemonic::RET => Return::execute(instruction, self),
            Mnemonic::RETI => ReturnEnableInterrupts::execute(instruction, self),
            Mnemonic::RL => RotateLeft::execute(instruction, self),
            Mnemonic::RLA => RotateALeft::execute(instruction, self),
            Mnemonic::RLC => RotateLeftCarry::execute(instruction, self),
            Mnemonic::RLCA => RotateALeftCarry::execute(instruction, self),
            Mnemonic::RR => RotateRight::execute(instruction, self),
            Mnemonic::RRA => RotateARight::execute(instruction, self),
            Mnemonic::RRC => RotateRightCarry::execute(instruction, self),
            Mnemonic::RRCA => RotateARightCarry::execute(instruction, self),
            Mnemonic::RST => Restart::execute(instruction, self),
            Mnemonic::SBC => SubtractCarry::execute(instruction, self),
            Mnemonic::SCF => SetCarryFlag::execute(instruction, self),
            Mnemonic::SET => Set::execute(instruction, self),
            Mnemonic::SLA => ShiftLeftArithmetic::execute(instruction, self),
            Mnemonic::SRA => ShiftRightArithmetic::execute(instruction, self),
            Mnemonic::SRL => ShiftRightLogical::execute(instruction, self),
            Mnemonic::STOP => Stop::execute(instruction, self),
            Mnemonic::SUB => Subtract::execute(instruction, self),
            Mnemonic::SWAP => Swap::execute(instruction, self),
            Mnemonic::XOR => Xor::execute(instruction, self),
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

    pub fn flag_cond(&mut self, flag: u8, cond: bool) {
        if cond == true {
            self.set_flag(flag);
        } else {
            self.clear_flag(flag);
        }
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

    pub fn set_state(&mut self, state: CPUState) {
        self.state = state;
    }

    pub fn enable_interrupts(&mut self) {
        self.interrupts = true;
    }

    pub fn disable_interrupts(&mut self) {
        self.interrupts = false;
    }

    pub fn stack_push(&mut self, b: u8) {
        self.ram.store(self.sp as usize, b);
        self.sp -= 1;
    }

    pub fn stack_pop(&mut self) -> u8 {
        let val = self.ram.load(self.sp as usize);
        self.sp += 1;
        val
    }
}
