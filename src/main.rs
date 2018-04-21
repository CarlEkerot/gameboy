extern crate gameboy;

use gameboy::memory::Memory;
use gameboy::cpu::CPU;

fn main() {
    let mut m = Memory::default();
    let mut cpu = CPU::new(m);
}
