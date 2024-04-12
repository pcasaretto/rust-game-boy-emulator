use crate::cpu::{Register16bTarget, CPU};
use crate::instructions;
use crate::memory::MemoryBus;
use crate::opcode_info::OpcodeInfo;

pub struct Gameboy<'a> {
    pub cpu: CPU,
    pub bus: MemoryBus,
    pub cartridge: &'a [u8; 0x200000],
    pub opcode_info: OpcodeInfo,
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
    gameboy.cpu.registers.pc = 0x100;
}

impl<'a> Gameboy<'a> {
    pub fn run(&'a mut self, cartridge: &'a [u8; 0x200000]) {
        self.cartridge = cartridge;

        // load first 0x8000 bytes of cartridge into memory
        self.bus.memory[..0x8000].copy_from_slice(&self.cartridge[..0x8000]);

        loop {
            self.step();
        }
    }

    pub fn step(&mut self) {
        let address = self.cpu.registers.get_u16(Register16bTarget::PC);
        let instruction_byte = self.bus.read_byte(address);
        let opcode_info = self
            .opcode_info
            .unprefixed
            .get(&format!("0x{:02X}", instruction_byte))
            .unwrap();
        log::debug!(
            "program counter: 0x{:04X}\ninstruction: {}\noperands: {:?}",
            self.cpu.registers.get_u16(Register16bTarget::PC),
            opcode_info.mnemonic,
            opcode_info.operands
        );
        let instruction = instructions::from_byte(instruction_byte);
        self.cpu
            .registers
            .set_u16(Register16bTarget::PC, address.wrapping_add(1));
        instruction(self);
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
