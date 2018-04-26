extern crate gameboy;

use std::fs::File;
use std::env::args;
use std::process;

use gameboy::debugger::Debugger;

fn main() {
    let args: Vec<_> = args().collect();
    if args.len() != 2 {
        println!("Usage: ./debugger [path to rom-file]");
        process::exit(1);
    }

    let rom_path = &args[1];
    let mut rom = File::open(rom_path).unwrap();
    let mut d = Debugger::new(&mut rom);

    d.start();
}
