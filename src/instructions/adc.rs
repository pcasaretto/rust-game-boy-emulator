use super::super::*;

pub fn adc(target: RegisterTarget) -> impl Fn(&mut CPU) {
    move |cpu: &mut CPU| {
        let mut target_value = cpu.read_single_register(target);
        let current_value = cpu.registers.a;
        if cpu.registers.f.carry {
            target_value += 1;
        }
        let (new_value, did_overflow) = current_value.overflowing_add(target_value);
        cpu.registers.a = new_value;

        cpu.registers.f.carry = did_overflow;
        cpu.registers.f.subtract = false;
        cpu.registers.f.zero = new_value == 0;
        cpu.registers.f.half_carry = (current_value & 0xF) + (target_value & 0xF) > 0xF;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adc() {
        let mut cpu = CPU {
            registers: Registers {
                a: 0,
                c: 1,
                f: FlagsRegister::from(0),
                ..Default::default()
            },
            ..Default::default()
        };
        adc(RegisterTarget::C)(&mut cpu);
        assert_eq!(cpu.registers.a, 1);
    }

    #[test]
    fn test_adc_with_carry() {
        let mut cpu = CPU {
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
        };
        adc(RegisterTarget::C)(&mut cpu);
        assert_eq!(cpu.registers.a, 2);
    }

    #[test]
    fn test_adc_overflow() {
        let mut cpu = CPU {
            registers: Registers {
                a: 255,
                c: 1,
                f: FlagsRegister::from(0),
                ..Default::default()
            },
            ..Default::default()
        };
        adc(RegisterTarget::C)(&mut cpu);
        assert_eq!(cpu.registers.a, 0);
    }

    #[test]
    fn test_adc_carry_flag() {
        let mut cpu = CPU {
            registers: Registers {
                a: 255,
                c: 1,
                f: FlagsRegister::from(0),
                ..Default::default()
            },
            ..Default::default()
        };
        adc(RegisterTarget::C)(&mut cpu);
        assert!(cpu.registers.f.carry);
    }

    #[test]
    fn test_adc_zero_flag() {
        let mut cpu = CPU {
            registers: Registers {
                a: 0,
                c: 0,
                f: FlagsRegister::from(0),
                ..Default::default()
            },
            ..Default::default()
        };
        adc(RegisterTarget::C)(&mut cpu);
        assert!(cpu.registers.f.zero);
    }

    #[test]
    fn test_adc_substract_flag() {
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
        adc(RegisterTarget::C)(&mut cpu);
        assert!(!cpu.registers.f.subtract);
    }

    #[test]
    fn test_adc_half_carry_flag() {
        let mut cpu = CPU {
            registers: Registers {
                a: 0b00001111,
                c: 0b00000001,
                f: FlagsRegister::from(0),
                ..Default::default()
            },
            ..Default::default()
        };
        adc(RegisterTarget::C)(&mut cpu);
        assert!(cpu.registers.f.half_carry);
    }
}
