use crate::gameboy::Gameboy;

pub fn jmp_a16() -> impl Fn(&mut Gameboy) -> u8 {
    move |gameboy: &mut Gameboy| {
        let low = gameboy.read_next_byte();
        let high = gameboy.read_next_byte();
        gameboy.cpu.registers.pc = u16::from_le_bytes([low, high]);
        const TICKS: u8 = 16;
        return TICKS;
    }
}

pub fn jr_z(gameboy: &mut Gameboy) -> u8 {
    if !gameboy.cpu.registers.f.zero {
        const TICKS: u8 = 8;
        return TICKS;
    }
    let offset = gameboy.bus.memory[gameboy.cpu.registers.pc as usize];
    gameboy.cpu.registers.pc = gameboy.cpu.registers.pc.wrapping_add(offset as i8 as u16);
    const TICKS: u8 = 12;
    return TICKS;
}

pub fn jr_nz(gameboy: &mut Gameboy) -> u8 {
    if gameboy.cpu.registers.f.zero {
        const TICKS: u8 = 8;
        return TICKS;
    }
    let offset = gameboy.bus.memory[gameboy.cpu.registers.pc as usize];
    gameboy.cpu.registers.pc = gameboy.cpu.registers.pc.wrapping_add(offset as i8 as u16);
    const TICKS: u8 = 12;
    return TICKS;
}

pub fn jr_nc(gameboy: &mut Gameboy) -> u8 {
    if gameboy.cpu.registers.f.carry {
        const TICKS: u8 = 8;
        return TICKS;
    }
    let offset = gameboy.bus.memory[gameboy.cpu.registers.pc as usize];
    gameboy.cpu.registers.pc = gameboy.cpu.registers.pc.wrapping_add(offset as i8 as u16);
    const TICKS: u8 = 12;
    return TICKS;
}

pub fn jr_c(gameboy: &mut Gameboy) -> u8 {
    if !gameboy.cpu.registers.f.carry {
        const TICKS: u8 = 8;
        return TICKS;
    }
    let offset = gameboy.bus.memory[gameboy.cpu.registers.pc as usize];
    gameboy.cpu.registers.pc = gameboy.cpu.registers.pc.wrapping_add(offset as i8 as u16);
    const TICKS: u8 = 12;
    return TICKS;
}

pub fn jr(gameboy: &mut Gameboy) -> u8 {
    const TICKS: u8 = 12;
    let offset = gameboy.bus.memory[gameboy.cpu.registers.pc as usize];
    gameboy.cpu.registers.pc = gameboy.cpu.registers.pc.wrapping_add(offset as i8 as u16);
    TICKS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jmp_a16() {
        let mut gameboy = Gameboy::default();
        gameboy.bus.memory[0] = 0x01;
        gameboy.bus.memory[1] = 0x02;
        jmp_a16()(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.pc, 0x0201);
    }

    #[test]
    fn test_jr() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.pc = 0x1000;
        gameboy.bus.memory[0x1000] = 0x05;
        jr(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.pc, 0x1005);
    }

    #[test]
    fn test_jr_signed_negative() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.pc = 0x1000;
        gameboy.bus.memory[0x1000] = -5i8 as u8;
        jr(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.pc, 0x0FFB);
    }

    #[test]
    fn test_jr_z_flag_unset() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.pc = 0x1000;
        gameboy.cpu.registers.f.zero = false;
        gameboy.bus.memory[0x1000] = 0x01;
        jr_z(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.pc, 0x1000);
    }

    #[test]
    fn test_jr_z_flag_set() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.pc = 0x1000;
        gameboy.cpu.registers.f.zero = true;
        gameboy.bus.memory[0x1000] = 0x05;
        jr_z(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.pc, 0x1005);
    }

    #[test]
    fn test_jr_z_flag_set_signed_negative() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.pc = 0x1000;
        gameboy.cpu.registers.f.zero = true;
        gameboy.bus.memory[0x1005] = -5i8 as u8;
        jr_z(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.pc, 0x1000);
    }

    #[test]
    fn test_jr_nz_flag_unset() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.pc = 0x1000;
        gameboy.cpu.registers.f.zero = false;
        gameboy.bus.memory[0x1000] = 0x01;
        jr_nz(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.pc, 0x1001);
    }

    #[test]
    fn test_jr_nz_flag_unset_signed_negative() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.pc = 0x1000;
        gameboy.cpu.registers.f.zero = false;
        gameboy.bus.memory[0x1005] = -5i8 as u8;
        jr_nz(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.pc, 0x1000);
    }

    #[test]
    fn test_jr_nz_flag_set() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.pc = 0x1000;
        gameboy.cpu.registers.f.zero = true;
        gameboy.bus.memory[0x1000] = 0x05;
        jr_nz(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.pc, 0x1000);
    }

    #[test]
    fn test_jr_c_flag_unset() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.pc = 0x1000;
        gameboy.cpu.registers.f.carry = false;
        gameboy.bus.memory[0x1000] = 0x01;
        jr_c(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.pc, 0x1000);
    }

    #[test]
    fn test_jr_c_flag_set() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.pc = 0x1000;
        gameboy.cpu.registers.f.carry = true;
        gameboy.bus.memory[0x1000] = 0x05;
        jr_c(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.pc, 0x1005);
    }

    #[test]
    fn test_jr_c_flag_set_signed_negative() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.pc = 0x1000;
        gameboy.cpu.registers.f.carry = true;
        gameboy.bus.memory[0x1005] = -5i8 as u8;
        jr_c(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.pc, 0x1000);
    }

    #[test]
    fn test_jr_nc_flag_unset() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.pc = 0x1000;
        gameboy.cpu.registers.f.carry = false;
        gameboy.bus.memory[0x1000] = 0x01;
        jr_nc(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.pc, 0x1001);
    }

    #[test]
    fn test_jr_nc_flag_set() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.pc = 0x1000;
        gameboy.cpu.registers.f.carry = true;
        gameboy.bus.memory[0x1000] = 0x05;
        jr_nc(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.pc, 0x1000);
    }

    #[test]
    fn test_jr_nc_flag_unset_signed_negative() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.pc = 0x1000;
        gameboy.cpu.registers.f.carry = false;
        gameboy.bus.memory[0x1005] = -5i8 as u8;
        jr_nc(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.pc, 0x1000);
    }
}
