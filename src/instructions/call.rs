use crate::gameboy::Gameboy;

pub fn call_a16(gameboy: &mut Gameboy) -> u8 {
    let [pc_high, pc_low] = gameboy.cpu.registers.pc.wrapping_add(3).to_be_bytes();
    gameboy.cpu.registers.sp = gameboy.cpu.registers.sp.wrapping_sub(1);
    gameboy.bus.write_byte(gameboy.cpu.registers.sp, pc_high);
    gameboy.cpu.registers.sp = gameboy.cpu.registers.sp.wrapping_sub(1);
    gameboy.bus.write_byte(gameboy.cpu.registers.sp, pc_low);

    let low = gameboy.read_next_byte();
    let high = gameboy.read_next_byte();

    gameboy.cpu.registers.pc = u16::from_le_bytes([low, high]);
    const TICKS: u8 = 24;
    TICKS
}

pub fn ret(gameboy: &mut Gameboy) -> u8 {
    let low = gameboy.bus.read_byte(gameboy.cpu.registers.sp);
    gameboy.cpu.registers.sp = gameboy.cpu.registers.sp.wrapping_add(1);
    let high = gameboy.bus.read_byte(gameboy.cpu.registers.sp);
    gameboy.cpu.registers.sp = gameboy.cpu.registers.sp.wrapping_add(1);

    gameboy.cpu.registers.pc = u16::from_le_bytes([low, high]);
    const TICKS: u8 = 16;
    TICKS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_call_a16() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.pc = 0x0301;
        gameboy.cpu.registers.sp = 0xFFFE;
        gameboy.bus.memory[gameboy.cpu.registers.pc as usize + 1] = 0xCD;
        gameboy.bus.memory[gameboy.cpu.registers.pc as usize + 2] = 0xAB;
        call_a16(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.pc, 0xABCD);
        assert_eq!(gameboy.cpu.registers.sp, 0xFFFC);
        assert_eq!(gameboy.bus.memory[0xFFFD], 0x03);
        assert_eq!(gameboy.bus.memory[0xFFFC], 0x04);
    }

    #[test]
    fn test_ret() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.sp = 0xFFFC;
        gameboy.bus.memory[0xFFFC] = 0x01;
        gameboy.bus.memory[0xFFFD] = 0x03;
        ret(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.pc, 0x0301);
        assert_eq!(gameboy.cpu.registers.sp, 0xFFFE);
    }
}
