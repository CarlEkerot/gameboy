extern crate gameboy;


use gameboy::memory::Memory;
use gameboy::cpu::CPU;

fn main() {
    let mut m = Memory::new();
    println!("0x10: {}", m.load(0x10));
    m.store(0x10, 300);
    println!("0x10: {}", m.load(0x10));
    let mut cpu = CPU::new(m);
}
