pub const EXTENDED_INSTRUCTION: u16 = 0xcb;

pub const REG_A: usize = 0;
pub const REG_F: usize = 1;
pub const REG_B: usize = 2;
pub const REG_C: usize = 3;
pub const REG_D: usize = 4;
pub const REG_E: usize = 5;
pub const REG_H: usize = 6;
pub const REG_L: usize = 7;

pub const BYTE: usize = 8;
pub const SHORT: usize = 16;

pub const OFFSET_BASE: usize = 0xff00;

pub const FLAG_Z: u8 = 0b1000_0000;
pub const FLAG_N: u8 = 0b0100_0000;
pub const FLAG_H: u8 = 0b0010_0000;
pub const FLAG_C: u8 = 0b0001_0000;

pub const LCD_PIXELS_X: usize = 160;
pub const LCD_PIXELS_Y: usize = 144;
pub const LCD_BYTES_X: usize = 20;
pub const LCD_BYTES_Y: usize = 18;
pub const LCD_TITLE: &'static str = "Gameboy";
