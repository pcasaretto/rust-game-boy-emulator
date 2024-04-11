use crate::cpu::{Register16bTarget, RegisterTarget, CPU};

pub fn inc_r(target: RegisterTarget) -> impl Fn(&mut CPU) {
    move |cpu: &mut CPU| {
        let current_value = cpu.registers.get_u8(target);
        let (new_value, did_overflow) = current_value.overflowing_add(1);
        cpu.registers.set_u8(target, new_value);

        cpu.registers.f.zero = new_value == 0;
        cpu.registers.f.subtract = false;
        cpu.registers.f.half_carry = current_value & 0xF == 0xF;
        cpu.registers.f.carry = did_overflow;
    }
}

pub fn inc_r16(target: Register16bTarget) -> impl Fn(&mut CPU) {
    move |cpu: &mut CPU| {
        let current_value = cpu.registers.get_u16(target);
        let new_value = current_value.wrapping_add(1);
        cpu.registers.set_u16(target, new_value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::{FlagsRegister, Registers};

    #[test]
    fn test_inc_r() {
        let mut cpu = CPU::default();
        inc_r(RegisterTarget::B)(&mut cpu);
        assert_eq!(cpu.registers.b, 1);
    }

    #[test]
    fn test_inc_r_overflow() {
        let mut cpu = CPU {
            registers: Registers {
                b: 0xFF,
                ..Default::default()
            },
            ..Default::default()
        };
        inc_r(RegisterTarget::B)(&mut cpu);
        assert_eq!(cpu.registers.b, 0);
    }

    #[test]
    fn test_inc_r_subtract_flag() {
        let mut cpu = CPU {
            registers: Registers {
                f: FlagsRegister {
                    subtract: true,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        inc_r(RegisterTarget::B)(&mut cpu);
        assert_eq!(cpu.registers.f.subtract, false);
    }

    #[test]
    fn test_inc_r_half_carry_flag() {
        let mut cpu = CPU {
            registers: Registers {
                b: 0x0F,
                f: FlagsRegister::from(0),
                ..Default::default()
            },
            ..Default::default()
        };
        inc_r(RegisterTarget::B)(&mut cpu);
        assert_eq!(cpu.registers.f.half_carry, true);
    }

    #[test]
    fn test_inc_r_carry_flag() {
        let mut cpu = CPU {
            registers: Registers {
                b: 0xFF,
                f: FlagsRegister::from(0),
                ..Default::default()
            },
            ..Default::default()
        };
        inc_r(RegisterTarget::B)(&mut cpu);
        assert_eq!(cpu.registers.f.carry, true);
    }
}
