use crate::cpu::{Register16bTarget, CPU};
use crate::instructions;
use crate::joypad;
use crate::memory::special_addresses::{self, *};
use crate::memory::{self, MemoryBus};
use crate::opcode_info::{OpcodeInfo, OperandInformation};
use crate::ppu::PPU;
use std::collections::HashMap;
use std::fmt::Debug;
use std::time::{Duration, Instant};

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
            Interrupt::VBlank => 1 << 0,
            Interrupt::LCDStat => 1 << 1,
            Interrupt::Timer => 1 << 2,
            Interrupt::Serial => 1 << 3,
            Interrupt::Joypad => 1 << 4,
        }
    }
}

impl Debug for Interrupt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let interrupt = match self {
            Interrupt::VBlank => "VBlank",
            Interrupt::LCDStat => "LCDStat",
            Interrupt::Timer => "Timer",
            Interrupt::Serial => "Serial",
            Interrupt::Joypad => "Joypad",
        };
        write!(f, "{}", interrupt)
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
    pub joypad: joypad::Joypad,
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
            joypad: joypad::Joypad::new(),
        }
    }
}

lazy_static! {
    static ref KEYMAP: HashMap<sdl2::keyboard::Keycode, joypad::JoypadButton> = {
        let mut m = HashMap::new();
        m.insert(sdl2::keyboard::Keycode::W, joypad::JoypadButton::Up);
        m.insert(sdl2::keyboard::Keycode::A, joypad::JoypadButton::Left);
        m.insert(sdl2::keyboard::Keycode::S, joypad::JoypadButton::Down);
        m.insert(sdl2::keyboard::Keycode::D, joypad::JoypadButton::Right);
        m.insert(sdl2::keyboard::Keycode::O, joypad::JoypadButton::A);
        m.insert(sdl2::keyboard::Keycode::K, joypad::JoypadButton::B);
        m.insert(sdl2::keyboard::Keycode::Return, joypad::JoypadButton::Start);
        m.insert(
            sdl2::keyboard::Keycode::Backspace,
            joypad::JoypadButton::Select,
        );
        m
    };
    static ref DMG_HARDWARE_REGISTER_INIT: HashMap<usize, u8> = {
        let mut m = HashMap::new();
        m.insert(memory::special_addresses::SB, 0x00);
        m.insert(memory::special_addresses::SC, 0x7E);
        m.insert(memory::special_addresses::DIV, 0x18);
        m.insert(memory::special_addresses::TIMA, 0x00);
        m.insert(memory::special_addresses::TMA, 0x00);
        m.insert(memory::special_addresses::TAC, 0xF8);
        m.insert(memory::special_addresses::IF, 0xE1);
        m.insert(memory::special_addresses::NR10, 0x80);
        m.insert(memory::special_addresses::NR11, 0xBF);
        m.insert(memory::special_addresses::NR12, 0xF3);
        m.insert(memory::special_addresses::NR13, 0xFF);
        m.insert(memory::special_addresses::NR14, 0xBF);
        m.insert(memory::special_addresses::NR21, 0x3F);
        m.insert(memory::special_addresses::NR22, 0x00);
        m.insert(memory::special_addresses::NR23, 0xFF);
        m.insert(memory::special_addresses::NR24, 0xBF);
        m.insert(memory::special_addresses::NR30, 0x7F);
        m.insert(memory::special_addresses::NR31, 0xFF);
        m.insert(memory::special_addresses::NR32, 0x9F);
        m.insert(memory::special_addresses::NR33, 0xBF);
        m.insert(memory::special_addresses::NR41, 0xFF);
        m.insert(memory::special_addresses::NR42, 0x00);
        m.insert(memory::special_addresses::NR43, 0x00);
        m.insert(memory::special_addresses::NR30, 0xBF);
        m.insert(memory::special_addresses::NR50, 0x77);
        m.insert(memory::special_addresses::NR51, 0xF3);
        m.insert(memory::special_addresses::NR52, 0xF1);
        m.insert(memory::special_addresses::LCDC, 0x91);
        m.insert(memory::special_addresses::SCY, 0x00);
        m.insert(memory::special_addresses::SCX, 0x00);
        m.insert(memory::special_addresses::LY, 0x85);
        m.insert(memory::special_addresses::LYC, 0x85);
        m.insert(memory::special_addresses::DMA, 0xFF);
        m.insert(memory::special_addresses::BGP, 0xFC);
        m.insert(memory::special_addresses::OBP0, 0x00);
        m.insert(memory::special_addresses::OBP1, 0x00);
        m.insert(memory::special_addresses::WY, 0x00);
        m.insert(memory::special_addresses::WX, 0x00);
        m
    };
}

