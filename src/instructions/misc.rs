use crate::gameboy::Gameboy;

pub fn stop(gameboy: &mut Gameboy) -> u8 {
    //TODO: stop until button pressed
    const TICKS: u8 = 4;
    TICKS
}

pub fn daa(gameboy: &mut Gameboy) -> u8 {
    // note: assumes a is a uint8_t and wraps from 0xff to 0
    // if (!n_flag) {  // after an addition, adjust if (half-)carry occurred or if result is out of bounds
    //     if (c_flag || a > 0x99) { a += 0x60; c_flag = 1; }
    //     if (h_flag || (a & 0x0f) > 0x09) { a += 0x6; }
    //   } else {  // after a subtraction, only adjust if (half-)carry occurred
    //     if (c_flag) { a -= 0x60; }
    //     if (h_flag) { a -= 0x6; }
    //   }
    //   // these flags are always updated
    //   z_flag = (a == 0); // the usual z flag
    //   h_flag = 0; // h flag is always cleared
    let value = gameboy.cpu.registers.a;
    let flags = gameboy.cpu.registers.f;
    if flags.subtract {
        if flags.carry {
            gameboy.cpu.registers.a = value.wrapping_sub(0x60);
        }
        if flags.half_carry {
            gameboy.cpu.registers.a = value.wrapping_sub(0x6);
        }
    } else {
        if flags.carry || value > 0x99 {
            gameboy.cpu.registers.a = value.wrapping_add(0x60);
            gameboy.cpu.registers.f.carry = true;
        }
        if flags.half_carry || (value & 0x0f) > 0x09 {
            gameboy.cpu.registers.a = value.wrapping_add(0x6);
        }
    }
    gameboy.cpu.registers.f.zero = gameboy.cpu.registers.a == 0;
    gameboy.cpu.registers.f.half_carry = false;
    const TICKS: u8 = 4;
    TICKS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_daa() {
        let mut gameboy = Gameboy::default();

        daa(&mut gameboy);
    }
}
