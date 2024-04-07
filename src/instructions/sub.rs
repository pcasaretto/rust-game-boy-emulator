use super::super::*;

pub fn sub(target: RegisterTarget) -> impl Fn(&mut CPU) {
    move |cpu: &mut CPU| {
        let target_value = cpu.read_single_register(target);
        let current_value = cpu.read_single_register(RegisterTarget::A);
        let (new_value, did_overflow) = current_value.overflowing_sub(target_value);
        cpu.registers.a = new_value;

        cpu.registers.f.carry = did_overflow;
        cpu.registers.f.zero = new_value == 0;
        cpu.registers.f.subtract = true;
        cpu.registers.f.half_carry = (current_value & 0xF) < (target_value & 0xF);

        cpu.pc = cpu.pc.wrapping_add(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        sub(RegisterTarget::C)(&mut cpu);
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
        sub(RegisterTarget::C)(&mut cpu);
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
        sub(RegisterTarget::C)(&mut cpu);
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
        sub(RegisterTarget::C)(&mut cpu);
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
        sub(RegisterTarget::C)(&mut cpu);
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
        sub(RegisterTarget::C)(&mut cpu);
        assert!(cpu.registers.f.half_carry);
    }

    #[test]
    fn test_sub_advance_pc() {
        let mut cpu = CPU {
            pc: 123,
            ..Default::default()
        };
        sub(RegisterTarget::C)(&mut cpu);
        assert_eq!(cpu.pc, 124);
    }

    #[test]
    fn test_sub_advance_pc_wrap() {
        let mut cpu = CPU {
            pc: 0xFFFF,
            ..Default::default()
        };
        sub(RegisterTarget::C)(&mut cpu);
        assert_eq!(cpu.pc, 0);
    }
}
