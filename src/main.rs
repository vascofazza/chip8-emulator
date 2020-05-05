mod hardware;
use hardware::cpu::CPU;

mod interfaces;
use sdl2::event::Event;

use clap::{App, Arg};
use std::fs;
use std::process;

use std::time::Instant;

fn main() {

    let matches = App::new("chip8-emulator")
        .version("0.1.0")
        .author("Federico Scozzafava <federico.scozzafava@gmail.com>")
        .about("A simple Chip-8 emulator written in Rust.")
        .arg("<ROM_FILE> 'The input ROM file to use'")
        .arg(Arg::with_name("speed")
            .short('s')
            .help_heading(Option::from("The emulation speed"))
            .default_value("500")
        )
        .get_matches();

    let emulation_speed: f64 = matches.value_of("speed").unwrap().parse().expect("Invalid speed value.");

    let rom_file: String = matches.value_of_t_or_exit("ROM_FILE");
    let rom_content = fs::read(rom_file).unwrap_or_else(|err| {
        eprintln!("An error occurred while reading the ROM FILE:\n{}", err);
        process::exit(1);
    });

    let sleep_duration = (1. / emulation_speed * 1000.) as u128;


    let mut cpu = CPU::new();

    cpu.load_memory(&rom_content);

    let mut interface_manager = interfaces::InterfaceManager::new();

    loop {

        let current_time = Instant::now();

        if let Some(Event::Quit {timestamp: _}) = interface_manager.run()
        {
            break;
        }

        let keypad = interface_manager.input_interface.poll(&interface_manager.event_pump);

        let state = cpu.emulate_cycle(keypad);

        if state.updated_vram {
            interface_manager.video_interface.draw(&cpu.vram);
        }

        if state.beep
        {
            interface_manager.audio_interface.beep();
        } else
        {
            interface_manager.audio_interface.no_beep();
        }

        while current_time.elapsed().as_millis() < sleep_duration {} ;
    }
}
