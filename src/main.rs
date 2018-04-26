extern crate gameboy;

use gameboy::memory::Memory;
use gameboy::cpu::CPU;
use std::fs::File;


fn main() {
    let mut mem = Memory::default();
    let mut rom = File::open("/home/kalle/temp/boot.gb").unwrap();
    let bytes_read = mem.load_rom(&mut rom).unwrap();
    assert_eq!(bytes_read, 256);

    let mut cpu = CPU::new(mem);

    loop {
        cpu.execute_next();
    }
}
