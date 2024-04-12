use crate::gameboy::Gameboy;

pub fn call_a16() -> impl Fn(&mut Gameboy) {
    move |gameboy: &mut Gameboy| {
        let [pc_high, pc_low] = gameboy.cpu.registers.pc.to_be_bytes();
        gameboy.cpu.registers.sp = gameboy.cpu.registers.sp.wrapping_sub(1);
        gameboy.bus.write_byte(gameboy.cpu.registers.sp, pc_high);
        gameboy.cpu.registers.sp = gameboy.cpu.registers.sp.wrapping_sub(1);
        gameboy.bus.write_byte(gameboy.cpu.registers.sp, pc_low);

        let low = gameboy.read_next_byte();
        let high = gameboy.read_next_byte();

        gameboy.cpu.registers.pc = u16::from_le_bytes([low, high]);
    }
}

pub fn ret() -> impl Fn(&mut Gameboy) {
    move |gameboy: &mut Gameboy| {
        let low = gameboy.bus.read_byte(gameboy.cpu.registers.sp);
        gameboy.cpu.registers.sp = gameboy.cpu.registers.sp.wrapping_add(1);
        let high = gameboy.bus.read_byte(gameboy.cpu.registers.sp);
        gameboy.cpu.registers.sp = gameboy.cpu.registers.sp.wrapping_add(1);

        gameboy.cpu.registers.pc = u16::from_le_bytes([low, high]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_call_a16() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.pc = 0x0301;
        gameboy.cpu.registers.sp = 0xFFFE;
        gameboy.bus.memory[gameboy.cpu.registers.pc as usize] = 0xCD;
        gameboy.bus.memory[gameboy.cpu.registers.pc as usize + 1] = 0xAB;
        call_a16()(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.pc, 0xABCD);
        assert_eq!(gameboy.cpu.registers.sp, 0xFFFC);
        assert_eq!(gameboy.bus.memory[0xFFFD], 0x03);
        assert_eq!(gameboy.bus.memory[0xFFFC], 0x01);
    }

    #[test]
    fn test_ret() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.sp = 0xFFFC;
        gameboy.bus.memory[0xFFFC] = 0x01;
        gameboy.bus.memory[0xFFFD] = 0x03;
        ret()(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.pc, 0x0301);
        assert_eq!(gameboy.cpu.registers.sp, 0xFFFE);
    }
}