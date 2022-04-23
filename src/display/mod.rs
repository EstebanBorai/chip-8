pub mod buffer;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;

pub const BACKGROUND_COLOR: Color = Color::RGB(u8::MIN, u8::MIN, u8::MIN);
pub const FOREGROUND_COLOR: Color = Color::RGB(u8::MAX, u8::MAX, u8::MAX);
pub const SCREEN_AREA: usize = SCREEN_HEIGHT as usize * SCREEN_WIDTH as usize;
pub const SCREEN_HEIGHT: u32 = 32;
pub const SCREEN_WIDTH: u32 = 64;

use self::buffer::DisplayBuffer;

pub struct Display {
    pub(crate) canvas: Canvas<Window>,
    pub(crate) scale: u32,
}

impl Display {
    pub fn new(context: &Sdl, title: &str, scale: u32) -> Self {
        let video = context.video().unwrap();
        let window = video
            .window(title, SCREEN_WIDTH * scale, SCREEN_HEIGHT * scale)
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();

        Self { canvas, scale }
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(BACKGROUND_COLOR);
        self.canvas.clear();
        self.canvas.present();
    }

    pub fn render(&mut self, buff: &DisplayBuffer) {
        for col in 0..SCREEN_WIDTH {
            for row in 0..SCREEN_HEIGHT {
                if buff[(row * SCREEN_WIDTH + col) as usize] > 0 {
                    self.canvas.set_draw_color(FOREGROUND_COLOR);
                    self.canvas
                        .fill_rect(self.make_rectangle(col, row))
                        .unwrap();
                    continue;
                }

                self.canvas.set_draw_color(BACKGROUND_COLOR);
                self.canvas
                    .fill_rect(self.make_rectangle(col, row))
                    .unwrap();
            }
        }

        self.canvas.present();
    }

    fn make_rectangle(&self, col: u32, row: u32) -> Rect {
        Rect::new(
            (col * self.scale) as i32,
            (row * self.scale) as i32,
            self.scale as u32,
            self.scale as u32,
        )
    }
}
