use crate::cpu::{RegisterTarget, CPU};

pub fn dec_r(target: RegisterTarget) -> impl Fn(&mut CPU) {
    move |cpu: &mut CPU| {
        let current_value = cpu.registers.get_u8(target);
        let (new_value, did_overflow) = current_value.overflowing_sub(1);
        cpu.registers.set_u8(target, new_value);

        cpu.registers.f.zero = new_value == 0;
        cpu.registers.f.subtract = true;
        cpu.registers.f.half_carry = current_value & 0x10 == 0x10;
        cpu.registers.f.carry = did_overflow;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::{FlagsRegister, Registers};

    #[test]
    fn test_dec_r() {
        let mut cpu = CPU {
            registers: Registers {
                b: 2,
                ..Default::default()
            },
            ..Default::default()
        };
        dec_r(RegisterTarget::B)(&mut cpu);
        assert_eq!(cpu.registers.b, 1);
    }

    #[test]
    fn test_dec_r_overflow() {
        let mut cpu = CPU {
            registers: Registers {
                b: 0,
                ..Default::default()
            },
            ..Default::default()
        };
        dec_r(RegisterTarget::B)(&mut cpu);
        assert_eq!(cpu.registers.b, 0xFF);
    }

    #[test]
    fn test_dec_r_subtract_flag() {
        let mut cpu = CPU {
            registers: Registers {
                f: FlagsRegister {
                    subtract: false,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        dec_r(RegisterTarget::B)(&mut cpu);
        assert!(cpu.registers.f.subtract);
    }

    #[test]
    fn test_dec_r_half_carry_flag() {
        let mut cpu = CPU {
            registers: Registers {
                b: 0x10,
                f: FlagsRegister::from(0),
                ..Default::default()
            },
            ..Default::default()
        };
        dec_r(RegisterTarget::B)(&mut cpu);
        assert!(cpu.registers.f.half_carry);
    }

    #[test]
    fn test_dec_r_carry_flag() {
        let mut cpu = CPU {
            registers: Registers {
                b: 0,
                f: FlagsRegister::from(0),
                ..Default::default()
            },
            ..Default::default()
        };
        dec_r(RegisterTarget::B)(&mut cpu);
        assert!(cpu.registers.f.carry);
    }
}
