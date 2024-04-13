use crate::cpu::{Register16bTarget, CPU};
use crate::instructions;
use crate::memory::MemoryBus;
use crate::opcode_info::OpcodeInfo;

pub struct Gameboy<'a> {
    pub cpu: CPU,
    pub bus: MemoryBus,
    pub cartridge: &'a [u8; 0x200000],
    pub opcode_info: OpcodeInfo,
    pub interrupts_enabled: bool,
}

impl Default for Gameboy<'_> {
    fn default() -> Self {
        // load opcode info
        let json = include_bytes!("Opcodes.json");
        let opcode_info: OpcodeInfo = serde_json::from_slice(json).unwrap();

        let cpu = CPU::default();

        Self {
            cpu,
            opcode_info,
            bus: MemoryBus::default(),
            cartridge: &[0; 0x200000],
            interrupts_enabled: false,
        }
    }
}

// DMG initialization sequence
pub fn initialize(gameboy: &mut Gameboy) {
    gameboy.cpu.registers.a = 0x01;
    gameboy.cpu.registers.f.zero = true;
    //TODO: set carry and half carry based on header checksum
    gameboy.cpu.registers.c = 0x13;
    gameboy.cpu.registers.e = 0xD8;
    gameboy.cpu.registers.h = 0x01;
    gameboy.cpu.registers.l = 0x4D;
}

pub fn load_dmg_rom(gameboy: &mut Gameboy) {
    let rom = include_bytes!("dmg.bin");
    gameboy.bus.memory[..rom.len()].copy_from_slice(rom);
}

impl<'a> Gameboy<'a> {
    pub fn run(&'a mut self, cartridge: &'a [u8; 0x200000]) {
        self.cartridge = cartridge;

        // load first 0x8000 bytes of cartridge into memory
        // self.bus.memory[..0x8000].copy_from_slice(&self.cartridge[..0x8000]);

        loop {
            self.step();
        }
    }

    pub fn step(&mut self) {
        let instruction = self.get_next_instruction();
        instruction(self);

        self.handle_interrupts();
        self.serial_comm();
    }

    fn handle_interrupts(&mut self) {
        // check if interrupts are enabled
        if self.interrupts_enabled {
            // check if any interrupts are enabled
            let interrupt_enable = self.bus.read_byte(0xFFFF);
            let interrupt_flags = self.bus.read_byte(0xFF0F);
            let interrupt = interrupt_enable & interrupt_flags;
            if interrupt != 0 {
                // disable interrupts
                self.interrupts_enabled = false;
                // push current program counter to stack
                let pc = self.cpu.registers.get_u16(Register16bTarget::PC);
                self.cpu.registers.stack_push(pc, &mut self.bus);
                // jump to interrupt handler
                let interrupt_handler = match interrupt {
                    0x01 => 0x40, // V-Blank
                    0x02 => 0x48, // LCD STAT
                    0x04 => 0x50, // Timer
                    0x08 => 0x58, // Serial
                    0x10 => 0x60, // Joypad
                    _ => panic!("invalid interrupt"),
                };
                self.cpu
                    .registers
                    .set_u16(Register16bTarget::PC, interrupt_handler);
            }
        }
    }

    fn serial_comm(&mut self) {
        // read from serial port if requested
        if self.bus.memory[0xFF02] == 0x81 {
            let byte = self.bus.read_byte(0xFF01);
            print!("{}", byte as char);
            self.bus.memory[0xFF02] = 0x0;
        }
    }

    fn get_next_instruction(&mut self) -> Box<dyn Fn(&mut Gameboy)> {
        let address = self.cpu.registers.get_u16(Register16bTarget::PC);
        let instruction_byte = self.bus.read_byte(address);
        let opcode_info;
        let instruction;
        self.cpu
            .registers
            .set_u16(Register16bTarget::PC, address.wrapping_add(1));
        if instruction_byte == 0xCB {
            let instruction_byte = self.bus.read_byte(address);
            self.cpu
                .registers
                .set_u16(Register16bTarget::PC, address.wrapping_add(1));
            instruction = instructions::from_prefixed_byte(instruction_byte);
            opcode_info = self
                .opcode_info
                .cbprefixed
                .get(&format!("0x{:02X}", instruction_byte))
                .unwrap();
        } else {
            instruction = instructions::from_byte(instruction_byte);
            opcode_info = self
                .opcode_info
                .unprefixed
                .get(&format!("0x{:02X}", instruction_byte))
                .unwrap();
        };
        log::debug!(
            "instruction byte: 0x{:02X}\nprogram counter: 0x{:04X}\ninstruction: {}\noperands: {:?}",
            instruction_byte,
            address,
            opcode_info.mnemonic,
            opcode_info.operands
        );
        instruction
    }

    pub fn read_next_byte(&mut self) -> u8 {
        let address = self.cpu.registers.get_u16(Register16bTarget::PC);
        let byte = self.bus.read_byte(address);
        self.cpu
            .registers
            .set_u16(Register16bTarget::PC, address.wrapping_add(1));
        byte
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::{FlagsRegister, RegisterTarget, Registers};

    #[test]
    fn test_add() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    a: 3,
                    c: 4,
                    f: FlagsRegister::from(0),
                    pc: 1245,
                    ..Default::default()
                },
                ..Default::default()
            },
            bus: MemoryBus {
                memory: [0x81; 0x10000],
            },
            ..Default::default()
        };
        gameboy.step();
        assert_eq!(gameboy.cpu.registers.get_u8(RegisterTarget::A), 7);
        assert_eq!(gameboy.cpu.registers.get_u16(Register16bTarget::PC), 1246);
    }
}
