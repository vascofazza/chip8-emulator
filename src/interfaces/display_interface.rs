use sdl2;
use sdl2::pixels;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::hardware::{CHIP8_HEIGHT, CHIP8_WIDTH};
use rand::Rng;

const SCALE_FACTOR: u32 = 10;
const NOISE_FACTOR: u32 = 4;
const SCREEN_WIDTH: u32 = (CHIP8_WIDTH as u32) * SCALE_FACTOR;
const SCREEN_HEIGHT: u32 = (CHIP8_HEIGHT as u32) * SCALE_FACTOR;

pub struct DisplayInterface {
    canvas: Canvas<Window>,
}

impl DisplayInterface {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        let video_subsys = sdl_context.video().unwrap();
        let window = video_subsys
            .window("Chip-8 Emulator", SCREEN_WIDTH, SCREEN_HEIGHT)
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        DisplayInterface { canvas }
    }

    pub fn draw(&mut self, pixels: &[u8; CHIP8_WIDTH * CHIP8_HEIGHT]) {
        for (index, &pixel) in pixels.iter().enumerate() {
            let x = index % CHIP8_WIDTH;
            let y = index / CHIP8_WIDTH;
            let x = (x as u32) * SCALE_FACTOR;
            let y = (y as u32) * SCALE_FACTOR;
            let mut rng = rand::thread_rng();
            for x_off in 0..SCALE_FACTOR / NOISE_FACTOR {
                for y_off in 0..SCALE_FACTOR / NOISE_FACTOR {
                    let rand: u8 = rng.gen_range(190, 240);

                    self.canvas.set_draw_color(color(pixel * rand));
                    self.canvas.fill_rect(Rect::new(
                        (x + x_off * NOISE_FACTOR) as i32,
                        (y + y_off * NOISE_FACTOR) as i32,
                        NOISE_FACTOR,
                        NOISE_FACTOR,
                    ));
                }
            }
        }
        self.canvas.present();
    }
}

fn color(value: u8) -> pixels::Color {
    if value == 0 {
        pixels::Color::RGB(0, 0, 0)
    } else {
        pixels::Color::RGB(0, value, 0)
    }
}
