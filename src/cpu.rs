// http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf
use memory::Memory;
use instructions::Instruction;
use definition::{Mnemonic, ImmediateType};
use errors::*;
use constants::*;
use operations::*;
use instruction_set::get_definition;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum CPUState {
    Running,
    Halted,
    Stopped,
}

pub struct CPU {
    pub reg: [u8; 8],
    pub sp: u16,
    pub pc: u16,
    pub flag: u8,
    pub ram: Memory,
    pub cycles: usize,
    pub state: CPUState,
    pub interrupts: bool,
    // Timer components
    prev_count: u8,
    count: u8,
    timer_count: usize,
}

impl fmt::Debug for CPU {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, r#"
A: {:02x} F: {:02x} B: {:02x} C: {:02x}
D: {:02x} E: {:02x} H: {:02x} L: {:02x}
SP: {:04x} PC: {:04x} Flags: {:08b}"#,
               self.reg[0], self.reg[1], self.reg[2], self.reg[3],
               self.reg[4], self.reg[5], self.reg[6], self.reg[7],
               self.sp, self.pc, self.flag)
    }

}

impl CPU {
    pub fn new(ram: Memory) -> CPU {
        CPU {
            reg: [0; 8],
            sp: 0,
            pc: 0,
            flag: 0,
            ram,
            cycles: 0,
            state: CPUState::Running,
            interrupts: true,
            prev_count: 0,
            count: 0,
            timer_count: 0,
        }
    }

    pub fn execute(&mut self, instruction: &Instruction) -> Result<()> {
        let res = match instruction.definition.mnemonic {
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
        };
        // NOTE: When other cycle count?
        let instruction_cycles = instruction.definition.cycles[0];
        self.cycles += instruction_cycles;
        self.increase_timer(instruction_cycles);
        self.pc = self.pc.wrapping_add(instruction.definition.length as u16);
        res
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
        if cond {
            self.set_flag(flag);
        } else {
            self.clear_flag(flag);
        }
    }

    pub fn is_carry(a: usize, b: usize) -> bool {
        // TODO: Perhaps make sure we use u8 wrapping arithmetics
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
        self.sp = self.sp.wrapping_sub(1);
        self.ram.store(self.sp as usize, b);
    }

    pub fn stack_pop(&mut self) -> u8 {
        let val = self.ram.load(self.sp as usize);
        self.sp = self.sp.wrapping_add(1);
        val
    }

    fn next_instruction_byte(&self, offset: &mut usize) -> u8 {
        let b = self.ram.load((self.pc as usize) + *offset);
        *offset += 1;
        b
    }

    pub fn current_instruction(&self) -> Result<Instruction> {
        let mut offset: usize = 0;
        let first = self.next_instruction_byte(&mut offset) as u16;
        let opcode = match first {
            0xcb => {
                let second = self.next_instruction_byte(&mut offset) as u16;
                first << 8 | second
            },
            _ => first,
        };

        let definition = get_definition(opcode);

        let immediate = definition.immediate_size.as_ref().map(|i| match *i {
            ImmediateType::Byte => self.next_instruction_byte(&mut offset) as u16,
            ImmediateType::Short => {
                let lo = self.next_instruction_byte(&mut offset) as u16;
                let hi = self.next_instruction_byte(&mut offset) as u16;
                hi << 8 | lo
            },
        });

        Ok(Instruction {
            definition,
            immediate
        })
    }

    pub fn execute_next(&mut self) -> Instruction {
        let instruction = self.current_instruction().unwrap();
        self.execute(&instruction);
        instruction
    }

    pub fn set_register_flag(&mut self, reg_addr: usize, flag: u8) {
        let current = self.ram.load(reg_addr);
        self.ram.store(reg_addr, current | flag);
    }

    pub fn clear_register_flag(&mut self, reg_addr: usize, lag: u8) {
        let current = self.ram.load(reg_addr);
        self.ram.store(reg_addr, current & !flag);
    }

