use constants::*;
use cpu::CPU;
use std::fs::File;
use memory::Memory;
use std::time::SystemTime;
use std::time::Duration;
use std::thread;

pub struct Emulator<'a> {
    cpu: CPU,
    pub rom: &'a File,
}

impl<'a> Emulator<'a> {
    pub fn new(rom: &'a mut File) -> Self {
        let mut mem = Memory::default();
        let bytes_read = mem.load_rom(rom).unwrap();
        println!("Loaded {} byte rom", bytes_read);

        Emulator {
            cpu: CPU::new(mem),
            rom,
        }
    }

    fn update(&mut self) {
        let cycles_per_frame = CLOCK_SPEED / FRAME_RATE;
        let mut cycle_count = 0;

        while cycle_count < cycles_per_frame {
            let instruction = self.cpu.execute_next();
            let cycles = instruction.definition.cycles;
            cycle_count += cycles[0];
        }
        // self.lcd.update_frame();
    }

    pub fn run(&mut self) {
        for _ in 0..10 {
            // Run for 10 seconds
            for _ in 0..60 {
                let start = SystemTime::now();
                self.update();
                let dur = start.elapsed().unwrap();
                let diff = Duration::from_micros(16667).checked_sub(dur)
                    .unwrap_or_else(|| Duration::new(0, 0));
                thread::sleep(diff);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_rom() {
        let mut rom = File::open("/home/kalle/temp/boot.gb").unwrap();
        let mut emu = Emulator::new(&mut rom);
        emu.run();
    }
}