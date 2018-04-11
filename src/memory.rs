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
}

impl Default for Memory {
    fn default() -> Memory {
        Memory::new(DEFAULT_RAM)
    }
}
