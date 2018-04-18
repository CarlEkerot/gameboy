use sdl2;
use sdl2::pixels::Color;
use sdl2::gfx::primitives::DrawRenderer;
use std::time::Duration;
use constants::*;

pub enum Shade {
    Shade0,
    Shade1,
    Shade2,
    Shade3,
}

pub struct LCD {
    data: [[Color; LCD_PIXELS_X]; LCD_PIXELS_Y],  // 160x144 bits
    canvas: sdl2::render::WindowCanvas,
}

impl LCD {
    pub fn new() -> LCD {
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
            data: [[Color::RGB(155, 188, 15); LCD_PIXELS_X]; LCD_PIXELS_Y],
            canvas,
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
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run() {
        let mut l = LCD::new();
        l.set_pixel(10, 10, Shade::Shade0);
        l.set_pixel(10, 11, Shade::Shade0);
        l.set_pixel(10, 12, Shade::Shade0);
        l.set_pixel(10, 13, Shade::Shade0);
        l.set_pixel(10, 14, Shade::Shade0);
        l.set_pixel(10, 15, Shade::Shade0);
        l.set_pixel(11, 15, Shade::Shade0);
        l.set_pixel(12, 15, Shade::Shade0);
        l.set_pixel(13, 15, Shade::Shade0);
        l.set_pixel(14, 15, Shade::Shade0);
        l.show();
    }
}