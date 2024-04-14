use crate::cpu::{Register16bTarget, RegisterTarget};
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

pub fn add_mem_at_hl(gameboy: &mut Gameboy) -> u8 {
    let hl = gameboy.cpu.registers.get_u16(Register16bTarget::HL);
    let target_value = gameboy.bus.read_byte(hl);
    let current_value = gameboy.cpu.registers.a;
    let (new_value, did_overflow) = current_value.overflowing_add(target_value);
    gameboy.cpu.registers.a = new_value;

    gameboy.cpu.registers.f.carry = did_overflow;
    gameboy.cpu.registers.f.zero = new_value == 0;
    gameboy.cpu.registers.f.subtract = false;
    gameboy.cpu.registers.f.half_carry = (current_value & 0xF) + (target_value & 0xF) > 0xF;
    const TICKS: u8 = 8;
    TICKS
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::{FlagsRegister, Register16bTarget, Registers, CPU};

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

    #[test]
    fn test_add_mem_at_hl() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    a: 0,
                    f: FlagsRegister::from(0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        gameboy.cpu.registers.set_u16(Register16bTarget::HL, 0xC050);
        gameboy.bus.write_byte(0xC050, 1);
        add_mem_at_hl(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.a, 1);
    }

    #[test]
    fn test_add_mem_at_hl_overflow() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    a: 255,
                    f: FlagsRegister::from(0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        gameboy.cpu.registers.set_u16(Register16bTarget::HL, 0xC050);
        gameboy.bus.write_byte(0xC050, 1);
        add_mem_at_hl(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.a, 0);
    }

    #[test]
    fn test_add_mem_at_hl_carry_flag() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    a: 255,
                    f: FlagsRegister::from(0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        gameboy.cpu.registers.set_u16(Register16bTarget::HL, 0xC050);
        gameboy.bus.write_byte(0xC050, 1);
        add_mem_at_hl(&mut gameboy);
        assert!(gameboy.cpu.registers.f.carry);
    }

    #[test]
    fn test_add_mem_at_hl_zero_flag() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    a: 0,
                    f: FlagsRegister::from(0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        gameboy.cpu.registers.set_u16(Register16bTarget::HL, 0xC050);
        gameboy.bus.write_byte(0xC050, 0);
        add_mem_at_hl(&mut gameboy);
        assert!(gameboy.cpu.registers.f.zero);
    }

    #[test]
    fn test_add_mem_at_hl_substract_flag() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    a: 0,
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
        gameboy.cpu.registers.set_u16(Register16bTarget::HL, 0xC050);
        gameboy.bus.write_byte(0xC050, 1);
        add_mem_at_hl(&mut gameboy);
        assert!(!gameboy.cpu.registers.f.subtract);
    }

    #[test]
    fn test_add_mem_at_hl_half_carry_flag() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    a: 0b00001111,
                    f: FlagsRegister::from(0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        gameboy.cpu.registers.set_u16(Register16bTarget::HL, 0xC050);
        gameboy.bus.write_byte(0xC050, 0b0000_0001);
        add_mem_at_hl(&mut gameboy);
        assert!(gameboy.cpu.registers.f.half_carry);
    }
}
