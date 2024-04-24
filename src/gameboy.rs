use crate::cpu::{Register16bTarget, RegisterTarget, CPU};
use crate::memory::special_addresses::*;
use crate::memory::{self, MemoryBus};
use crate::opcode_info::{OpcodeInfo, OperandInformation};
use crate::ppu::PPU;
use crate::{instructions, ppu};
use std::mem;
use std::time::Instant;

macro_rules! flag_set_at {
    ($byte:expr, $bit:expr) => {
        ($byte >> $bit) & 1 == 1
    };
}

pub enum Interrupt {
    VBlank,
    LCDStat,
    Timer,
    Serial,
    Joypad,
}

impl Interrupt {
    pub fn interrupt_address(value: u8) -> (Interrupt, u16) {
        if value & u8::from(Interrupt::VBlank) != 0 {
            (Interrupt::VBlank, 0x0040)
        } else if value & u8::from(Interrupt::LCDStat) != 0 {
            (Interrupt::LCDStat, 0x0048)
        } else if value & u8::from(Interrupt::Timer) != 0 {
            (Interrupt::Timer, 0x0050)
        } else if value & u8::from(Interrupt::Serial) != 0 {
            (Interrupt::Serial, 0x0058)
        } else if value & u8::from(Interrupt::Joypad) != 0 {
            (Interrupt::Joypad, 0x0060)
        } else {
            panic!("invalid interrupt");
        }
    }
}

impl std::convert::From<u8> for Interrupt {
    fn from(byte: u8) -> Self {
        match byte {
            0x01 => Interrupt::VBlank,
            0x02 => Interrupt::LCDStat,
            0x04 => Interrupt::Timer,
            0x08 => Interrupt::Serial,
            0x10 => Interrupt::Joypad,
            _ => panic!("invalid interrupt"),
        }
    }
}
impl std::convert::From<Interrupt> for u8 {
    fn from(value: Interrupt) -> Self {
        match value {
            Interrupt::VBlank => 0x01,
            Interrupt::LCDStat => 0x02,
            Interrupt::Timer => 0x04,
            Interrupt::Serial => 0x08,
            Interrupt::Joypad => 0x10,
        }
    }
}

