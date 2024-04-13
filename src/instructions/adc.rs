use crate::cpu::{Register16bTarget, RegisterTarget};
use crate::gameboy::Gameboy;

pub fn adc(target: RegisterTarget) -> impl Fn(&mut Gameboy) -> u8 {
    move |gameboy: &mut Gameboy| {
        let mut addend = gameboy.cpu.registers.get_u8(target);
        let current_value = gameboy.cpu.registers.a;
        if gameboy.cpu.registers.f.carry {
            addend += 1;
        }
        let (new_value, did_overflow) = current_value.overflowing_add(addend);
        gameboy.cpu.registers.a = new_value;

        gameboy.cpu.registers.f.carry = did_overflow;
        gameboy.cpu.registers.f.subtract = false;
        gameboy.cpu.registers.f.zero = new_value == 0;
        gameboy.cpu.registers.f.half_carry = (current_value & 0xF) + (addend & 0xF) > 0xF;
        const TICKS: u8 = 4;
        TICKS
    }
}

pub fn adc_mem_at_hl() -> impl Fn(&mut Gameboy) -> u8 {
    move |gameboy: &mut Gameboy| {
        let addr = gameboy.cpu.registers.get_u16(Register16bTarget::HL);
        let mut addend = gameboy.bus.read_byte(addr);
        let current_value = gameboy.cpu.registers.get_u8(RegisterTarget::A);
        if gameboy.cpu.registers.f.carry {
            addend += 1;
        }
        let (new_value, did_overflow) = current_value.overflowing_add(addend);
        gameboy.cpu.registers.set_u8(RegisterTarget::A, new_value);

        gameboy.cpu.registers.f.carry = did_overflow;
        gameboy.cpu.registers.f.subtract = false;
        gameboy.cpu.registers.f.zero = new_value == 0;
        gameboy.cpu.registers.f.half_carry = (current_value & 0xF) + (addend & 0xF) > 0xF;
        const TICKS: u8 = 8;
        TICKS
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::{FlagsRegister, Registers, CPU};

    #[test]
    fn test_adc() {
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
        adc(RegisterTarget::C)(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.a, 1);
    }

    #[test]
    fn test_adc_with_carry() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    a: 0,
                    c: 1,
                    f: FlagsRegister {
                        carry: true,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        adc(RegisterTarget::C)(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.a, 2);
    }

    #[test]
    fn test_adc_overflow() {
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
        adc(RegisterTarget::C)(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.a, 0);
    }

    #[test]
    fn test_adc_carry_flag() {
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
        adc(RegisterTarget::C)(&mut gameboy);
        assert!(gameboy.cpu.registers.f.carry);
    }

    #[test]
    fn test_adc_zero_flag() {
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
        adc(RegisterTarget::C)(&mut gameboy);
        assert!(gameboy.cpu.registers.f.zero);
    }

    #[test]
    fn test_adc_substract_flag() {
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
        adc(RegisterTarget::C)(&mut gameboy);
        assert!(!gameboy.cpu.registers.f.subtract);
    }

    #[test]
    fn test_adc_half_carry_flag() {
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
        adc(RegisterTarget::C)(&mut gameboy);
        assert!(gameboy.cpu.registers.f.half_carry);
    }

    #[test]
    fn test_adc_mem_at_r16() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    a: 34,
                    h: 0xFF,
                    l: 0xDA,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        gameboy.bus.write_byte(0xFFDA, 13);
        adc_mem_at_hl()(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.a, 47);
    }

    #[test]
    fn test_adc_mem_at_r16_carry() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    a: 34,
                    h: 0xFF,
                    l: 0xDA,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        gameboy.cpu.registers.f.carry = true;
        gameboy.bus.write_byte(0xFFDA, 13);
        adc_mem_at_hl()(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.a, 48);
    }

    #[test]
    fn test_adc_mem_at_hl_carry_flag() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    a: 0xFF,
                    h: 0xFF,
                    l: 0xDA,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        gameboy.bus.write_byte(0xFFDA, 13);
        adc_mem_at_hl()(&mut gameboy);
        assert!(gameboy.cpu.registers.f.carry);
    }

    #[test]
    fn test_adc_mem_at_hl_zero_flag() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    a: 0xF0,
                    h: 0xFF,
                    l: 0xDA,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        gameboy.bus.write_byte(0xFFDA, 0x10);
        adc_mem_at_hl()(&mut gameboy);
        assert!(gameboy.cpu.registers.f.zero);
    }

    #[test]
    fn test_adc_mem_at_hl_substract_flag() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    a: 0xF0,
                    h: 0xFF,
                    l: 0xDA,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        gameboy.cpu.registers.f.subtract = true;
        gameboy.bus.write_byte(0xFFDA, 0x10);
        adc_mem_at_hl()(&mut gameboy);
        assert!(!gameboy.cpu.registers.f.subtract);
    }

    #[test]
    fn test_adc_mem_at_hl_half_carry_flag() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    a: 0x0F,
                    h: 0xFF,
                    l: 0xDA,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        gameboy.bus.write_byte(0xFFDA, 1);
        adc_mem_at_hl()(&mut gameboy);
        assert!(gameboy.cpu.registers.f.half_carry);
    }
}
