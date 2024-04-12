use crate::gameboy;

pub fn run(cartridge: &[u8; 0x200000]) {
    let mut gameboy = gameboy::Gameboy::default();
    gameboy.run(cartridge);
}
