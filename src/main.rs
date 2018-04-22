extern crate gameboy;

use gameboy::memory::Memory;
use gameboy::cpu::CPU;
use std::fs::File;
use std::io::{stdin, Read};


fn main() {
    let mut mem = Memory::default();
    let mut rom = File::open("/home/kalle/temp/boot.gb").unwrap();
    let bytes_read = mem.load_rom(&mut rom).unwrap();
    assert_eq!(bytes_read, 256);

    let mut cpu = CPU::new(mem);

    loop {
        if (cpu.pc >= 0xc) {
            println!("CPU State before: {:?}", cpu);
        }
        let instruction = cpu.execute_next();
        if (cpu.pc >= 0xc) {
            println!("CPU State after: {:?}", cpu);

            let mut buffer = String::new();
            stdin().read_line(&mut buffer).unwrap();
        }
    }
}
