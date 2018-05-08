use constants::*;
use interrupts::*;
use memory::Memory;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Timer {
    mem: Rc<RefCell<Memory>>,
    timer_count: usize,
    divider_count: u8,
}

impl Timer {
    pub fn new(mem: Rc<RefCell<Memory>>) -> Timer {
        Timer {
            mem,
            timer_count: 0,
            divider_count: 0,
        }
    }

    // Returns true if overflow
    fn increase_timer(&mut self, cycles: usize) {
        // Load current divider
        let mut mem = self.mem.borrow_mut();
        let tac = mem.load(MREG_TAC);

        // If timer not enabled, do nothing
        if (tac & 0b100) == 0 {
            return
        }

        let cycles_per_tick = TIMER_CYCLES_PER_TICK[(tac & 0b11) as usize];

        let prev_count = mem.load(MREG_TIMA);
        self.timer_count += cycles;
        if self.timer_count >= cycles_per_tick {
            let count = prev_count.wrapping_add(1);
            mem.store(MREG_TIMA, count);
            self.timer_count %= cycles_per_tick;

            if count < prev_count {
                // Set overflow
                mem.set_interrupt_flag(INTERRUPT_TIMER.flag);

                // Set contents of TIMA to that of TMA
                let tma = mem.load(MREG_TMA);
                mem.store(MREG_TIMA, tma);
            }
        }
    }

    fn increase_divider(&mut self, cycles: usize) {
        let prev_count = self.divider_count;
        self.divider_count =  self.divider_count.wrapping_add(cycles as u8);
        if self.divider_count < prev_count {
            let mut mem = self.mem.borrow_mut();
            let div = mem.load(MREG_DIV);
            mem.store(MREG_DIV, div.wrapping_add(1));
        }
    }

    pub fn increase(&mut self, cycles: usize) {
        self.increase_timer(cycles);
        self.increase_divider(cycles);
    }
}
