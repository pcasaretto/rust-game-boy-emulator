pub struct MemoryBus<'a> {
    pub memory: [u8; 0x10000],
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

impl<'a> MemoryBus<'a> {
    pub fn read_byte(&self, address: u16) -> u8 {
        if self.boot_rom_enabled && address < self.boot_rom.len() as u16 {
            return self.boot_rom[address as usize];
        }
        let value = match address {
            0x0000..=0x3FFF => self.cartridge_rom[address as usize],
            other => self.memory[other as usize],
        };
        log::debug!("Read from {:04X}: value {:02X}", address, value);
        value
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        log::info!("Writing {:02X} to {:04X}", value, address);
        match address {
            0xFF46 => {
                // DMA transfer
                let start_address = (value as u16) << 8;
                for i in 0..0xA0 {
                    let byte = self.memory[(start_address + i) as usize];
                    self.memory[0xFE00 + i as usize] = byte;
                }
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