// DMG initialization sequence
pub fn initialize(gameboy: &mut Gameboy) {
    gameboy.cpu.registers.a = 0x01;
    gameboy.cpu.registers.b = 0x00;
    gameboy.cpu.registers.f.zero = true;
    gameboy.cpu.registers.c = 0x13;
    gameboy.cpu.registers.e = 0xD8;
    gameboy.cpu.registers.h = 0x01;
    gameboy.cpu.registers.l = 0x4D;

    DMG_HARDWARE_REGISTER_INIT
        .iter()
        .for_each(|(address, value)| {
            gameboy.bus.memory[*address] = *value;
        });
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
                // check if keycode is in KEYMAP

                match event {
                    sdl2::event::Event::Quit { .. } => break 'running,
                    sdl2::event::Event::KeyUp { keycode, .. } => {
                        if let Some(button) = KEYMAP.get(&keycode.unwrap()) {
                            self.update_joypad_state(button, false);
                        }
                    }
                    sdl2::event::Event::KeyDown { keycode, .. } => {
                        if let Some(button) = KEYMAP.get(&keycode.unwrap()) {
                            self.update_joypad_state(button, true);
                        }
                    }
                    _ => {}
                }
            }

            let start_frame = Instant::now();
            self.run_frame(&mut ppu);
            let frame_duration = start_frame.elapsed();
            let frame_time = Duration::from_secs_f64(1.0 / 60.0);
            if frame_duration < frame_time {
                std::thread::sleep(frame_time - frame_duration);
            }
            let fps = 1.0 / before.elapsed().as_secs_f64();
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
                let ticks = if self.cpu.halted {
                    4
                } else {
                    self.run_next_instruction()
                };

                self.handle_interrupts();
                self.serial_comm();
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
        // TODO: add 20 clocks to the cycle if an interrupt is triggered
        // 24 if HALTed
        let interrupt_enable = self.bus.memory[IE];
        let interrupt_flags = self.bus.memory[IF];
        let interrupt_value = interrupt_enable & interrupt_flags;

        if interrupt_value != 0 {
            self.cpu.halted = false;

            if self.interrupts_enabled {
                // check if any interrupts are enabled
                log::info!("Interrupt: {:?}", Interrupt::from(interrupt_value));
                // disable interrupts
                self.interrupts_enabled = false;
                let (interrupt, interrupt_handler) = Interrupt::interrupt_address(interrupt_value);
                // disable interrupt
                self.bus.memory[IF] = interrupt_flags & !u8::from(interrupt);
                // push current program counter to stack
                let pc = self.cpu.registers.get_u16(Register16bTarget::PC);
                self.stack_push(pc);
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

    fn get_next_instruction(&mut self) -> &'static instructions::Instruction {
        let address = self.cpu.registers.get_u16(Register16bTarget::PC);

        let mut instruction_byte = self.read_byte(address);

        let opcode_info;
        let instruction;

        if instruction_byte == 0xCB {
            instruction_byte = self.read_byte(address.wrapping_add(1));

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
        self.read_byte(new_address)
    }

    fn update_graphics(&mut self, ticks: u8, ppu: &mut PPU) {
        self.scanline_counter += ticks as u64;
        if self.scanline_counter >= 456 * 4 {
            self.scanline_counter = 0;
            let mut current_scanline = self.bus.memory[LY];
            if current_scanline == 144 {
                self.request_interrupt(Interrupt::VBlank);
            }
            if current_scanline > 153 {
                current_scanline = 0;
            } else {
                current_scanline += 1;
            }
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
            0 => 4096,
            1 => 262144,
            2 => 65536,
            3 => 16382,
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

    pub fn read_byte(&self, address: u16) -> u8 {
        if self.bus.boot_rom_enabled && address < self.bus.boot_rom.len() as u16 {
            return self.bus.boot_rom[address as usize];
        }

        match address as usize {
            0x0000..=0x7FFF => self.bus.cartridge_rom[address as usize],
            special_addresses::P1 => self.get_joypad_state(),
            other => self.bus.memory[other as usize],
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address as usize {
            special_addresses::DMA => {
                // DMA transfer
                let start_address = (value as u16) << 8;
                for i in 0..0xA0 {
                    let byte = self.bus.memory[(start_address + i) as usize];
                    self.bus.memory[0xFE00 + i as usize] = byte;
                }
            }
            special_addresses::DIV => {
                self.bus.memory[special_addresses::DIV] = 0;
            }
            0xFF50 if self.bus.boot_rom_enabled => {
                log::info!("Disabling boot ROM");
                self.bus.boot_rom_enabled = false;
            }
            0x0000..=0x7FFF => {
                log::warn!("Attempted to write to ROM at address {:04X}", address);
                return;
            }
            0xA000..=0xBFFF => {
                log::warn!(
                    "Attempted to write to external RAM at address {:04X}",
                    address
                );
            }
            0xE000..=0xFDFF => {
                log::warn!("Attempted to write to echo RAM at address {:04X}", address);
            }
            0xFEA0..=0xFEFF => {
                log::warn!(
                    "Attempted to write to unusable memory at address {:04X}",
                    address
                );
            }
            _ => {}
        }
        self.bus.memory[address as usize] = value;
    }

    pub fn stack_push(&mut self, pc: u16) {
        let [high, low] = pc.to_be_bytes();
        self.cpu.registers.sp = self.cpu.registers.sp.wrapping_sub(1);
        self.write_byte(self.cpu.registers.sp, high);
        self.cpu.registers.sp = self.cpu.registers.sp.wrapping_sub(1);
        self.write_byte(self.cpu.registers.sp, low);
    }

    fn get_joypad_state(&self) -> u8 {
        if flag_set_at!(self.bus.memory[special_addresses::P1], 4) {
            self.joypad.standard_buttons
        } else {
            self.joypad.directional_keys
        }
    }

    fn update_joypad_state(&mut self, button: &joypad::JoypadButton, pressed: bool) {
        let changed = self.joypad.set_button_state(button, pressed);
        if pressed && changed {
            self.request_interrupt(Interrupt::Joypad);
        }
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
