use env_logger;
use rust_game_boy_emulator::CPU;
use serde::Deserialize;
use serde_json::Map;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::prelude::*;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
struct FlagInformation {
    z: String,
    n: String,
    h: String,
    c: String,
}

#[derive(Deserialize, Debug)]
struct OperandInformation {
    name: String,
    immediate: bool,
}

#[derive(Deserialize, Debug)]
struct UnprefixedOpcode {
    operands: Vec<OperandInformation>,
    mnemonic: String,
    bytes: u8,
    immediate: bool,
    cycles: Vec<u8>,
    flags: FlagInformation,
}

#[derive(Deserialize, Debug)]
struct OpcodeInfo {
    unprefixed: HashMap<String, UnprefixedOpcode>,
}

fn main() {
    env_logger::init();

    // load opcode info
    let json = std::fs::read_to_string("Opcodes.json").unwrap();

    let opcode_info: OpcodeInfo = serde_json::from_str(&json).unwrap();

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
    let mut i = 0;

    let hook = move |instruction_byte: u8| {
        i += 1;
        let opcode = opcode_info
            .unprefixed
            .get(&format!("0x{:02X}", instruction_byte));
        if let Some(opcode) = opcode {
            println!(
                "PC = {:04X}: Next Instruction {} {:?} {}",
                cpu.pc, opcode.mnemonic, opcode.operands, i
            );
        } else {
            println!("Instruction: {:02X}", instruction_byte);
        }
    };
    cpu.stepHook = Box::new(hook);

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
