use crate::cpu::{Register16bTarget, CPU};
use crate::instructions;
use crate::memory::MemoryBus;
use crate::opcode_info::{OpcodeInfo, OperandInformation};

pub struct Gameboy<'a> {
    pub cpu: CPU,
    pub bus: MemoryBus<'a>,
    pub opcode_info: OpcodeInfo,
    pub interrupts_enabled: bool,
}

impl<'a> Default for Gameboy<'a> {
    fn default() -> Self {
        // load opcode info
        let json = include_bytes!("Opcodes.json");
        let opcode_info: OpcodeInfo = serde_json::from_slice(json).unwrap();

        let cpu = CPU::default();

        Self {
            cpu,
            opcode_info,
            bus: MemoryBus::default(),
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

impl<'a> Gameboy<'a> {
    pub fn run(&mut self, cartridge: &'a [u8; 0x200000]) {
        self.bus.cartridge_rom = cartridge;

        // load first 0x8000 bytes of cartridge into memory
        // self.bus.memory[..0x8000].copy_from_slice(&self.cartridge[..0x8000]);

        let mut total_cycles: u64 = 0;
        loop {
            log::debug!("{:?}", self.cpu.registers);
            if self.cpu.registers.get_u16(Register16bTarget::PC) >= 0x100 {
                break;
            }
            total_cycles += self.step();
        }
        // log::info!("total cycles: {}", total_cycles);
    }

    pub fn step(&mut self) -> u64 {
        let instruction = self.get_next_instruction();
        let cycles = instruction(self);

        self.handle_interrupts();
        self.serial_comm();
        cycles as u64
    }

    fn handle_interrupts(&mut self) {
        // check if interrupts are enabled
        if self.interrupts_enabled {
            // check if any interrupts are enabled
            let interrupt_enable = self.bus.read_byte(0xFFFF);
            let interrupt_flags = self.bus.read_byte(0xFF0F);
            let interrupt = interrupt_enable & interrupt_flags;
            if interrupt != 0 {
                log::debug!("Interrupt: 0x{:02X}", interrupt);
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

    fn get_next_instruction(&mut self) -> Box<instructions::Instruction> {
        let address = self.cpu.registers.get_u16(Register16bTarget::PC);

        let mut instruction_byte = self.bus.read_byte(address);

        let opcode_info;
        let instruction;

        if instruction_byte == 0xCB {
            instruction_byte = self.bus.read_byte(address.wrapping_add(1));

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
            "PC: 0x{:04X}: 0x{:02X} instruction: {} {:?}",
            address,
            instruction_byte,
            opcode_info.mnemonic,
            <Vec<OperandInformation> as Clone>::clone(&opcode_info.operands)
                .into_iter()
                .map(|operands| operands.name)
                .collect::<Vec<String>>(),
        );
        instruction
    }

    pub fn read_next_byte(&mut self) -> u8 {
        let address = self.cpu.registers.get_u16(Register16bTarget::PC);
        let new_address = address.wrapping_add(1);
        self.cpu
            .registers
            .set_u16(Register16bTarget::PC, new_address);
        self.bus.read_byte(new_address)
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
                ..Default::default()
            },
            ..Default::default()
        };
        gameboy.step();
        assert_eq!(gameboy.cpu.registers.get_u8(RegisterTarget::A), 7);
        assert_eq!(gameboy.cpu.registers.get_u16(Register16bTarget::PC), 1246);
    }
}
