extern crate gameboy;

use gameboy::memory::Memory;
use gameboy::cpu::CPU;

fn main() {
    let mut m = Memory::default();
    println!("0x10: {}", m.load(0x10));
    m.store(0x10, 10);
    println!("0x10: {}", m.load(0x10));
    let cpu = CPU::new(m);
}
