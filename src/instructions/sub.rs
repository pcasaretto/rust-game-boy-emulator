use crate::cpu::{Register16bTarget, RegisterTarget};
use crate::gameboy::Gameboy;

pub fn sub_r_r_a(target: RegisterTarget) -> impl Fn(&mut Gameboy) {
    move |gameboy: &mut Gameboy| {
        let target_value = gameboy.cpu.registers.get_u8(target);
        let current_value = gameboy.cpu.registers.get_u8(RegisterTarget::A);
        let (new_value, did_overflow) = current_value.overflowing_sub(target_value);
        gameboy.cpu.registers.a = new_value;

        gameboy.cpu.registers.f.carry = did_overflow;
        gameboy.cpu.registers.f.zero = new_value == 0;
        gameboy.cpu.registers.f.subtract = true;
        gameboy.cpu.registers.f.half_carry = (current_value & 0xF) < (target_value & 0xF);
    }
}

pub fn sub_d8() -> impl Fn(&mut Gameboy) {
    move |gameboy: &mut Gameboy| {
        let d8 = gameboy.bus.memory[gameboy.cpu.registers.get_u16(Register16bTarget::PC) as usize];
        let current_value = gameboy.cpu.registers.get_u8(RegisterTarget::A);
        let (new_value, did_overflow) = current_value.overflowing_sub(d8);
        gameboy.cpu.registers.a = new_value;

        gameboy.cpu.registers.f.carry = did_overflow;
        gameboy.cpu.registers.f.zero = new_value == 0;
        gameboy.cpu.registers.f.subtract = true;
        gameboy.cpu.registers.f.half_carry = (current_value & 0xF) < (d8 & 0xF);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::{FlagsRegister, Registers, CPU};

    #[test]
    fn test_sub() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    a: 4,
                    c: 1,
                    f: FlagsRegister::from(0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        sub_r_r_a(RegisterTarget::C)(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.a, 3);
    }

    #[test]
    fn test_sub_overflow() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    a: 4,
                    c: 5,
                    f: FlagsRegister::from(0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        sub_r_r_a(RegisterTarget::C)(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.a, 255);
    }

    #[test]
    fn test_sub_carry_flag() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    a: 4,
                    c: 5,
                    f: FlagsRegister::from(0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        sub_r_r_a(RegisterTarget::C)(&mut gameboy);
        assert!(gameboy.cpu.registers.f.carry);
    }

    #[test]
    fn test_sub_zero_flag() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    a: 5,
                    c: 5,
                    f: FlagsRegister::from(0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        sub_r_r_a(RegisterTarget::C)(&mut gameboy);
        assert!(gameboy.cpu.registers.f.zero);
    }

    #[test]
    fn test_sub_substract_flag() {
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
        sub_r_r_a(RegisterTarget::C)(&mut gameboy);
        assert!(gameboy.cpu.registers.f.subtract);
    }

    #[test]
    fn test_sub_half_carry_flag() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    a: 0b00010000,
                    c: 0b00000001,
                    f: FlagsRegister::from(0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        sub_r_r_a(RegisterTarget::C)(&mut gameboy);
        assert!(gameboy.cpu.registers.f.half_carry);
    }

    #[test]
    fn test_sub_d8() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    a: 4,
                    pc: 0x0012,
                    f: FlagsRegister::from(0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        gameboy.bus.memory[0x0012] = 1;
        sub_d8()(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.a, 3);
    }

    #[test]
    fn test_sub_d8_overflow() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    pc: 0x0012,
                    a: 4,
                    f: FlagsRegister::from(0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        gameboy.bus.memory[0x0012] = 5;
        sub_d8()(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.a, 255);
    }

    #[test]
    fn test_sub_d8_carry_flag() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    a: 4,
                    pc: 0x0012,
                    f: FlagsRegister::from(0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        gameboy.bus.memory[0x0012] = 5;
        sub_d8()(&mut gameboy);
        assert!(gameboy.cpu.registers.f.carry);
    }

    #[test]
    fn test_sub_d8_zero_flag() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    pc: 0x0012,
                    a: 5,
                    f: FlagsRegister::from(0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        gameboy.bus.memory[0x0012] = 5;
        sub_d8()(&mut gameboy);
        assert!(gameboy.cpu.registers.f.zero);
    }

    #[test]
    fn test_sub_d8_substract_flag() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    pc: 0x0012,
                    a: 5,
                    f: FlagsRegister::from(0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        gameboy.bus.memory[0x0012] = 5;
        sub_d8()(&mut gameboy);
        assert!(gameboy.cpu.registers.f.subtract);
    }

    #[test]
    fn test_sub_d8_half_carry_flag() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    pc: 0x0012,
                    a: 0b00010000,
                    f: FlagsRegister::from(0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        gameboy.bus.memory[0x0012] = 1;
        sub_d8()(&mut gameboy);
        assert!(gameboy.cpu.registers.f.half_carry);
    }
}
