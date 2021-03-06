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
pub const LCD_TITLE: &str = "Gameboy";

pub const MREG_P1: usize = 0xff00;
pub const MREG_SB: usize = 0xff01;
pub const MREG_SC: usize = 0xff02;
pub const MREG_DIV: usize = 0xff04;
pub const MREG_TIMA: usize = 0xff05;
pub const MREG_TMA: usize = 0xff06;
pub const MREG_TAC: usize = 0xff07;
pub const MREG_IF: usize = 0xff0f;
pub const MREG_NR10: usize = 0xff10;
pub const MREG_NR11: usize = 0xff11;
pub const MREG_NR12: usize = 0xff12;
pub const MREG_NR13: usize = 0xff13;
pub const MREG_NR14: usize = 0xff14;
pub const MREG_NR21: usize = 0xff16;
pub const MREG_NR22: usize = 0xff17;
pub const MREG_NR23: usize = 0xff18;
pub const MREG_NR24: usize = 0xff19;
pub const MREG_NR30: usize = 0xff1a;
pub const MREG_NR31: usize = 0xff1b;
pub const MREG_NR32: usize = 0xff1c;
pub const MREG_NR33: usize = 0xff1d;
pub const MREG_NR34: usize = 0xff1e;
pub const MREG_NR41: usize = 0xff20;
pub const MREG_NR42: usize = 0xff21;
pub const MREG_NR43: usize = 0xff22;
pub const MREG_NR44: usize = 0xff23;
pub const MREG_NR50: usize = 0xff24;
pub const MREG_NR51: usize = 0xff25;
pub const MREG_NR52: usize = 0xff26;
pub const MREG_WAV00: usize = 0xff30;
pub const MREG_WAV01: usize = 0xff31;
pub const MREG_WAV02: usize = 0xff32;
pub const MREG_WAV03: usize = 0xff33;
pub const MREG_WAV04: usize = 0xff34;
pub const MREG_WAV05: usize = 0xff35;
pub const MREG_WAV06: usize = 0xff36;
pub const MREG_WAV07: usize = 0xff37;
pub const MREG_WAV08: usize = 0xff38;
pub const MREG_WAV09: usize = 0xff39;
pub const MREG_WAV10: usize = 0xff3a;
pub const MREG_WAV11: usize = 0xff3b;
pub const MREG_WAV12: usize = 0xff3c;
pub const MREG_WAV13: usize = 0xff3d;
pub const MREG_WAV14: usize = 0xff3e;
pub const MREG_WAV15: usize = 0xff3f;
pub const MREG_LCDC: usize = 0xff40;
pub const MREG_STAT: usize = 0xff41;
pub const MREG_SCY: usize = 0xff42;
pub const MREG_SCX: usize = 0xff43;
pub const MREG_LY: usize = 0xff44;
pub const MREG_LYC: usize = 0xff45;
pub const MREG_DMA: usize = 0xff46;
pub const MREG_BGP: usize = 0xff47;
pub const MREG_OBP0: usize = 0xff48;
pub const MREG_OBP1: usize = 0xff49;
pub const MREG_WY: usize = 0xff4a;
pub const MREG_WX: usize = 0xff4b;
pub const MREG_KEY1: usize = 0xff4d;
pub const MREG_VBK: usize = 0xff4f;
pub const MREG_BOOT: usize = 0xff50;
pub const MREG_HDMA1: usize = 0xff51;
pub const MREG_HDMA2: usize = 0xff52;
pub const MREG_HDMA3: usize = 0xff53;
pub const MREG_HDMA4: usize = 0xff54;
pub const MREG_HDMA5: usize = 0xff55;
pub const MREG_RP: usize = 0xff56;
pub const MREG_BCPS: usize = 0xff68;
pub const MREG_BCPD: usize = 0xff69;
pub const MREG_OCPS: usize = 0xff6a;
pub const MREG_OCPD: usize = 0xff6b;
pub const MREG_SVBK: usize = 0xff70;
pub const MREG_PCM12: usize = 0xff76;
pub const MREG_PCM34: usize = 0xff77;
pub const MREG_IE: usize = 0xffff;

pub const CLOCK_SPEED: usize = 4_194_304;
pub const FRAME_RATE: usize = 60;
pub const H_SYNC: usize = 9_198_000;
pub const V_SYNC: usize = 60;

pub const TIMER_CYCLES_PER_TICK: [usize; 4] = [
    1024,
    16,
    64,
    256,
];

pub const LCD_MODE0_FLAG: u8 = 0b00;
pub const LCD_MODE1_FLAG: u8 = 0b01;
pub const LCD_MODE2_FLAG: u8 = 0b10;
pub const LCD_MODE3_FLAG: u8 = 0b11;

pub const LCD_MODE0_CYCLES: usize = 204;
pub const LCD_MODE1_CYCLES: usize = 4560;
pub const LCD_MODE2_CYCLES: usize = 80;
pub const LCD_MODE3_CYCLES: usize = 172;

pub const LY_MAX: usize = 154;
