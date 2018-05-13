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
            mem.store_unchecked(MREG_DIV, div.wrapping_add(1));
        }
    }

    pub fn increase(&mut self, cycles: usize) {
        self.increase_timer(cycles);
        self.increase_divider(cycles);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cpu::CPU;
    use test_helpers::test_cpu;

    #[test]
    fn test_timer() {
        for i in 0..TIMER_CYCLES_PER_TICK.len() {
            let mem = Rc::new(RefCell::new(Memory::default()));
            let flag = (0b100 | i) as u8;
            mem.borrow_mut().store(MREG_TAC, flag);

            let mut timer = Timer::new(Rc::clone(&mem));

            let cycles = TIMER_CYCLES_PER_TICK[i];

            for _ in 0..(CLOCK_SPEED - 1) {
                timer.increase(1);
            }

            assert_eq!(mem.borrow().load(MREG_TIMA), 255);
            assert_eq!(timer.timer_count, cycles - 1);
        }
    }

    #[test]
    fn test_timer_overflow() {
        let mem = Rc::new(RefCell::new(Memory::default()));
        let mut timer = Timer::new(Rc::clone(&mem));
        let overflow_ticks = 256 * TIMER_CYCLES_PER_TICK[0];
        mem.borrow_mut().store(MREG_TAC, 0b100);
        for _ in 0..overflow_ticks {
            timer.increase(1);
        }

        let flag = INTERRUPT_TIMER.flag;
        let reg_value = mem.borrow().load(MREG_IF);
        assert_ne!(reg_value & flag, 0);
    }

    #[test]
    fn test_divider() {
        let mem = Rc::new(RefCell::new(Memory::default()));
        let mut timer = Timer::new(Rc::clone(&mem));

        for _ in 0..(256 * 10) {
            timer.increase_divider(1);
        }

        let div = mem.borrow().load(MREG_DIV);
        assert_eq!(div, 10);
    }

    #[test]
    fn test_divider_overflow() {
        let mem = Rc::new(RefCell::new(Memory::default()));
        let mut timer = Timer::new(Rc::clone(&mem));

        for _ in 0..(256 * 257) {
            timer.increase_divider(1);
        }

        let div = mem.borrow().load(MREG_DIV);
        assert_eq!(div, 1);
    }
}
