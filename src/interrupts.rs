pub struct Interrupt {
    pub flag: u8,
    pub handler_addr: usize,
}

pub const INTERRUPT_VBLANK: Interrupt = Interrupt {
    flag: 0b0000_0001,
    handler_addr: 0x0040,
};

pub const INTERRUPT_LCD_STAT: Interrupt = Interrupt {
    flag: 0b0000_0010,
    handler_addr: 0x0048,
};

pub const INTERRUPT_TIMER: Interrupt = Interrupt {
    flag: 0b0000_0100,
    handler_addr: 0x0050,
};

pub const INTERRUPT_SERIAL: Interrupt = Interrupt {
    flag: 0b0000_1000,
    handler_addr: 0x0058,
};

pub const INTERRUPT_JOYPAD: Interrupt = Interrupt {
    flag: 0b0001_0000,
    handler_addr: 0x0060,
};


pub const INTERRUPTS: [Interrupt; 5] = [
    INTERRUPT_VBLANK,
    INTERRUPT_LCD_STAT,
    INTERRUPT_TIMER,
    INTERRUPT_SERIAL,
    INTERRUPT_JOYPAD
];
