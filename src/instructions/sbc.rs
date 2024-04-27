use crate::cpu::RegisterTarget;
use crate::gameboy::Gameboy;

pub fn sbc_r_r_a(target: RegisterTarget) -> impl Fn(&mut Gameboy) -> u8 {
    move |gameboy: &mut Gameboy| {
        let current_value = gameboy.cpu.registers.get_u8(RegisterTarget::A);
        let operand = gameboy.cpu.registers.get_u8(target);

        let borrow_in = if gameboy.cpu.registers.f.carry { 1 } else { 0 };
        let (new_value, first_overflow) = current_value.overflowing_sub(operand);
        let (new_value, second_overflow) = new_value.overflowing_sub(borrow_in);
        let half_borrow = (current_value & 0xF) < (operand & 0xF) + borrow_in;

        gameboy.cpu.registers.a = new_value;

        gameboy.cpu.registers.f.carry = first_overflow || second_overflow;
        gameboy.cpu.registers.f.zero = new_value == 0;
        gameboy.cpu.registers.f.subtract = true;
        gameboy.cpu.registers.f.half_carry = half_borrow;
        const TICKS: u8 = 4;
        TICKS
    }
}

pub fn sbc_n8(gameboy: &mut Gameboy) -> u8 {
    let operand = gameboy.read_next_byte();
    let current_value = gameboy.cpu.registers.get_u8(RegisterTarget::A);

    let borrow_in = if gameboy.cpu.registers.f.carry { 1 } else { 0 };
    let (new_value, first_overflow) = current_value.overflowing_sub(operand);
    let (new_value, second_overflow) = new_value.overflowing_sub(borrow_in);
    let half_borrow = (current_value & 0xF) < (operand & 0xF) + borrow_in;

    gameboy.cpu.registers.a = new_value;

    gameboy.cpu.registers.f.carry = first_overflow || second_overflow;
    gameboy.cpu.registers.f.zero = new_value == 0;
    gameboy.cpu.registers.f.subtract = true;
    gameboy.cpu.registers.f.half_carry = half_borrow;
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
    fn test_sbc_half_carry_flag_2() {
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
        sbc_r_r_a(RegisterTarget::C)(&mut gameboy);
        assert!(gameboy.cpu.registers.f.half_carry);
    }

    #[test]
    fn test_sbc_n8() {
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
        sbc_n8(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.a, 3);
    }

    #[test]
    fn test_sbc_n8_overflow() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    pc: 0xC050,
                    a: 4,
                    f: FlagsRegister::from(0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        gameboy.bus.memory[0xC051] = 5;
        sbc_n8(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.a, 255);
    }

    #[test]
    fn test_sbc_n8_carry_flag() {
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
        gameboy.bus.memory[0xC051] = 5;
        sbc_n8(&mut gameboy);
        assert!(gameboy.cpu.registers.f.carry);
    }

    #[test]
    fn test_sbc_n8_carry_flag_2() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    a: 0,
                    pc: 0xC050,
                    f: FlagsRegister::from(0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        gameboy.bus.memory[0xC051] = 0xF0;
        sbc_n8(&mut gameboy);
        assert!(gameboy.cpu.registers.f.carry);
    }

    #[test]
    fn test_sbc_n8_carry_flag_3() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    a: 0,
                    pc: 0xC050,
                    f: FlagsRegister::from(0b0010000),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        gameboy.bus.memory[0xC051] = 0xFF;
        sbc_n8(&mut gameboy);
        assert!(gameboy.cpu.registers.f.carry);
    }

    #[test]
    fn test_sbc_n8_zero_flag() {
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
        sbc_n8(&mut gameboy);
        assert!(gameboy.cpu.registers.f.zero);
    }

    #[test]
    fn test_sbc_n8_substract_flag() {
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
        sbc_n8(&mut gameboy);
        assert!(gameboy.cpu.registers.f.subtract);
    }

    #[test]
    fn test_sbc_n8_half_carry_flag() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    pc: 0xC050,
                    a: 0b00010000,
                    f: FlagsRegister::from(0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        gameboy.bus.memory[0xC051] = 1;
        sbc_n8(&mut gameboy);
        assert!(gameboy.cpu.registers.f.half_carry);
    }
}
