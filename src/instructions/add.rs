use crate::cpu::RegisterTarget;
use crate::gameboy::Gameboy;

pub fn add(target: RegisterTarget) -> impl Fn(&mut Gameboy) -> u8 {
    move |gameboy: &mut Gameboy| {
        let target_value = gameboy.cpu.registers.get_u8(target);
        let current_value = gameboy.cpu.registers.a;
        let (new_value, did_overflow) = current_value.overflowing_add(target_value);
        gameboy.cpu.registers.a = new_value;

        gameboy.cpu.registers.f.carry = did_overflow;
        gameboy.cpu.registers.f.zero = new_value == 0;
        gameboy.cpu.registers.f.subtract = false;
        gameboy.cpu.registers.f.half_carry = (current_value & 0xF) + (target_value & 0xF) > 0xF;
        const TICKS: u8 = 4;
        TICKS
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::{FlagsRegister, Registers, CPU};

    #[test]
    fn test_add() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    a: 0,
                    c: 1,
                    f: FlagsRegister::from(0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        add(RegisterTarget::C)(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.a, 1);
    }

    #[test]
    fn test_add_overflow() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    a: 255,
                    c: 1,
                    f: FlagsRegister::from(0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        add(RegisterTarget::C)(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.a, 0);
    }

    #[test]
    fn test_add_carry_flag() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    a: 255,
                    c: 1,
                    f: FlagsRegister::from(0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        add(RegisterTarget::C)(&mut gameboy);
        assert!(gameboy.cpu.registers.f.carry);
    }

    #[test]
    fn test_add_zero_flag() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    a: 0,
                    c: 0,
                    f: FlagsRegister::from(0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        add(RegisterTarget::C)(&mut gameboy);
        assert!(gameboy.cpu.registers.f.zero);
    }

    #[test]
    fn test_add_substract_flag() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    a: 0,
                    c: 1,
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
        add(RegisterTarget::C)(&mut gameboy);
        assert!(!gameboy.cpu.registers.f.subtract);
    }

    #[test]
    fn test_add_half_carry_flag() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    a: 0b00001111,
                    c: 0b00000001,
                    f: FlagsRegister::from(0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        add(RegisterTarget::C)(&mut gameboy);
        assert!(gameboy.cpu.registers.f.half_carry);
    }
}