pub struct Gameboy<'a> {
    pub cpu: CPU,
    pub bus: MemoryBus<'a>,
    pub opcode_info: OpcodeInfo,
    pub interrupts_enabled: bool,
    pub scanline_counter: u64,
    pub divider_counter: u8,
    pub timer_counter: u64,
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
            scanline_counter: 0,
            divider_counter: 0,
            timer_counter: 0,
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

        let sdl_context = sdl2::init().unwrap();

        let mut ppu = PPU::new(&sdl_context);

        let mut event_pump = sdl_context.event_pump().unwrap();

        'running: loop {
            let before = Instant::now();
            for event in event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit { .. }
                    | sdl2::event::Event::KeyDown {
                        keycode: Some(sdl2::keyboard::Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }

            self.run_frame(&mut ppu);
            let elapsed = before.elapsed();
            let fps = 1.0 / elapsed.as_secs_f64();
            log::info!("FPS: {:.2?}", fps);
            ppu.draw();
        }
    }

    fn run_frame(&mut self, ppu: &mut PPU) {
        const MAX_TICKS: u64 = 69905 * 4; // 69905 cycles per frame, 4 ticks per cycle
                                          // frame
        {
            let mut frame_ticks: u64 = 0;
            while frame_ticks < MAX_TICKS {
                log::debug!("{:?}", self.cpu.registers);
                //     break;
                // if self.cpu.registers.get_u16(Register16bTarget::PC) >= 0x100 {
                let ticks = self.run_next_instruction();

                self.handle_interrupts();
                // self.serial_comm();
                self.update_timers(ticks);
                self.update_graphics(ticks, ppu);
                frame_ticks += ticks as u64;
            }
        }
    }

    pub fn run_next_instruction(&mut self) -> u8 {
        self.get_next_instruction()(self)
    }

    fn handle_interrupts(&mut self) {
        // check if interrupts are enabled
        if self.interrupts_enabled {
            // check if any interrupts are enabled
            let interrupt_enable = self.bus.memory[IE];
            let interrupt_flags = self.bus.memory[IF];
            let interrupt_value = interrupt_enable & interrupt_flags;
            if interrupt_value != 0 {
                log::debug!("Interrupt: 0x{:02X}", interrupt_value);
                // disable interrupts
                self.interrupts_enabled = false;
                let (interrupt, interrupt_handler) = Interrupt::interrupt_address(interrupt_value);
                // disable interrupt
                self.bus.memory[IF] = interrupt_flags & !u8::from(interrupt);
                // push current program counter to stack
                let pc = self.cpu.registers.get_u16(Register16bTarget::PC);
                self.cpu.registers.stack_push(pc, &mut self.bus);
                // jump to interrupt handler
                self.cpu
                    .registers
                    .set_u16(Register16bTarget::PC, interrupt_handler);
            }
        }
    }

    fn serial_comm(&mut self) {
        // read from serial port if requested
        if self.bus.memory[SC] == 0x81 {
            let byte = self.bus.memory[SB];
            print!("{}", byte as char);
            self.bus.memory[SB] = 0x0;
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

    pub fn request_interrupt(&mut self, interrupt: Interrupt) {
        let interrupt_flags = self.bus.memory[IF];
        self.bus.memory[IF] = interrupt_flags | u8::from(interrupt);
    }

    pub fn read_next_byte(&mut self) -> u8 {
        let address = self.cpu.registers.get_u16(Register16bTarget::PC);
        let new_address = address.wrapping_add(1);
        self.cpu
            .registers
            .set_u16(Register16bTarget::PC, new_address);
        self.bus.read_byte(new_address)
    }

    fn update_graphics(&mut self, ticks: u8, ppu: &mut PPU) {
        self.scanline_counter += ticks as u64;
        if self.scanline_counter >= 456 * 4 {
            self.scanline_counter = 0;
            // self.update_scanline();
            let mut current_scanline = self.bus.memory[LY];
            if current_scanline == 144 {
                // RequestInterupt(0);
            }
            if current_scanline > 153 {
                current_scanline = 0;
            } else {
                current_scanline += 1;
            }
            // TODO: writing directly to memory
            self.bus.memory[0xFF44] = current_scanline;
            ppu.update(self, current_scanline);
        }
    }

    const CLOCK_SPEED: u64 = 4194304;
    fn update_timers(&mut self, ticks: u8) {
        let mut tima = self.bus.memory[TIMA]; // timer
        let tma = self.bus.memory[TMA]; // timer modulo
        let tac = self.bus.memory[TAC]; // timer controller
        let mut div = self.bus.memory[DIV];

        let frequency = match tac & 0x3 {
            0 => 1024,
            1 => 16,
            2 => 64,
            3 => 256,
            _ => panic!("invalid timer"),
        };

        let (new_divider_counter, overflow) = self.divider_counter.overflowing_add(ticks);
        self.divider_counter = new_divider_counter;
        if overflow {
            div = div.wrapping_add(1);
        }

        if flag_set_at!(tac, 2) {
            self.timer_counter += ticks as u64;
            if self.timer_counter >= Self::CLOCK_SPEED / frequency {
                self.timer_counter = 0;
                tima = tima.wrapping_add(1);
                if tima == 0 {
                    tima = tma;
                    self.request_interrupt(Interrupt::Timer);
                }
            }
        }

        self.bus.memory[memory::special_addresses::DIV] = div;
        self.bus.memory[TIMA] = tima;
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
                    pc: 0xC050,
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
        gameboy.run_next_instruction();
        assert_eq!(gameboy.cpu.registers.get_u8(RegisterTarget::A), 7);
        assert_eq!(gameboy.cpu.registers.get_u16(Register16bTarget::PC), 0xC051);
    }
}
