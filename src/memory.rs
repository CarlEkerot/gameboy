use constants::BYTE;

const DEFAULT_RAM: usize = 0x10000; // 64 kB

pub struct Memory {
    size: usize,
    mem: Vec<u8>,
}

impl Memory {
    pub fn new(size: usize) -> Memory {
        Memory {
            size,
            mem: vec![0u8; size],
        }
    }

    pub fn store(&mut self, addr: usize, value: u8) {
        assert!(addr < self.size,
                "Attempt to store outside of memory bound. {:04x} > {:04x}",
                addr, self.size);
        self.mem[addr] = value;
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
}

impl Default for Memory {
    fn default() -> Memory {
        Memory::new(DEFAULT_RAM)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use constants::*;

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
}
