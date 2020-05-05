mod hardware;
use hardware::cpu::CPU;
use clap::App;
use std::fs;
use std::process;

use std::thread;
use std::time::Duration;

use sdl2::event::Event;

fn main() {

    let sdl_context = sdl2::init().unwrap();

    let mut display_driver = DisplayDriver::new(&sdl_context);

    let matches = App::new("chip8-emulator")
        .version("0.1.0")
        .author("Federico Scozzafava <federico.scozzafava@gmail.com>")
        .about("A simple Chip-8 emulator written in Rust.")
        .arg("<ROM_FILE> 'The input ROM file to use'")
        .get_matches();

    let rom_file: String = matches.value_of_t_or_exit("ROM_FILE");
    let rom_content = fs::read(rom_file).unwrap_or_else(|err| {
        eprintln!("An error occurred while reading the ROM FILE:\n{}", err);
        process::exit(1);
    });

    let sleep_duration = Duration::from_millis(2);





    let mut cpu = CPU::new();
    cpu.load_memory(&rom_content);

    let mut event_pump = sdl_context.event_pump().unwrap();



    'outer: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'outer;
                },
                _ => {}
            }
        }

        let keys: Vec<Keycode> = event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        let mut chip8_keys = [false; 16];

        for key in keys {
            let index = match key {
                Keycode::Num1 => Some(0x1),
                Keycode::Num2 => Some(0x2),
                Keycode::Num3 => Some(0x3),
                Keycode::Num4 => Some(0xc),
                Keycode::Q => Some(0x4),
                Keycode::W => Some(0x5),
                Keycode::E => Some(0x6),
                Keycode::R => Some(0xd),
                Keycode::A => Some(0x7),
                Keycode::S => Some(0x8),
                Keycode::D => Some(0x9),
                Keycode::F => Some(0xe),
                Keycode::Z => Some(0xa),
                Keycode::X => Some(0x0),
                Keycode::C => Some(0xb),
                Keycode::V => Some(0xf),
                _ => None,
            };

            if let Some(i) = index {
                chip8_keys[i] = true;
            }
        }

        cpu.emulate_cycle(chip8_keys);
        if cpu.video_flag {
            display_driver.draw(&cpu.vram);
        }
        thread::sleep(sleep_duration);
    }
}

use sdl2;
use sdl2::pixels;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::keyboard::Keycode;

const CHIP8_WIDTH: usize = 64;
const CHIP8_HEIGHT: usize = 32;

const SCALE_FACTOR: u32 = 10;
const SCREEN_WIDTH: u32 = (CHIP8_WIDTH as u32) * SCALE_FACTOR;
const SCREEN_HEIGHT: u32 = (CHIP8_HEIGHT as u32) * SCALE_FACTOR;

pub struct DisplayDriver {
    canvas: Canvas<Window>,
}

impl DisplayDriver {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        let video_subsys = sdl_context.video().unwrap();
        let window = video_subsys
            .window(
                "rust-sdl2_gfx: draw line & FPSManager",
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
            )
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        DisplayDriver { canvas: canvas }
    }

    pub fn draw(&mut self, pixels: &[u8; CHIP8_WIDTH * CHIP8_HEIGHT]) {
        for (index, &pixel) in pixels.iter().enumerate() {
            let x = index % CHIP8_WIDTH;
            let y = index / CHIP8_WIDTH;
            let x = (x as u32) * SCALE_FACTOR;
            let y = (y as u32) * SCALE_FACTOR;

            self.canvas.set_draw_color(color(pixel));
            let _ = self.canvas
                .fill_rect(Rect::new(x as i32, y as i32, SCALE_FACTOR, SCALE_FACTOR));
        }
        self.canvas.present();
    }
}

fn color(value: u8) -> pixels::Color {
    if value == 0 {
        pixels::Color::RGB(0, 0, 0)
    } else {
        pixels::Color::RGB(0, 250, 0)
    }
}
