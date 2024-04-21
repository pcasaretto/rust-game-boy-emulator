use crate::cpu::RegisterTarget;
use crate::gameboy::{self, Gameboy};
use crate::instructions::Register16bTarget;

pub fn dec_r(target: RegisterTarget) -> impl Fn(&mut Gameboy) -> u8 {
    move |gameboy: &mut Gameboy| {
        let current_value = gameboy.cpu.registers.get_u8(target);
        let new_value = current_value.wrapping_sub(1);
        gameboy.cpu.registers.set_u8(target, new_value);

        gameboy.cpu.registers.f.zero = new_value == 0;
        gameboy.cpu.registers.f.subtract = true;
        gameboy.cpu.registers.f.half_carry = current_value & 0x0F == 0;
        const TICKS: u8 = 4;
        TICKS
    }
}

pub fn dec_mem_at_hl(gameboy: &mut Gameboy) -> u8 {
    let address = gameboy.cpu.registers.get_u16(Register16bTarget::HL);
    let current_value = gameboy.bus.read_byte(address);
    let new_value = current_value.wrapping_sub(1);
    gameboy.bus.write_byte(address, new_value);

    gameboy.cpu.registers.f.zero = new_value == 0;
    gameboy.cpu.registers.f.subtract = true;
    gameboy.cpu.registers.f.half_carry = current_value & 0x0F == 0;
    const TICKS: u8 = 12;
    TICKS
}

pub fn dec_r16(target: Register16bTarget) -> impl Fn(&mut Gameboy) -> u8 {
    const TICKS: u8 = 8;
    move |gameboy: &mut Gameboy| {
        let current_value = gameboy.cpu.registers.get_u16(target);
        gameboy
            .cpu
            .registers
            .set_u16(target, current_value.wrapping_sub(1));
        TICKS
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::{FlagsRegister, Register16bTarget, Registers, CPU};

    #[test]
    fn test_dec_mem_at_hl() {
        let mut gameboy = Gameboy::default();
        let addr: u16 = 0xC050;
        gameboy.cpu.registers.set_u16(Register16bTarget::HL, addr);
        gameboy.bus.write_byte(addr, 0x12);
        dec_mem_at_hl(&mut gameboy);
        assert_eq!(gameboy.bus.read_byte(addr), 0x11);
    }

    #[test]
    fn test_dec_mem_at_hl_overflow() {
        let mut gameboy = Gameboy::default();
        let addr: u16 = 0xC050;
        gameboy.cpu.registers.set_u16(Register16bTarget::HL, addr);
        gameboy.bus.write_byte(addr, 0x00);
        dec_mem_at_hl(&mut gameboy);
        assert_eq!(gameboy.bus.read_byte(addr), 0xFF);
    }

    #[test]
    fn test_dec_mem_at_hl_zero() {
        let mut gameboy = Gameboy::default();
        let addr: u16 = 0xC050;
        gameboy.cpu.registers.set_u16(Register16bTarget::HL, addr);
        gameboy.bus.write_byte(addr, 0x01);
        dec_mem_at_hl(&mut gameboy);
        assert!(gameboy.cpu.registers.f.zero);
    }

    #[test]
    fn test_dec_mem_at_hl_half_carry() {
        let mut gameboy = Gameboy::default();
        let addr: u16 = 0xC050;
        gameboy.cpu.registers.set_u16(Register16bTarget::HL, addr);
        gameboy.bus.write_byte(addr, 0x60);
        // Set if no borrow from bit 4.
        dec_mem_at_hl(&mut gameboy);
        assert!(gameboy.cpu.registers.f.half_carry);
    }

    #[test]
    fn test_dec_r() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    b: 2,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        dec_r(RegisterTarget::B)(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.b, 1);
    }

    #[test]
    fn test_dec_r_overflow() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    b: 0,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        dec_r(RegisterTarget::B)(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.b, 0xFF);
    }

    #[test]
    fn test_dec_r_subtract_flag() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    f: FlagsRegister {
                        subtract: false,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        dec_r(RegisterTarget::B)(&mut gameboy);
        assert!(gameboy.cpu.registers.f.subtract);
    }

    #[test]
    fn test_dec_r_half_carry_flag() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.set_u8(RegisterTarget::B, 0x60);
        dec_r(RegisterTarget::B)(&mut gameboy);
        assert!(gameboy.cpu.registers.f.half_carry);

        gameboy.cpu.registers.set_u8(RegisterTarget::B, 0x10);
        dec_r(RegisterTarget::B)(&mut gameboy);
        assert!(gameboy.cpu.registers.f.half_carry);

        gameboy.cpu.registers.set_u8(RegisterTarget::B, 0x11);
        dec_r(RegisterTarget::B)(&mut gameboy);
        assert!(!gameboy.cpu.registers.f.half_carry);
    }

    #[test]
    fn test_dec_r_carry_flag() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    b: 0,
                    f: FlagsRegister::from(0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        dec_r(RegisterTarget::B)(&mut gameboy);
        assert!(!gameboy.cpu.registers.f.carry);
    }
}
