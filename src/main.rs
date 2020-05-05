mod hardware;
use hardware::cpu::CPU;

mod interfaces;
use sdl2::event::Event;

use clap::Clap;
use std::fs;
use std::process;

use std::path::PathBuf;
use std::time::Instant;

#[derive(Clap)]
#[clap(version, author)]
struct Opt {
    #[clap(short, long, default_value = "500")]
    /// The emulation speed
    speed: f64,
    #[clap(takes_value = true)]
    rom_file: PathBuf,
}

fn main() {
    let opts = Opt::parse();
    let emulation_speed: f64 = opts.speed;
    let rom_content = fs::read(opts.rom_file).unwrap_or_else(|err| {
        eprintln!("An error occurred while reading the ROM FILE:\n{}", err);
        process::exit(1);
    });

    let sleep_duration = (1. / emulation_speed * 1000.) as u128;

    let mut cpu = CPU::new();

    cpu.load_memory(&rom_content);

    let mut interface_manager = interfaces::InterfaceManager::new();

    loop {
        let current_time = Instant::now();

        if let Some(Event::Quit { timestamp: _ }) = interface_manager.run() {
            break;
        }

        let keypad = interface_manager
            .input_interface
            .poll(&interface_manager.event_pump);

        let state = cpu.emulate_cycle(keypad);

        if state.updated_vram {
            interface_manager.video_interface.draw(&cpu.vram);
        }

        if state.beep {
            interface_manager.audio_interface.beep();
        } else {
            interface_manager.audio_interface.no_beep();
        }

        while current_time.elapsed().as_millis() < sleep_duration {}
    }
}
