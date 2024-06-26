use crate::gameboy::Gameboy;

pub fn stop(gameboy: &mut Gameboy) -> u8 {
    //TODO: stop until button pressed
    const TICKS: u8 = 4;
    TICKS
}

pub fn halt(gameboy: &mut Gameboy) -> u8 {
    gameboy.cpu.halted = true;
    const TICKS: u8 = 4;
    TICKS
}

pub fn daa(gameboy: &mut Gameboy) -> u8 {
    let mut value = gameboy.cpu.registers.a;
    let flags = gameboy.cpu.registers.f;
    if flags.subtract {
        if flags.carry {
            value = value.wrapping_sub(0x60);
        }
        if flags.half_carry {
            value = value.wrapping_sub(0x6);
        }
    } else {
        if flags.carry || value > 0x99 {
            value = value.wrapping_add(0x60);
            gameboy.cpu.registers.f.carry = true;
        }
        if flags.half_carry || (value & 0x0f) > 0x09 {
            value = value.wrapping_add(0x6);
        }
    }
    gameboy.cpu.registers.a = value;
    gameboy.cpu.registers.f.zero = value == 0;
    gameboy.cpu.registers.f.half_carry = false;
    const TICKS: u8 = 4;
    TICKS
}

pub fn cpl(gameboy: &mut Gameboy) -> u8 {
    gameboy.cpu.registers.a = !gameboy.cpu.registers.a;
    gameboy.cpu.registers.f.subtract = true;
    gameboy.cpu.registers.f.half_carry = true;
    const TICKS: u8 = 4;
    TICKS
}

pub fn ccf(gameboy: &mut Gameboy) -> u8 {
    gameboy.cpu.registers.f.subtract = false;
    gameboy.cpu.registers.f.half_carry = false;
    gameboy.cpu.registers.f.carry = !gameboy.cpu.registers.f.carry;
    const TICKS: u8 = 4;
    TICKS
}

pub fn scf(gameboy: &mut Gameboy) -> u8 {
    gameboy.cpu.registers.f.subtract = false;
    gameboy.cpu.registers.f.half_carry = false;
    gameboy.cpu.registers.f.carry = true;
    const TICKS: u8 = 4;
    TICKS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_daa() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.a = 0x9A;
        daa(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.a, 0x00);
        assert!(gameboy.cpu.registers.f.zero);
        assert!(gameboy.cpu.registers.f.carry);
    }
}
