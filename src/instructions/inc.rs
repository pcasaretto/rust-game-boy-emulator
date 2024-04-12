use crate::cpu::{Register16bTarget, RegisterTarget};
use crate::gameboy::Gameboy;

pub fn inc_r(target: RegisterTarget) -> impl Fn(&mut Gameboy) {
    move |gameboy: &mut Gameboy| {
        let current_value = gameboy.cpu.registers.get_u8(target);
        let (new_value, did_overflow) = current_value.overflowing_add(1);
        gameboy.cpu.registers.set_u8(target, new_value);

        gameboy.cpu.registers.f.zero = new_value == 0;
        gameboy.cpu.registers.f.subtract = false;
        gameboy.cpu.registers.f.half_carry = current_value & 0xF == 0xF;
        gameboy.cpu.registers.f.carry = did_overflow;
    }
}

pub fn inc_r16(target: Register16bTarget) -> impl Fn(&mut Gameboy) {
    move |gameboy: &mut Gameboy| {
        let current_value = gameboy.cpu.registers.get_u16(target);
        let new_value = current_value.wrapping_add(1);
        gameboy.cpu.registers.set_u16(target, new_value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::{FlagsRegister, Registers, CPU};

    #[test]
    fn test_inc_r() {
        let mut gameboy = Gameboy::default();
        inc_r(RegisterTarget::B)(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.b, 1);
    }

    #[test]
    fn test_inc_r_overflow() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    b: 0xFF,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        inc_r(RegisterTarget::B)(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.b, 0);
    }

    #[test]
    fn test_inc_r_subtract_flag() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    f: FlagsRegister {
                        subtract: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        inc_r(RegisterTarget::B)(&mut gameboy);
        assert!(!gameboy.cpu.registers.f.subtract);
    }

    #[test]
    fn test_inc_r_half_carry_flag() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    b: 0x0F,
                    f: FlagsRegister::from(0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        inc_r(RegisterTarget::B)(&mut gameboy);
        assert!(gameboy.cpu.registers.f.half_carry);
    }

    #[test]
    fn test_inc_r_carry_flag() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    b: 0xFF,
                    f: FlagsRegister::from(0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        inc_r(RegisterTarget::B)(&mut gameboy);
        assert!(gameboy.cpu.registers.f.carry);
    }
}
