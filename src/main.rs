use rust_game_boy_emulator::emulator;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() {
    env_logger::init();

    let mut cartridge: [u8; 0x200000] = [0; 0x200000];

    // load cartdrige file from command line argument
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <cartdrige file>", args[0]);
        std::process::exit(1);
    }

    let mut file = OpenOptions::new().read(true).open(&args[1]).unwrap();
    match file.read_exact(&mut cartridge) {
        Ok(_) => (),
        Err(ref error) if error.kind() == std::io::ErrorKind::UnexpectedEof => (),
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            std::process::exit(1);
        }
    }

    emulator::run(&cartridge);
}
