use crate::cpu::Register16bTarget;
use crate::cpu::RegisterTarget;
use crate::gameboy::Gameboy;

pub fn set_mem_at_hl(bit_position: u8) -> impl Fn(&mut Gameboy) -> u8 {
    move |gameboy: &mut Gameboy| {
        let hl = gameboy.cpu.registers.get_u16(Register16bTarget::HL);
        let value = gameboy.bus.read_byte(hl);
        let result = value | (1 << bit_position);

        gameboy.bus.write_byte(hl, result);
        const CYCLES: u8 = 4;
        CYCLES
    }
}

pub fn set_r(target: RegisterTarget, bit_position: u8) -> impl Fn(&mut Gameboy) -> u8 {
    move |gameboy: &mut Gameboy| {
        let value = gameboy.cpu.registers.get_u8(target);
        let result = value | (1 << bit_position);

        gameboy.cpu.registers.set_u8(target, result);
        const CYCLES: u8 = 4;
        CYCLES
    }
}

pub fn res_mem_at_hl(bit_position: u8) -> impl Fn(&mut Gameboy) -> u8 {
    move |gameboy: &mut Gameboy| {
        let hl = gameboy.cpu.registers.get_u16(Register16bTarget::HL);
        let value = gameboy.bus.read_byte(hl);
        let result = value & !(1 << bit_position);

        gameboy.bus.write_byte(hl, result);
        const CYCLES: u8 = 4;
        CYCLES
    }
}

pub fn res_r(target: RegisterTarget, bit_position: u8) -> impl Fn(&mut Gameboy) -> u8 {
    move |gameboy: &mut Gameboy| {
        let value = gameboy.cpu.registers.get_u8(target);
        let result = value & !(1 << bit_position);

        gameboy.cpu.registers.set_u8(target, result);
        const CYCLES: u8 = 4;
        CYCLES
    }
}

pub fn bit_r(target: RegisterTarget, bit_position: u8) -> impl Fn(&mut Gameboy) -> u8 {
    move |gameboy: &mut Gameboy| {
        let value = gameboy.cpu.registers.get_u8(target);
        let result = value & (1 << bit_position);

        gameboy.cpu.registers.f.zero = result == 0;
        gameboy.cpu.registers.f.subtract = false;
        gameboy.cpu.registers.f.half_carry = true;

        const CYCLES: u8 = 8;
        CYCLES
    }
}

pub fn bit_mem_at_hl(bit_position: u8) -> impl Fn(&mut Gameboy) -> u8 {
    move |gameboy: &mut Gameboy| {
        let addr = gameboy.cpu.registers.get_u16(Register16bTarget::HL);
        let value = gameboy.bus.read_byte(addr);
        let result = value & (1 << bit_position);

        gameboy.cpu.registers.f.zero = result == 0;
        gameboy.cpu.registers.f.subtract = false;
        gameboy.cpu.registers.f.half_carry = true;

        const CYCLES: u8 = 16;
        CYCLES
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_set_mem_at_hl() {
        let mut gameboy = Gameboy::default();
        let addr = 0xC050;
        gameboy.cpu.registers.set_u16(Register16bTarget::HL, addr);
        gameboy.bus.write_byte(addr, 0b00000000);

        set_mem_at_hl(3)(&mut gameboy);

        assert_eq!(gameboy.bus.read_byte(addr), 0b00001000);
    }

    #[test]
    fn test_set_r() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.set_u8(RegisterTarget::A, 0b00000000);
        set_r(RegisterTarget::A, 3)(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.get_u8(RegisterTarget::A), 0b00001000);
    }

    #[test]
    fn test_bit_r() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.set_u8(RegisterTarget::A, 0b00001000);
        bit_r(RegisterTarget::A, 3)(&mut gameboy);
        assert!(!gameboy.cpu.registers.f.zero);
        assert!(!gameboy.cpu.registers.f.subtract);
        assert!(gameboy.cpu.registers.f.half_carry);
    }

    #[test]
    fn test_bit_r_true() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.set_u8(RegisterTarget::A, 0b00000000);
        bit_r(RegisterTarget::A, 3)(&mut gameboy);
        assert!(gameboy.cpu.registers.f.zero);
        assert!(!gameboy.cpu.registers.f.subtract);
        assert!(gameboy.cpu.registers.f.half_carry);
    }

    #[test]
    fn test_bit_mem_at_hl() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.set_u16(Register16bTarget::HL, 0xC050);
        gameboy.bus.write_byte(0xC050, 0b00001000);
        bit_mem_at_hl(3)(&mut gameboy);
        assert!(!gameboy.cpu.registers.f.zero);
        assert!(!gameboy.cpu.registers.f.subtract);
        assert!(gameboy.cpu.registers.f.half_carry);
    }

    #[test]
    fn test_bit_mem_at_hl_true() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.set_u16(Register16bTarget::HL, 0xC050);
        gameboy.bus.write_byte(0xC050, 0b00000000);
        bit_mem_at_hl(3)(&mut gameboy);
        assert!(gameboy.cpu.registers.f.zero);
        assert!(!gameboy.cpu.registers.f.subtract);
        assert!(gameboy.cpu.registers.f.half_carry);
    }
}
