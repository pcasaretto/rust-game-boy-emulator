use crate::gameboy::Gameboy;

pub fn jmp_a16() -> impl Fn(&mut Gameboy) {
    move |gameboy: &mut Gameboy| {
        let low = gameboy.bus.memory[(gameboy.cpu.registers.pc) as usize];
        let high = gameboy.bus.memory[(gameboy.cpu.registers.pc + 1) as usize];
        gameboy.cpu.registers.pc = u16::from_le_bytes([low, high]);
    }
}

pub fn jr_z() -> impl Fn(&mut Gameboy) {
    move |gameboy: &mut Gameboy| {
        if !gameboy.cpu.registers.f.zero {
            return;
        }
        let offset = gameboy.bus.memory[gameboy.cpu.registers.pc as usize];
        gameboy.cpu.registers.pc = gameboy.cpu.registers.pc.wrapping_add(offset as i8 as u16);
    }
}

pub fn jr_nz() -> impl Fn(&mut Gameboy) {
    move |gameboy: &mut Gameboy| {
        if gameboy.cpu.registers.f.zero {
            return;
        }
        let offset = gameboy.bus.memory[gameboy.cpu.registers.pc as usize];
        gameboy.cpu.registers.pc = gameboy.cpu.registers.pc.wrapping_add(offset as i8 as u16);
    }
}

pub fn jr_nc() -> impl Fn(&mut Gameboy) {
    move |gameboy: &mut Gameboy| {
        if gameboy.cpu.registers.f.carry {
            return;
        }
        let offset = gameboy.bus.memory[gameboy.cpu.registers.pc as usize];
        gameboy.cpu.registers.pc = gameboy.cpu.registers.pc.wrapping_add(offset as i8 as u16);
    }
}
pub fn jr_c() -> impl Fn(&mut Gameboy) {
    move |gameboy: &mut Gameboy| {
        if !gameboy.cpu.registers.f.carry {
            return;
        }
        let offset = gameboy.bus.memory[gameboy.cpu.registers.pc as usize];
        gameboy.cpu.registers.pc = gameboy.cpu.registers.pc.wrapping_add(offset as i8 as u16);
    }
}

pub fn jr() -> impl Fn(&mut Gameboy) {
    move |gameboy: &mut Gameboy| {
        let offset = gameboy.bus.memory[gameboy.cpu.registers.pc as usize];
        gameboy.cpu.registers.pc = gameboy.cpu.registers.pc.wrapping_add(offset as i8 as u16);
    }
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
        jr()(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.pc, 0x1005);
    }

    #[test]
    fn test_jr_signed_negative() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.pc = 0x1000;
        gameboy.bus.memory[0x1000] = -5i8 as u8;
        jr()(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.pc, 0x0FFB);
    }

    #[test]
    fn test_jr_z_flag_unset() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.pc = 0x1000;
        gameboy.cpu.registers.f.zero = false;
        gameboy.bus.memory[0x1000] = 0x01;
        jr_z()(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.pc, 0x1000);
    }

    #[test]
    fn test_jr_z_flag_set() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.pc = 0x1000;
        gameboy.cpu.registers.f.zero = true;
        gameboy.bus.memory[0x1000] = 0x05;
        jr_z()(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.pc, 0x1005);
    }

    #[test]
    fn test_jr_z_flag_set_signed_negative() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.pc = 0x1000;
        gameboy.cpu.registers.f.zero = true;
        gameboy.bus.memory[0x1005] = -5i8 as u8;
        jr_z()(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.pc, 0x1000);
    }

    #[test]
    fn test_jr_nz_flag_unset() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.pc = 0x1000;
        gameboy.cpu.registers.f.zero = false;
        gameboy.bus.memory[0x1000] = 0x01;
        jr_nz()(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.pc, 0x1001);
    }

    #[test]
    fn test_jr_nz_flag_unset_signed_negative() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.pc = 0x1000;
        gameboy.cpu.registers.f.zero = false;
        gameboy.bus.memory[0x1005] = -5i8 as u8;
        jr_nz()(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.pc, 0x1000);
    }

    #[test]
    fn test_jr_nz_flag_set() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.pc = 0x1000;
        gameboy.cpu.registers.f.zero = true;
        gameboy.bus.memory[0x1000] = 0x05;
        jr_nz()(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.pc, 0x1000);
    }

    #[test]
    fn test_jr_c_flag_unset() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.pc = 0x1000;
        gameboy.cpu.registers.f.carry = false;
        gameboy.bus.memory[0x1000] = 0x01;
        jr_c()(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.pc, 0x1000);
    }

    #[test]
    fn test_jr_c_flag_set() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.pc = 0x1000;
        gameboy.cpu.registers.f.carry = true;
        gameboy.bus.memory[0x1000] = 0x05;
        jr_c()(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.pc, 0x1005);
    }

    #[test]
    fn test_jr_c_flag_set_signed_negative() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.pc = 0x1000;
        gameboy.cpu.registers.f.carry = true;
        gameboy.bus.memory[0x1005] = -5i8 as u8;
        jr_c()(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.pc, 0x1000);
    }

    #[test]
    fn test_jr_nc_flag_unset() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.pc = 0x1000;
        gameboy.cpu.registers.f.carry = false;
        gameboy.bus.memory[0x1000] = 0x01;
        jr_nc()(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.pc, 0x1001);
    }

    #[test]
    fn test_jr_nc_flag_set() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.pc = 0x1000;
        gameboy.cpu.registers.f.carry = true;
        gameboy.bus.memory[0x1000] = 0x05;
        jr_nc()(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.pc, 0x1000);
    }

    #[test]
    fn test_jr_nc_flag_unset_signed_negative() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.pc = 0x1000;
        gameboy.cpu.registers.f.carry = false;
        gameboy.bus.memory[0x1005] = -5i8 as u8;
        jr_nc()(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.pc, 0x1000);
    }
}
