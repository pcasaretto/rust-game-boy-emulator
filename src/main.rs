use rust_game_boy_emulator::CPU;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() {
    let mut cartridge = vec![0; 0x200000];

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

    let mut cpu = CPU::default();
    initialize(&mut cpu);

    // load first 0x8000 bytes of cartridge into memory
    cpu.bus.memory[..0x8000].copy_from_slice(&cartridge[..0x8000]);

    loop {
        cpu.step();
    }
}

// DMG initialization sequence
fn initialize(cpu: &mut CPU) {
    cpu.registers.a = 0x01;
    cpu.registers.f.zero = true;
    //TODO: set carry and half carry based on header checksum
    cpu.registers.c = 0x13;
    cpu.registers.e = 0xD8;
    cpu.registers.h = 0x01;
    cpu.registers.l = 0x4D;
    cpu.pc = 0x100;
}
