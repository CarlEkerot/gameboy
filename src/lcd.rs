use sdl2;
use sdl2::pixels::Color;
use sdl2::gfx::primitives::DrawRenderer;
use std::time::Duration;
use std::rc::Rc;
use std::cell::RefCell;
use constants::*;
use memory::Memory;

pub enum Shade {
    Shade0,
    Shade1,
    Shade2,
    Shade3,
}

pub struct LCD {
    mem: Rc<RefCell<Memory>>,
    data: [[Color; LCD_PIXELS_X]; LCD_PIXELS_Y],  // 160x144 bits
    canvas: sdl2::render::WindowCanvas,
    state: u8,
    cycles: usize,
}

impl LCD {
    pub fn new(mem: Rc<RefCell<Memory>>) -> LCD {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window(LCD_TITLE,
                                            LCD_PIXELS_X as u32,
                                            LCD_PIXELS_Y as u32)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();
        LCD {
            mem,
            data: [[Color::RGB(155, 188, 15); LCD_PIXELS_X]; LCD_PIXELS_Y],
            canvas,
            state: LCD_MODE0_FLAG,
            cycles: 0,
        }

    }

    pub fn set_pixel(&mut self, x: usize, y: usize, shade: Shade) {
        self.data[y][x] = match shade {
            Shade::Shade0 => Color::RGB(15, 56, 15),
            Shade::Shade1 => Color::RGB(48, 98, 48),
            Shade::Shade2 => Color::RGB(139, 172, 15),
            Shade::Shade3 => Color::RGB(155, 188, 15),
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.mem.borrow().is_set(MREG_LCDC, 7)
    }

    pub fn show(&mut self) {
        self.canvas.clear();
        for y in 0..LCD_PIXELS_Y {
            for x in 0..LCD_PIXELS_X {
                let color = self.data[y][x];
                self.canvas.pixel(x as i16, y as i16, color).unwrap();
            }
        }
        self.canvas.present();
        ::std::thread::sleep(Duration::new(10, 1_000_000_000u32 / 60));
    }

    pub fn update_frame(&mut self) {

    }

    fn increase_line_count(&self) -> u8 {
        let mut mem = self.mem.borrow();
        let ly = mem.load_unchecked(MREG_LY);
        let new_ly = (ly + 1) % LY_MAX;
        mem.store_unchecked(MREG_LY, new_ly);
        new_ly
    }

    fn set_mode(&mut self, flag: u8) {
        let mut mem = self.mem.borrow_mut();
        let stat = mem.load(MREG_STAT);
        let mask = 0b11111100 | flag;
        mem.store(MREG_STAT, stat & mask);
        self.cycles = 0;
    }

    fn update_mode0(&mut self) {
        if self.cycles >= LCD_MODE0_CYCLES {
            let ly = self.increase_line_count();

            self.set_mode(LCD_MODE1_FLAG);
        }
    }

    fn update_mode1(&mut self) {
        if self.cycles >= LCD_MODE1_CYCLES {
            self.set_mode(LCD_MODE2_FLAG);
        }
    }

    fn update_mode2(&mut self) {
        if self.cycles >= LCD_MODE2_CYCLES {
            self.set_mode(LCD_MODE3_FLAG);
        }
    }

    fn update_mode3(&mut self) {
        if self.cycles >= LCD_MODE3_CYCLES {
            self.set_mode(LCD_MODE0_FLAG);
        }
    }

    pub fn update(&mut self, cycles: usize) {
        self.cycles += cycles;
        let state = self.mem.borrow().load(MREG_STAT) & 0b11;
        match self.state {
            LCD_MODE0_FLAG => self.update_mode0(),
            LCD_MODE1_FLAG => self.update_mode1(),
            LCD_MODE2_FLAG => self.update_mode2(),
            LCD_MODE3_FLAG => self.update_mode3(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lcd_enabled() {
        let mem = Rc::new(RefCell::new(Memory::default()));
        let lcd = LCD::new(Rc::clone(&mem));
        assert!(!lcd.is_enabled());
        mem.borrow_mut().set_register_flag(MREG_LCDC, 0b1000_0000);
        assert!(lcd.is_enabled());
    }
}
