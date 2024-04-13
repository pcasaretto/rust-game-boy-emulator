use crate::cpu::RegisterTarget;
use crate::gameboy::Gameboy;

pub fn dec_r(target: RegisterTarget) -> impl Fn(&mut Gameboy) -> u8 {
    const TICKS: u8 = 4;
    move |gameboy: &mut Gameboy| {
        let current_value = gameboy.cpu.registers.get_u8(target);
        let (new_value, did_overflow) = current_value.overflowing_sub(1);
        gameboy.cpu.registers.set_u8(target, new_value);

        gameboy.cpu.registers.f.zero = new_value == 0;
        gameboy.cpu.registers.f.subtract = true;
        gameboy.cpu.registers.f.half_carry = current_value & 0x10 == 0x10;
        gameboy.cpu.registers.f.carry = did_overflow;
        TICKS
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::{FlagsRegister, Registers, CPU};

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
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    b: 0x10,
                    f: FlagsRegister::from(0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        dec_r(RegisterTarget::B)(&mut gameboy);
        assert!(gameboy.cpu.registers.f.half_carry);
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
        assert!(gameboy.cpu.registers.f.carry);
    }
}
