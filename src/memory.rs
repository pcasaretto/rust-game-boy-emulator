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
        self.memory[address as usize] = value;
    }
}
