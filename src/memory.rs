pub struct MemoryBus<'a> {
    pub(super) memory: [u8; 0x10000],
    pub boot_rom: &'static [u8],
    pub cartridge_rom: &'a [u8],
    pub boot_rom_enabled: bool,
}

impl Default for MemoryBus<'_> {
    fn default() -> Self {
        MemoryBus {
            memory: [0; 0x10000],
            boot_rom_enabled: true,
            boot_rom: include_bytes!("dmg.bin"),
            cartridge_rom: &[],
        }
    }
}

pub mod special_addresses {
    pub const P1: usize = 0xFF00;
    pub const SB: usize = 0xFF01;
    pub const SC: usize = 0xFF02;
    pub const DIV: usize = 0xFF04;
    pub const TIMA: usize = 0xFF05;
    pub const TMA: usize = 0xFF06;
    pub const TAC: usize = 0xFF07;
    pub const IF: usize = 0xFF0F;
    pub const LCDC: usize = 0xFF40;
    pub const STAT: usize = 0xFF41;
    pub const SCY: usize = 0xFF42;
    pub const SCX: usize = 0xFF43;
    pub const LY: usize = 0xFF44;
    pub const LYC: usize = 0xFF45;
    pub const DMA: usize = 0xFF46;
    pub const BGP: usize = 0xFF47;
    pub const OBP0: usize = 0xFF48;
    pub const OBP1: usize = 0xFF49;
    pub const WY: usize = 0xFF4A;
    pub const WX: usize = 0xFF4B;
    pub const IE: usize = 0xFFFF;
}

impl<'a> MemoryBus<'a> {
    pub fn read_byte(&self, address: u16) -> u8 {
        if self.boot_rom_enabled && address < self.boot_rom.len() as u16 {
            return self.boot_rom[address as usize];
        }
        let value = match address {
            0x0000..=0x7FFF => self.cartridge_rom[address as usize],
            other => self.memory[other as usize],
        };
        // log::debug!("Read from {:04X}: value {:02X}", address, value);
        value
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        // log::info!("Writing {:02X} to {:04X}", value, address);
        match address as usize {
            special_addresses::DMA => {
                // DMA transfer
                let start_address = (value as u16) << 8;
                for i in 0..0xA0 {
                    let byte = self.memory[(start_address + i) as usize];
                    self.memory[0xFE00 + i as usize] = byte;
                }
            }
            special_addresses::DIV => {
                // Reset the divider register
                self.memory[special_addresses::DIV] = 0;
            }
            0xFF50 if self.boot_rom_enabled => {
                log::info!("Disabling boot ROM");
                self.boot_rom_enabled = false;
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
        self.memory[address as usize] = value;
    }
}
