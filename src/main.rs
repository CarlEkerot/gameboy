extern crate gameboy;

use gameboy::emulator::Emulator;
use std::fs::File;

fn main() {
    let mut rom = File::open("/home/kalle/temp/boot.gb").unwrap();
    let mut emu = Emulator::new(&mut rom);
    emu.run();
}
