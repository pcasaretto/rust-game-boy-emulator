use crate::cpu::{Register16bTarget, RegisterTarget, CPU};

pub fn adc(target: RegisterTarget) -> impl Fn(&mut CPU) {
    move |cpu: &mut CPU| {
        let mut addend = cpu.registers.get_u8(target);
        let current_value = cpu.registers.a;
        if cpu.registers.f.carry {
            addend += 1;
        }
        let (new_value, did_overflow) = current_value.overflowing_add(addend);
        cpu.registers.a = new_value;

        cpu.registers.f.carry = did_overflow;
        cpu.registers.f.subtract = false;
        cpu.registers.f.zero = new_value == 0;
        cpu.registers.f.half_carry = (current_value & 0xF) + (addend & 0xF) > 0xF;
    }
}

pub fn adc_mem_at_hl() -> impl Fn(&mut CPU) {
    move |cpu: &mut CPU| {
        let addr = cpu.registers.get_u16(Register16bTarget::HL);
        let mut addend = cpu.bus.read_byte(addr);
        let current_value = cpu.registers.get_u8(RegisterTarget::A);
        if cpu.registers.f.carry {
            addend += 1;
        }
        let (new_value, did_overflow) = current_value.overflowing_add(addend);
        cpu.registers.set_u8(RegisterTarget::A, new_value);

        cpu.registers.f.carry = did_overflow;
        cpu.registers.f.subtract = false;
        cpu.registers.f.zero = new_value == 0;
        cpu.registers.f.half_carry = (current_value & 0xF) + (addend & 0xF) > 0xF;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::{FlagsRegister, Registers};

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

    #[test]
    fn test_adc_mem_at_r16() {
        let mut cpu = CPU {
            registers: Registers {
                a: 34,
                h: 0xFF,
                l: 0xDA,
                ..Default::default()
            },
            ..Default::default()
        };
        cpu.bus.write_byte(0xFFDA, 13);
        adc_mem_at_hl()(&mut cpu);
        assert_eq!(cpu.registers.a, 47);
    }

    #[test]
    fn test_adc_mem_at_r16_carry() {
        let mut cpu = CPU {
            registers: Registers {
                a: 34,
                h: 0xFF,
                l: 0xDA,
                ..Default::default()
            },
            ..Default::default()
        };
        cpu.registers.f.carry = true;
        cpu.bus.write_byte(0xFFDA, 13);
        adc_mem_at_hl()(&mut cpu);
        assert_eq!(cpu.registers.a, 48);
    }

    #[test]
    fn test_adc_mem_at_hl_carry_flag() {
        let mut cpu = CPU {
            registers: Registers {
                a: 0xFF,
                h: 0xFF,
                l: 0xDA,
                ..Default::default()
            },
            ..Default::default()
        };
        cpu.bus.write_byte(0xFFDA, 13);
        adc_mem_at_hl()(&mut cpu);
        assert!(cpu.registers.f.carry);
    }

    #[test]
    fn test_adc_mem_at_hl_zero_flag() {
        let mut cpu = CPU {
            registers: Registers {
                a: 0xF0,
                h: 0xFF,
                l: 0xDA,
                ..Default::default()
            },
            ..Default::default()
        };
        cpu.bus.write_byte(0xFFDA, 0x10);
        adc_mem_at_hl()(&mut cpu);
        assert!(cpu.registers.f.zero);
    }

    #[test]
    fn test_adc_mem_at_hl_substract_flag() {
        let mut cpu = CPU {
            registers: Registers {
                a: 0xF0,
                h: 0xFF,
                l: 0xDA,
                ..Default::default()
            },
            ..Default::default()
        };
        cpu.registers.f.subtract = true;
        cpu.bus.write_byte(0xFFDA, 0x10);
        adc_mem_at_hl()(&mut cpu);
        assert!(!cpu.registers.f.subtract);
    }

    #[test]
    fn test_adc_mem_at_hl_half_carry_flag() {
        let mut cpu = CPU {
            registers: Registers {
                a: 0x0F,
                h: 0xFF,
                l: 0xDA,
                ..Default::default()
            },
            ..Default::default()
        };
        cpu.bus.write_byte(0xFFDA, 1);
        adc_mem_at_hl()(&mut cpu);
        assert!(cpu.registers.f.half_carry);
    }
}
