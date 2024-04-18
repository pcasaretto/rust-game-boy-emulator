use crate::cpu::{Register16bTarget, RegisterTarget};
use crate::gameboy::Gameboy;

pub fn sbc_r_r_a(target: RegisterTarget) -> impl Fn(&mut Gameboy) -> u8 {
    move |gameboy: &mut Gameboy| {
        let mut target_value = gameboy.cpu.registers.get_u8(target);
        if gameboy.cpu.registers.f.carry {
            target_value += 1;
        }
        let current_value = gameboy.cpu.registers.get_u8(RegisterTarget::A);
        let (new_value, did_overflow) = current_value.overflowing_sub(target_value);
        gameboy.cpu.registers.a = new_value;

        gameboy.cpu.registers.f.carry = did_overflow;
        gameboy.cpu.registers.f.zero = new_value == 0;
        gameboy.cpu.registers.f.subtract = true;
        gameboy.cpu.registers.f.half_carry = (current_value & 0xF) < (target_value & 0xF);
        const TICKS: u8 = 4;
        TICKS
    }
}

pub fn sbc_n8(gameboy: &mut Gameboy) -> u8 {
    let mut d8 = gameboy.bus.memory[gameboy.cpu.registers.get_u16(Register16bTarget::PC) as usize];
    if gameboy.cpu.registers.f.carry {
        d8 += 1;
    }
    let current_value = gameboy.cpu.registers.get_u8(RegisterTarget::A);
    let (new_value, did_overflow) = current_value.overflowing_sub(d8);
    gameboy.cpu.registers.a = new_value;

    gameboy.cpu.registers.f.carry = did_overflow;
    gameboy.cpu.registers.f.zero = new_value == 0;
    gameboy.cpu.registers.f.subtract = true;
    gameboy.cpu.registers.f.half_carry = (current_value & 0xF) < (d8 & 0xF);
    const TICKS: u8 = 8;
    TICKS
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::{FlagsRegister, Registers, CPU};

    #[test]
    fn test_sbc() {
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
        sbc_r_r_a(RegisterTarget::C)(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.a, 3);
    }

    #[test]
    fn test_sbc_carry() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    a: 4,
                    c: 1,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        gameboy.cpu.registers.f.carry = true;
        sbc_r_r_a(RegisterTarget::C)(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.a, 2);
    }

    #[test]
    fn test_sbc_overflow() {
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
        sbc_r_r_a(RegisterTarget::C)(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.a, 255);
    }

    #[test]
    fn test_sbc_carry_flag() {
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
        sbc_r_r_a(RegisterTarget::C)(&mut gameboy);
        assert!(gameboy.cpu.registers.f.carry);
    }

    #[test]
    fn test_sbc_zero_flag() {
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
        sbc_r_r_a(RegisterTarget::C)(&mut gameboy);
        assert!(gameboy.cpu.registers.f.zero);
    }

    #[test]
    fn test_sbc_substract_flag() {
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
        sbc_r_r_a(RegisterTarget::C)(&mut gameboy);
        assert!(gameboy.cpu.registers.f.subtract);
    }

    #[test]
    fn test_sbc_half_carry_flag() {
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
        sbc_r_r_a(RegisterTarget::C)(&mut gameboy);
        assert!(gameboy.cpu.registers.f.half_carry);
    }

    #[test]
    fn test_sbc_n8() {
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
        sbc_n8(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.a, 3);
    }

    #[test]
    fn test_sbc_n8_overflow() {
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
        sbc_n8(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.a, 255);
    }

    #[test]
    fn test_sbc_n8_carry_flag() {
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
        sbc_n8(&mut gameboy);
        assert!(gameboy.cpu.registers.f.carry);
    }

    #[test]
    fn test_sbc_n8_zero_flag() {
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
        sbc_n8(&mut gameboy);
        assert!(gameboy.cpu.registers.f.zero);
    }

    #[test]
    fn test_sbc_n8_substract_flag() {
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
        sbc_n8(&mut gameboy);
        assert!(gameboy.cpu.registers.f.subtract);
    }

    #[test]
    fn test_sbc_n8_half_carry_flag() {
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
        sbc_n8(&mut gameboy);
        assert!(gameboy.cpu.registers.f.half_carry);
    }
}
