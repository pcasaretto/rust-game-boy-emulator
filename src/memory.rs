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
    pub const NR10: usize = 0xFF10;
    pub const NR11: usize = 0xFF11;
    pub const NR12: usize = 0xFF12;
    pub const NR13: usize = 0xFF13;
    pub const NR14: usize = 0xFF14;
    pub const NR21: usize = 0xFF16;
    pub const NR22: usize = 0xFF17;
    pub const NR23: usize = 0xFF18;
    pub const NR24: usize = 0xFF19;
    pub const NR30: usize = 0xFF1A;
    pub const NR31: usize = 0xFF1B;
    pub const NR32: usize = 0xFF1C;
    pub const NR33: usize = 0xFF1D;
    pub const NR34: usize = 0xFF1E;
    pub const NR41: usize = 0xFF20;
    pub const NR42: usize = 0xFF21;
    pub const NR43: usize = 0xFF22;
    pub const NR44: usize = 0xFF23;
    pub const NR50: usize = 0xFF24;
    pub const NR51: usize = 0xFF25;
    pub const NR52: usize = 0xFF26;
}
