pub struct MemoryBus {
    pub memory: [u8; 0x10000],
}

impl Default for MemoryBus {
    fn default() -> Self {
        MemoryBus {
            // memory: core::array::from_fn(|_| random()),
            memory: [0; 0x10000],
        }
    }
}

impl MemoryBus {
    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        log::debug!("Writing {:02X} to {:04X}", value, address);
        match address {
            0x0000..=0x7FFF => {
                log::warn!("Attempted to write to ROM at address {:04X}", address);
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
