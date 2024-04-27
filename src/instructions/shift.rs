use crate::cpu::{Register16bTarget, RegisterTarget};
use crate::gameboy::Gameboy;

pub fn sla(target: RegisterTarget) -> impl Fn(&mut Gameboy) -> u8 {
    move |gameboy: &mut Gameboy| {
        let value = gameboy.cpu.registers.get_u8(target);
        let result = value << 1;

        gameboy.cpu.registers.set_u8(target, result);

        gameboy.cpu.registers.f.zero = result == 0;
        gameboy.cpu.registers.f.subtract = false;
        gameboy.cpu.registers.f.half_carry = false;
        gameboy.cpu.registers.f.carry = value & 0b1000_0000 != 0;
        const TICKS: u8 = 8;
        TICKS
    }
}

pub fn sla_mem_at_hl(gameboy: &mut Gameboy) -> u8 {
    let addr = gameboy.cpu.registers.get_u16(Register16bTarget::HL);
    let value = gameboy.bus.read_byte(addr);
    let result = value << 1;

    gameboy.bus.write_byte(addr, result);

    gameboy.cpu.registers.f.zero = result == 0;
    gameboy.cpu.registers.f.subtract = false;
    gameboy.cpu.registers.f.half_carry = false;
    gameboy.cpu.registers.f.carry = value & 0b1000_0000 != 0;
    const TICKS: u8 = 16;
    TICKS
}

pub fn sra(target: RegisterTarget) -> impl Fn(&mut Gameboy) -> u8 {
    move |gameboy: &mut Gameboy| {
        let value = gameboy.cpu.registers.get_u8(target);
        let result = (value >> 1) | (value & 0b1000_0000);

        gameboy.cpu.registers.set_u8(target, result);

        gameboy.cpu.registers.f.zero = result == 0;
        gameboy.cpu.registers.f.subtract = false;
        gameboy.cpu.registers.f.half_carry = false;
        gameboy.cpu.registers.f.carry = value & 0b0000_0001 != 0;
        const TICKS: u8 = 8;
        TICKS
    }
}

pub fn sra_mem_at_hl(gameboy: &mut Gameboy) -> u8 {
    let addr = gameboy.cpu.registers.get_u16(Register16bTarget::HL);
    let value = gameboy.bus.read_byte(addr);
    let result = (value >> 1) | (value & 0b1000_0000);

    gameboy.bus.write_byte(addr, result);

    gameboy.cpu.registers.f.zero = result == 0;
    gameboy.cpu.registers.f.subtract = false;
    gameboy.cpu.registers.f.half_carry = false;
    gameboy.cpu.registers.f.carry = value & 0b0000_0001 != 0;
    const TICKS: u8 = 16;
    TICKS
}

pub fn srl(target: RegisterTarget) -> impl Fn(&mut Gameboy) -> u8 {
    move |gameboy: &mut Gameboy| {
        let value = gameboy.cpu.registers.get_u8(target);
        let result = value >> 1;

        gameboy.cpu.registers.set_u8(target, result);

        gameboy.cpu.registers.f.zero = result == 0;
        gameboy.cpu.registers.f.subtract = false;
        gameboy.cpu.registers.f.half_carry = false;
        gameboy.cpu.registers.f.carry = value & 0b0000_0001 != 0;
        const TICKS: u8 = 8;
        TICKS
    }
}

pub fn srl_mem_at_hl(gameboy: &mut Gameboy) -> u8 {
    let addr = gameboy.cpu.registers.get_u16(Register16bTarget::HL);
    let value = gameboy.bus.read_byte(addr);
    let result = value >> 1;

    gameboy.bus.write_byte(addr, result);

    gameboy.cpu.registers.f.zero = result == 0;
    gameboy.cpu.registers.f.subtract = false;
    gameboy.cpu.registers.f.half_carry = false;
    gameboy.cpu.registers.f.carry = value & 0b0000_0001 != 0;
    const TICKS: u8 = 16;
    TICKS
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sla() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.set_u8(RegisterTarget::A, 0b1100_1100);
        let cycles = sla(RegisterTarget::A)(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.get_u8(RegisterTarget::A), 0b1001_1000);
        assert!(!gameboy.cpu.registers.f.zero);
        assert!(!gameboy.cpu.registers.f.subtract);
        assert!(!gameboy.cpu.registers.f.half_carry);
        assert!(gameboy.cpu.registers.f.carry);
        assert_eq!(cycles, 8);
    }

    #[test]
    fn test_sla_mem_at_hl() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.set_u16(Register16bTarget::HL, 0xC050);
        gameboy.bus.write_byte(0xC050, 0b1100_1100);
        let cycles = sla_mem_at_hl(&mut gameboy);
        assert_eq!(gameboy.bus.read_byte(0xC050), 0b1001_1000);
        assert!(!gameboy.cpu.registers.f.zero);
        assert!(!gameboy.cpu.registers.f.subtract);
        assert!(!gameboy.cpu.registers.f.half_carry);
        assert!(gameboy.cpu.registers.f.carry);
        assert_eq!(cycles, 16);
    }

    #[test]
    fn test_sra() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.set_u8(RegisterTarget::A, 0b1100_1101);
        let cycles = sra(RegisterTarget::A)(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.get_u8(RegisterTarget::A), 0b1110_0110);
        assert!(!gameboy.cpu.registers.f.zero);
        assert!(!gameboy.cpu.registers.f.subtract);
        assert!(!gameboy.cpu.registers.f.half_carry);
        assert!(gameboy.cpu.registers.f.carry);
        assert_eq!(cycles, 8);
    }

    #[test]
    fn test_sra_mem_at_hl() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.set_u16(Register16bTarget::HL, 0xC050);
        gameboy.bus.write_byte(0xC050, 0b1100_1101);
        let cycles = sra_mem_at_hl(&mut gameboy);
        assert_eq!(gameboy.bus.read_byte(0xC050), 0b1110_0110);
        assert!(!gameboy.cpu.registers.f.zero);
        assert!(!gameboy.cpu.registers.f.subtract);
        assert!(!gameboy.cpu.registers.f.half_carry);
        assert!(gameboy.cpu.registers.f.carry);
        assert_eq!(cycles, 16);
    }

    #[test]
    fn test_srl() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.set_u8(RegisterTarget::A, 0b1100_1101);
        let cycles = srl(RegisterTarget::A)(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.get_u8(RegisterTarget::A), 0b0110_0110);
        assert!(!gameboy.cpu.registers.f.zero);
        assert!(!gameboy.cpu.registers.f.subtract);
        assert!(!gameboy.cpu.registers.f.half_carry);
        assert!(gameboy.cpu.registers.f.carry);
        assert_eq!(cycles, 8);
    }

    #[test]
    fn test_srl_mem_at_hl() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.set_u16(Register16bTarget::HL, 0xC050);
        gameboy.bus.write_byte(0xC050, 0b1100_1101);
        let cycles = srl_mem_at_hl(&mut gameboy);
        assert_eq!(gameboy.bus.read_byte(0xC050), 0b0110_0110);
        assert!(!gameboy.cpu.registers.f.zero);
        assert!(!gameboy.cpu.registers.f.subtract);
        assert!(!gameboy.cpu.registers.f.half_carry);
        assert!(gameboy.cpu.registers.f.carry);
        assert_eq!(cycles, 16);
    }
}