    pub fn increase_timer(&mut self, cycles: usize) {
        // Load current divider
        let tac = self.ram.load(MREG_TAC);

        // If timer not enabled, do nothing
        if (tac & 0b100) == 0 {
            return
        }

        cycles_per_tick = TIMER_CYCLES_PER_TICK[(tac & 0b11) as usize];

        let prev_count = self.ram.load(MREG_TIMA);
        self.timer_count += cycles;
        if self.timer_count >= cycles_per_tick {
            let count = prev_count.wrapping_add(1);
            self.ram.store(MREG_TIMA, count);
            self.timer_count %= cycles_per_tick;

            if self.count < prev_count {
                // Set overflow
                // TODO!

                // Set contents of TIMA to that of TMA
                let tma = self.ram.load(MREG_TMA);
                self.ram.store(MREG_TIMA, tma);
            }
        }
    }

    pub fn reset(&mut self) {
        self.reg = [0; 8];
        self.sp = 0;
        self.pc = 0;
        self.flag = 0;
        self.cycles = 0;
        self.state = CPUState::Running;
        self.enable_interrupts();
        self.ram.clear()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use definition::Operand;
    use memory::Memory;
    use std::fs::File;

    #[test]
    fn test_parse() {
        let mut mem = Memory::default();
        mem.store(0x100, 0xaf);

        let mut cpu = CPU::new(mem);
        let instruction = cpu.current_instruction().unwrap();

        assert_eq!(instruction.definition.mnemonic, Mnemonic::XOR);
        assert_eq!(instruction.definition.operands[0], Operand::Register(REG_A));
    }

    #[test]
    fn test_parse_with_immediate() {
        let mut mem = Memory::default();
        mem.store(0x100, 0x31);
        mem.store(0x101, 0xfe);
        mem.store(0x102, 0xff);

        let mut cpu = CPU::new(mem);
        let instruction = cpu.current_instruction().unwrap();

        assert_eq!(instruction.definition.mnemonic, Mnemonic::LD);
        assert_eq!(instruction.definition.operands[0], Operand::SP);
        assert_eq!(instruction.definition.operands[1], Operand::Immediate(SHORT));
        assert_eq!(instruction.immediate, Some(0xfffe));
    }

    #[test]
    fn test_parse_rom() {
        let mut mem = Memory::default();
        let mut rom = File::open("/home/kalle/temp/boot.gb").unwrap();
        let bytes_read = mem.load_rom(&mut rom).unwrap();
        assert_eq!(bytes_read, 256);

        let mut cpu = CPU::new(mem);

        let expected = [
            "LD SP, $fffe",
            "XOR A",
            "LD HL, $9fff",
            "LDD (HL), A",
            "BIT 7, H"
        ];

        for &e in expected.iter() {
            let instruction = cpu.current_instruction().unwrap();
            assert_eq!(instruction.to_string(), e);
            cpu.pc += instruction.definition.length as u16;
        }
    }

    #[test]
    fn test_stack() {
        let mut mem = Memory::default();
        let mut cpu = CPU::new(mem);
        cpu.sp = 0x1122;
        cpu.stack_push(0x12);
        let res = cpu.stack_pop();
        assert_eq!(res, 0x12);
    }

    #[test]
    fn test_timer() {
        for &cycles in TIMER_CYCLES_PER_TICK.iter() {
            let mut mem = Memory::default();
            let mut cpu = CPU::new(mem);
            cpu.cycles_per_tick = cycles;

            for _ in 0..(CLOCK_SPEED - 1) {
                cpu.increase_timer(1)
            }

            assert_eq!(cpu.ram.load(MREG_TIMA), 255);
            assert_eq!(cpu.timer_count, cycles - 1);
        }
    }

    #[test]
    fn test_timer_overflow() {
        let mut mem = Memory::default();
        let mut cpu = CPU::new(mem);
        let overflow_ticks = 256 * TIMER_CYCLES_PER_TICK[0];
        for _ in 0..overflow_ticks {
            cpu.increase_timer(1)
        }
        assert!(t.did_overflow());
    }
}
