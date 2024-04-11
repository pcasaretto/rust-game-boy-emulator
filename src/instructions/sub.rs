use crate::cpu::{RegisterTarget, CPU};

pub fn sub_r_r_a(target: RegisterTarget) -> impl Fn(&mut CPU) {
    move |cpu: &mut CPU| {
        let target_value = cpu.registers.get_u8(target);
        let current_value = cpu.registers.get_u8(RegisterTarget::A);
        let (new_value, did_overflow) = current_value.overflowing_sub(target_value);
        cpu.registers.a = new_value;

        cpu.registers.f.carry = did_overflow;
        cpu.registers.f.zero = new_value == 0;
        cpu.registers.f.subtract = true;
        cpu.registers.f.half_carry = (current_value & 0xF) < (target_value & 0xF);
    }
}

pub fn sub_d8() -> impl Fn(&mut CPU) {
    move |cpu: &mut CPU| {
        let d8 = cpu.bus.memory[cpu.pc as usize + 1];
        let current_value = cpu.registers.get_u8(RegisterTarget::A);
        let (new_value, did_overflow) = current_value.overflowing_sub(d8);
        cpu.registers.a = new_value;

        cpu.registers.f.carry = did_overflow;
        cpu.registers.f.zero = new_value == 0;
        cpu.registers.f.subtract = true;
        cpu.registers.f.half_carry = (current_value & 0xF) < (d8 & 0xF);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::FlagsRegister;
    use crate::cpu::Registers;

    #[test]
    fn test_sub() {
        let mut cpu = CPU {
            registers: Registers {
                a: 4,
                c: 1,
                f: FlagsRegister::from(0),
                ..Default::default()
            },
            ..Default::default()
        };
        sub_r_r_a(RegisterTarget::C)(&mut cpu);
        assert_eq!(cpu.registers.a, 3);
    }

    #[test]
    fn test_sub_overflow() {
        let mut cpu = CPU {
            registers: Registers {
                a: 4,
                c: 5,
                f: FlagsRegister::from(0),
                ..Default::default()
            },
            ..Default::default()
        };
        sub_r_r_a(RegisterTarget::C)(&mut cpu);
        assert_eq!(cpu.registers.a, 255);
    }

    #[test]
    fn test_sub_carry_flag() {
        let mut cpu = CPU {
            registers: Registers {
                a: 4,
                c: 5,
                f: FlagsRegister::from(0),
                ..Default::default()
            },
            ..Default::default()
        };
        sub_r_r_a(RegisterTarget::C)(&mut cpu);
        assert!(cpu.registers.f.carry);
    }

    #[test]
    fn test_sub_zero_flag() {
        let mut cpu = CPU {
            registers: Registers {
                a: 5,
                c: 5,
                f: FlagsRegister::from(0),
                ..Default::default()
            },
            ..Default::default()
        };
        sub_r_r_a(RegisterTarget::C)(&mut cpu);
        assert!(cpu.registers.f.zero);
    }

    #[test]
    fn test_sub_substract_flag() {
        let mut cpu = CPU {
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
        };
        sub_r_r_a(RegisterTarget::C)(&mut cpu);
        assert!(cpu.registers.f.subtract);
    }

    #[test]
    fn test_sub_half_carry_flag() {
        let mut cpu = CPU {
            registers: Registers {
                a: 0b00010000,
                c: 0b00000001,
                f: FlagsRegister::from(0),
                ..Default::default()
            },
            ..Default::default()
        };
        sub_r_r_a(RegisterTarget::C)(&mut cpu);
        assert!(cpu.registers.f.half_carry);
    }

    #[test]
    fn test_sub_d8() {
        let mut cpu = CPU {
            pc: 0x0012,
            registers: Registers {
                a: 4,
                f: FlagsRegister::from(0),
                ..Default::default()
            },
            ..Default::default()
        };
        cpu.bus.memory[0x0013] = 1;
        sub_d8()(&mut cpu);
        assert_eq!(cpu.registers.a, 3);
    }

    #[test]
    fn test_sub_d8_overflow() {
        let mut cpu = CPU {
            pc: 0x0012,
            registers: Registers {
                a: 4,
                f: FlagsRegister::from(0),
                ..Default::default()
            },
            ..Default::default()
        };
        cpu.bus.memory[0x0013] = 5;
        sub_d8()(&mut cpu);
        assert_eq!(cpu.registers.a, 255);
    }

    #[test]
    fn test_sub_d8_carry_flag() {
        let mut cpu = CPU {
            pc: 0x0012,
            registers: Registers {
                a: 4,
                f: FlagsRegister::from(0),
                ..Default::default()
            },
            ..Default::default()
        };
        cpu.bus.memory[0x0013] = 5;
        sub_d8()(&mut cpu);
        assert!(cpu.registers.f.carry);
    }

    #[test]
    fn test_sub_d8_zero_flag() {
        let mut cpu = CPU {
            pc: 0x0012,
            registers: Registers {
                a: 5,
                f: FlagsRegister::from(0),
                ..Default::default()
            },
            ..Default::default()
        };
        cpu.bus.memory[0x0013] = 5;
        sub_d8()(&mut cpu);
        assert!(cpu.registers.f.zero);
    }

    #[test]
    fn test_sub_d8_substract_flag() {
        let mut cpu = CPU {
            pc: 0x0012,
            registers: Registers {
                a: 5,
                f: FlagsRegister::from(0),
                ..Default::default()
            },
            ..Default::default()
        };
        cpu.bus.memory[0x0013] = 5;
        sub_d8()(&mut cpu);
        assert!(cpu.registers.f.subtract);
    }

    #[test]
    fn test_sub_d8_half_carry_flag() {
        let mut cpu = CPU {
            pc: 0x0012,
            registers: Registers {
                a: 0b00010000,
                f: FlagsRegister::from(0),
                ..Default::default()
            },
            ..Default::default()
        };
        cpu.bus.memory[0x0013] = 1;
        sub_d8()(&mut cpu);
        assert!(cpu.registers.f.half_carry);
    }
}
