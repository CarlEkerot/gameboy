pub struct Memory {
    mem: [u8; 32000]
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            mem: [0; 32000]
        }
    }

    pub fn store(&mut self, addr: usize, value: u8) {
        self.mem[addr] = value;
    }

    pub fn load(&self, addr: usize) -> u8 {
        self.mem[addr]
    }
}
