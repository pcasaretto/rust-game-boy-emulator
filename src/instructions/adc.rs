use crate::cpu::{Register16bTarget, RegisterTarget};
use crate::gameboy::Gameboy;

pub fn adc(target: RegisterTarget) -> impl Fn(&mut Gameboy) -> u8 {
    move |gameboy: &mut Gameboy| {
        let addend = gameboy.cpu.registers.get_u8(target);
        let current_value = gameboy.cpu.registers.a;

        let carry_in = if gameboy.cpu.registers.f.carry { 1 } else { 0 };
        let new_value = current_value.wrapping_add(addend).wrapping_add(carry_in);
        let carry = (current_value as u16 + addend as u16 + carry_in as u16) > 0xFF;
        let half_carry = ((current_value & 0xF) + (addend & 0xF) + carry_in) > 0xF;

        gameboy.cpu.registers.a = new_value;

        gameboy.cpu.registers.f.carry = carry;
        gameboy.cpu.registers.f.subtract = false;
        gameboy.cpu.registers.f.zero = new_value == 0;
        gameboy.cpu.registers.f.half_carry = half_carry;
        const TICKS: u8 = 4;
        TICKS
    }
}

pub fn adc_n8(gameboy: &mut Gameboy) -> u8 {
    let addend = gameboy.read_next_byte();
    let current_value = gameboy.cpu.registers.a;

    let carry_in = if gameboy.cpu.registers.f.carry { 1 } else { 0 };
    let new_value = current_value.wrapping_add(addend).wrapping_add(carry_in);
    let carry = (current_value as u16 + addend as u16 + carry_in as u16) > 0xFF;
    let half_carry = ((current_value & 0xF) + (addend & 0xF) + carry_in) > 0xF;

    gameboy.cpu.registers.a = new_value;
    gameboy.cpu.registers.f.carry = carry;
    gameboy.cpu.registers.f.subtract = false;
    gameboy.cpu.registers.f.zero = new_value == 0;
    gameboy.cpu.registers.f.half_carry = half_carry;
    const TICKS: u8 = 8;
    TICKS
}

pub fn adc_mem_at_hl(gameboy: &mut Gameboy) -> u8 {
    let addr = gameboy.cpu.registers.get_u16(Register16bTarget::HL);
    let addend = gameboy.read_byte(addr);
    let current_value = gameboy.cpu.registers.get_u8(RegisterTarget::A);

    let carry_in = if gameboy.cpu.registers.f.carry { 1 } else { 0 };
    let new_value = current_value.wrapping_add(addend).wrapping_add(carry_in);
    let carry = (current_value as u16 + addend as u16 + carry_in as u16) > 0xFF;
    let half_carry = ((current_value & 0xF) + (addend & 0xF) + carry_in) > 0xF;

    gameboy.cpu.registers.set_u8(RegisterTarget::A, new_value);

    gameboy.cpu.registers.f.carry = carry;
    gameboy.cpu.registers.f.subtract = false;
    gameboy.cpu.registers.f.zero = new_value == 0;
    gameboy.cpu.registers.f.half_carry = half_carry;
    const TICKS: u8 = 8;
    TICKS
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
    fn test_adc_half_carry_flag_2() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    a: 0x00,
                    c: 0x0F,
                    f: FlagsRegister::from(0xF0),
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
        gameboy.write_byte(0xFFDA, 13);
        adc_mem_at_hl(&mut gameboy);
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
        gameboy.write_byte(0xFFDA, 13);
        adc_mem_at_hl(&mut gameboy);
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
        gameboy.write_byte(0xFFDA, 13);
        adc_mem_at_hl(&mut gameboy);
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
        gameboy.write_byte(0xFFDA, 0x10);
        adc_mem_at_hl(&mut gameboy);
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
        gameboy.write_byte(0xFFDA, 0x10);
        adc_mem_at_hl(&mut gameboy);
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
        gameboy.write_byte(0xFFDA, 1);
        adc_mem_at_hl(&mut gameboy);
        assert!(gameboy.cpu.registers.f.half_carry);
    }

    #[test]
    fn test_adc_n8_half_carry_flag() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.pc = 0xC050;
        gameboy.cpu.registers.a = 0x00;
        gameboy.write_byte(0xC051, 0x0F);
        gameboy.cpu.registers.f.carry = true;
        adc_n8(&mut gameboy);
        gameboy.cpu.registers.a = 0x10;
        assert!(gameboy.cpu.registers.f.half_carry);
    }

    #[test]
    fn test_adc_n8_half_carry_flag_2() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.pc = 0xC050;
        gameboy.cpu.registers.a = 0xFF;
        gameboy.write_byte(0xC051, 0x01);
        adc_n8(&mut gameboy);
        gameboy.cpu.registers.a = 0x00;
        assert!(gameboy.cpu.registers.f.half_carry);
    }
}
