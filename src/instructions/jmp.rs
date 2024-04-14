use crate::gameboy::Gameboy;

pub fn jmp_a16(gameboy: &mut Gameboy) -> u8 {
    let low = gameboy.read_next_byte();
    let high = gameboy.read_next_byte();
    gameboy.cpu.registers.pc = u16::from_be_bytes([high, low]);
    const TICKS: u8 = 16;
    return TICKS;
}

pub fn jr_z(gameboy: &mut Gameboy) -> u8 {
    if !gameboy.cpu.registers.f.zero {
        gameboy.cpu.registers.pc = gameboy.cpu.registers.pc.wrapping_add(2);
        const TICKS: u8 = 8;
        return TICKS;
    }
    let offset = gameboy.read_next_byte();
    gameboy.cpu.registers.pc = gameboy
        .cpu
        .registers
        .pc
        .wrapping_add(offset as i8 as u16 + 1);
    const TICKS: u8 = 12;
    return TICKS;
}

pub fn jr_nz(gameboy: &mut Gameboy) -> u8 {
    if gameboy.cpu.registers.f.zero {
        gameboy.cpu.registers.pc = gameboy.cpu.registers.pc.wrapping_add(2);
        const TICKS: u8 = 8;
        return TICKS;
    }
    let offset = gameboy.read_next_byte();
    gameboy.cpu.registers.pc = gameboy
        .cpu
        .registers
        .pc
        .wrapping_add(offset as i8 as u16 + 1);
    const TICKS: u8 = 12;
    return TICKS;
}

pub fn jr_nc(gameboy: &mut Gameboy) -> u8 {
    if gameboy.cpu.registers.f.carry {
        gameboy.cpu.registers.pc = gameboy.cpu.registers.pc.wrapping_add(2);
        const TICKS: u8 = 8;
        return TICKS;
    }
    let offset = gameboy.read_next_byte();
    gameboy.cpu.registers.pc = gameboy
        .cpu
        .registers
        .pc
        .wrapping_add(offset as i8 as u16 + 1);
    const TICKS: u8 = 12;
    return TICKS;
}

pub fn jr_c(gameboy: &mut Gameboy) -> u8 {
    if !gameboy.cpu.registers.f.carry {
        gameboy.cpu.registers.pc = gameboy.cpu.registers.pc.wrapping_add(2);
        const TICKS: u8 = 8;
        return TICKS;
    }
    let offset = gameboy.read_next_byte();
    gameboy.cpu.registers.pc = gameboy
        .cpu
        .registers
        .pc
        .wrapping_add(offset as i8 as u16 + 1);
    const TICKS: u8 = 12;
    return TICKS;
}

pub fn jr(gameboy: &mut Gameboy) -> u8 {
    const TICKS: u8 = 12;
    let offset = gameboy.read_next_byte();
    gameboy.cpu.registers.pc = gameboy
        .cpu
        .registers
        .pc
        .wrapping_add(offset as i8 as u16 + 1);
    TICKS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jmp_a16() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.pc = 0xC050;
        gameboy.bus.memory[0xC051] = 0x01;
        gameboy.bus.memory[0xC052] = 0x02;
        jmp_a16(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.pc, 0x0201);
    }

    #[test]
    fn test_jr() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.pc = 0xC050;
        gameboy.bus.memory[0xC051] = 0x05;
        jr(&mut gameboy);
        // jump is relative to next instruction
        assert_eq!(gameboy.cpu.registers.pc, 0xC057);
    }

    #[test]
    fn test_jr_signed_negative() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.pc = 0xC050;
        gameboy.bus.memory[0xC051] = -5i8 as u8;
        jr(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.pc, 0xC04D);
    }

    #[test]
    fn test_jr_z_flag_unset() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.pc = 0xC050;
        gameboy.cpu.registers.f.zero = false;
        gameboy.bus.memory[0xC051] = 0x01;
        jr_z(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.pc, 0xC052);
    }

    #[test]
    fn test_jr_z_flag_set() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.pc = 0xC050;
        gameboy.cpu.registers.f.zero = true;
        gameboy.bus.memory[0xC051] = 0x05;
        jr_z(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.pc, 0xC057);
    }

    #[test]
    fn test_jr_z_flag_set_signed_negative() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.pc = 0xC050;
        gameboy.cpu.registers.f.zero = true;
        gameboy.bus.memory[0xC051] = -5i8 as u8;
        jr_z(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.pc, 0xC04D);
    }

    #[test]
    fn test_jr_nz_flag_set() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.pc = 0xC050;
        gameboy.cpu.registers.f.zero = true;
        gameboy.bus.memory[0xC050] = 0x01;
        jr_nz(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.pc, 0xC052);
    }

    #[test]
    fn test_jr_nz_flag_unset_signed_negative() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.pc = 0xC050;
        gameboy.cpu.registers.f.zero = false;
        gameboy.bus.memory[0xC051] = -5 as i8 as u8;
        jr_nz(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.pc, 0xC04D);
    }

    #[test]
    fn test_jr_c_flag_unset() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.pc = 0xC050;
        gameboy.cpu.registers.f.carry = false;
        gameboy.bus.memory[0xC050] = 0x01;
        jr_c(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.pc, 0xC052);
    }

    #[test]
    fn test_jr_c_flag_set() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.pc = 0xC050;
        gameboy.cpu.registers.f.carry = true;
        gameboy.bus.memory[0xC051] = 0x05;
        jr_c(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.pc, 0xC057);
    }

    #[test]
    fn test_jr_c_flag_set_signed_negative() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.pc = 0xC050;
        gameboy.cpu.registers.f.carry = true;
        gameboy.bus.memory[0xC051] = -5i8 as u8;
        jr_c(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.pc, 0xC04D);
    }

    #[test]
    fn test_jr_nc_flag_unset() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.pc = 0xC050;
        gameboy.cpu.registers.f.carry = false;
        gameboy.bus.memory[0xC051] = 0x05;
        jr_nc(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.pc, 0xC057);
    }

    #[test]
    fn test_jr_nc_flag_set() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.pc = 0xC050;
        gameboy.cpu.registers.f.carry = true;
        gameboy.bus.memory[0xC051] = 0x05;
        jr_nc(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.pc, 0xC052);
    }

    #[test]
    fn test_jr_nc_flag_unset_signed_negative() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.pc = 0xC050;
        gameboy.cpu.registers.f.carry = false;
        gameboy.bus.memory[0xC051] = -5i8 as u8;
        jr_nc(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.pc, 0xC04D);
    }
}
