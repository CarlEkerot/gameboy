use constants::*;
use errors::*;
use std::fmt;
use std::io::Read;
use std::fs::File;

const DEFAULT_RAM: usize = 0x10000; // 64 kB

pub struct Memory {
    size: usize,
    mem: [u8; DEFAULT_RAM],
}

impl Memory {
    pub fn new(size: usize) -> Memory {
        // TODO: Set actual size
        Memory {
            size,
            mem: [0u8; DEFAULT_RAM],
        }
    }

    pub fn store(&mut self, addr: usize, value: u8) {
        assert!(addr < self.size,
                "Attempt to store outside of memory bound. {:04x} > {:04x}",
                addr, self.size);
        self.mem[addr] = match addr {
            MREG_DIV => 0,
            _ => value,
        };
    }

    pub fn load(&self, addr: usize) -> u8 {
        assert!(addr < self.size,
                "Attempt to load outside of memory bound. {:04x} > {:04x}",
                addr, self.size);
        self.mem[addr]
    }

    pub fn set_bit(&mut self, addr: usize, bit: usize) {
        assert!(bit < BYTE, "Attempt to set bit outside of bounds: {}", bit);
        self.mem[addr] |= 1u8 << bit;
    }

    pub fn clear_bit(&mut self, addr: usize, bit: usize) {
        assert!(bit < BYTE, "Attempt to clear bit outside of bounds: {}", bit);
        self.mem[addr] &= !(1u8 << bit);
    }

    pub fn is_set(&self, addr: usize, bit: usize) -> bool {
        assert!(bit < BYTE, "Attempt to read bit outside of bounds: {}", bit);
        self.mem[addr] & (1u8 << bit) != 0
    }

    pub fn load_rom(&mut self, rom: &mut File) -> Result<usize> {
        rom.read(&mut self.mem).chain_err(|| "Failed to read rom")
    }

    pub fn clear(&mut self) {
        self.mem = [0u8; DEFAULT_RAM];
    }
}

impl Default for Memory {
    fn default() -> Memory {
        Memory::new(DEFAULT_RAM)
    }
}

impl fmt::Debug for Memory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Memory[{}]", self.size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_set_register() {
        let mut mem = Memory::default();
        mem.set_bit(MREG_LCDC, 7);
        assert!(mem.is_set(MREG_LCDC, 7));
    }

    #[test]
    #[should_panic]
    fn test_set_bit_out_of_bounds() {
        let mut mem = Memory::default();
        mem.set_bit(MREG_LCDC, 8);
    }

    #[test]
    fn test_load_rom() {
        let mut mem = Memory::default();
        let mut rom = File::open("/home/kalle/temp/boot.gb").unwrap();
        let bytes_read = mem.load_rom(&mut rom).unwrap();
        assert_eq!(bytes_read, 256);

        assert_eq!(mem.load(0x00), 0x31);
        assert_eq!(mem.load(0x01), 0xfe);
        assert_eq!(mem.load(0x02), 0xff);
    }

    #[test]
    fn test_write_to_reset() {
        let mut mem = Memory::default();
        mem.mem[MREG_DIV] = 0xab;
        mem.store(MREG_DIV, 0xbb);

        assert_eq!(mem.load(MREG_DIV), 0);
    }
}
