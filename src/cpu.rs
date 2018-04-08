// http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf
use memory::Memory;

pub struct CPU {
    regs: [u8; 8],
    sp: u16,
    pc: u16,
    flag: u8,
    ram: Memory,
}

impl CPU {
    pub fn new(ram: Memory) -> CPU {
        CPU {
            regs: [0; 8],
            sp: 0xfffe,
            pc: 0x100,
            flag: 0,
            ram: ram,
        }
    }
}