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

pub fn add_d8(gameboy: &mut Gameboy) -> u8 {
    let d8 = gameboy.read_next_byte();
    let current_value = gameboy.cpu.registers.get_u8(RegisterTarget::A);
    let (new_value, did_overflow) = current_value.overflowing_add(d8);
    gameboy.cpu.registers.a = new_value;

    gameboy.cpu.registers.f.carry = did_overflow;
    gameboy.cpu.registers.f.zero = new_value == 0;
    gameboy.cpu.registers.f.subtract = false;
    gameboy.cpu.registers.f.half_carry = (current_value & 0xF) + (d8 & 0xF) > 0xF;
    const TICKS: u8 = 8;
    TICKS
}

pub fn add_hl_r16(target: Register16bTarget) -> impl Fn(&mut Gameboy) -> u8 {
    move |gameboy: &mut Gameboy| {
        let hl = gameboy.cpu.registers.get_u16(Register16bTarget::HL);
        let target = gameboy.cpu.registers.get_u16(target);
        let (new_value, did_overflow) = hl.overflowing_add(target);
        gameboy
            .cpu
            .registers
            .set_u16(Register16bTarget::HL, new_value);

        gameboy.cpu.registers.f.carry = did_overflow;
        gameboy.cpu.registers.f.subtract = false;
        gameboy.cpu.registers.f.half_carry = (hl ^ target ^ new_value) & 0x1000 == 0x1000;
        const TICKS: u8 = 8;
        TICKS
    }
}

pub fn add_sp_n8(gameboy: &mut Gameboy) -> u8 {
    let n8 = gameboy.read_next_byte() as i8;
    let sp = gameboy.cpu.registers.sp;

    let (result, carry, half_carry) = if n8 < 0 {
        let t = n8.abs() as u16;
        let carry = false;
        let half_carry = false;
        (sp.wrapping_sub(t), carry, half_carry)
    } else {
        let carry = sp as u8 > 0xFF - n8 as u8;
        let half_carry = sp as u8 & 0xF + n8 as u8 & 0xF > 0xF;
        (sp.wrapping_add(n8 as u16), carry, half_carry)
    };

    gameboy.cpu.registers.sp = result;

    gameboy.cpu.registers.f.zero = false;
    gameboy.cpu.registers.f.subtract = false;
    gameboy.cpu.registers.f.half_carry = half_carry;
    gameboy.cpu.registers.f.carry = carry;
    const TICKS: u8 = 16;
    TICKS
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::{FlagsRegister, Register16bTarget, Registers, CPU};

    #[test]
    fn test_add_hl_r16() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.set_u16(Register16bTarget::HL, 0xC050);
        gameboy.cpu.registers.set_u16(Register16bTarget::BC, 0x001);
        add_hl_r16(Register16bTarget::BC)(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.get_u16(Register16bTarget::HL), 0xC051);
    }

    #[test]
    fn test_add_hl_r16_half_carry_flag() {
        // set if carry from bit 11
        let mut gameboy = Gameboy::default();
        gameboy
            .cpu
            .registers
            .set_u16(Register16bTarget::HL, 0b0000_1111_1111_1111);
        gameboy.cpu.registers.set_u16(Register16bTarget::BC, 0b1);
        add_hl_r16(Register16bTarget::BC)(&mut gameboy);
        assert!(gameboy.cpu.registers.f.half_carry);

        gameboy
            .cpu
            .registers
            .set_u16(Register16bTarget::HL, 0b0000_1111_1111_1110);
        gameboy.cpu.registers.set_u16(Register16bTarget::BC, 0b1);
        add_hl_r16(Register16bTarget::BC)(&mut gameboy);
        assert!(!gameboy.cpu.registers.f.half_carry);

        gameboy.cpu.registers.set_u16(Register16bTarget::HL, 0x4C00);
        add_hl_r16(Register16bTarget::HL)(&mut gameboy);
        assert!(gameboy.cpu.registers.f.half_carry);
    }

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

    #[test]
    fn test_add_d8() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    a: 4,
                    pc: 0xC050,
                    f: FlagsRegister::from(0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        gameboy.bus.memory[0xC051] = 1;
        add_d8(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.a, 5);
    }

    #[test]
    fn test_add_d8_overflow() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    pc: 0xC050,
                    a: 0xFF,
                    f: FlagsRegister::from(0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        gameboy.bus.memory[0xC051] = 2;
        add_d8(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.a, 1);
    }

    #[test]
    fn test_add_d8_carry_flag() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    a: 0xFF,
                    pc: 0xC050,
                    f: FlagsRegister::from(0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        gameboy.bus.memory[0xC051] = 5;
        add_d8(&mut gameboy);
        assert!(gameboy.cpu.registers.f.carry);
    }

    #[test]
    fn test_add_d8_zero_flag() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    pc: 0xC050,
                    a: 0xFF,
                    f: FlagsRegister::from(0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        gameboy.bus.memory[0xC051] = 1;
        add_d8(&mut gameboy);
        assert!(gameboy.cpu.registers.f.zero);
    }

    #[test]
    fn test_add_d8_substract_flag() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    pc: 0xC050,
                    a: 5,
                    f: FlagsRegister::from(0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        gameboy.bus.memory[0xC051] = 5;
        add_d8(&mut gameboy);
        assert!(!gameboy.cpu.registers.f.subtract);
    }

    #[test]
    fn test_add_d8_half_carry_flag() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    pc: 0xC050,
                    a: 0b00001111,
                    f: FlagsRegister::from(0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        gameboy.bus.memory[0xC051] = 1;
        add_d8(&mut gameboy);
        assert!(gameboy.cpu.registers.f.half_carry);
    }
}
