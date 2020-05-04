mod hardware;
use hardware::cpu::CPU;
use clap::App;
use std::fs;
use std::process;


fn main() {

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


    let mut cpu = CPU::new();
    cpu.load_memory(&rom_content);
    loop {
        cpu.emulate_cycle();
        println!("{:?}", cpu);
    }
}
