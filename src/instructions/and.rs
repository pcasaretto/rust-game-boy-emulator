use crate::{Register16bTarget, RegisterTarget, CPU};

pub fn and(target: RegisterTarget) -> impl Fn(&mut CPU) {
    move |cpu: &mut CPU| {
        let r = cpu.registers.get_u8(target);
        let a = cpu.registers.get_u8(RegisterTarget::A);

        let value = a & r;

        cpu.registers.set_u8(RegisterTarget::A, value);

        cpu.registers.f.zero = value == 0;
        cpu.registers.f.subtract = false;
        cpu.registers.f.half_carry = true;
        cpu.registers.f.carry = false;
    }
}

pub fn and_mem_at_r16(reg: Register16bTarget) -> impl Fn(&mut CPU) {
    move |cpu: &mut CPU| {
        let addr = cpu.registers.get_u16(reg);
        let value = cpu.bus.read_byte(addr);
        let a = cpu.registers.get_u8(RegisterTarget::A);

        let result = a & value;

        cpu.registers.set_u8(RegisterTarget::A, result);

        cpu.registers.f.zero = result == 0;
        cpu.registers.f.subtract = false;
        cpu.registers.f.half_carry = true;
        cpu.registers.f.carry = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FlagsRegister, Registers};

    #[test]
    fn test_and() {
        let mut cpu = CPU {
            registers: Registers {
                a: 0b1100_1010,
                b: 0b1010_1010,
                ..Default::default()
            },
            ..Default::default()
        };
        and(RegisterTarget::B)(&mut cpu);
        assert_eq!(cpu.registers.a, 0b1000_1010);
    }

    #[test]
    fn test_and_zero_flag() {
        let mut cpu = CPU {
            registers: Registers {
                a: 31,
                b: 0b0000_0000,
                f: FlagsRegister::from(0),
                ..Default::default()
            },
            ..Default::default()
        };
        and(RegisterTarget::B)(&mut cpu);
        assert_eq!(cpu.registers.a, 0b0000_0000);
        assert!(cpu.registers.f.zero);
    }

    #[test]
    fn test_and_half_carry_flag() {
        let mut cpu = CPU::default();
        cpu.registers.f.half_carry = false;
        and(RegisterTarget::B)(&mut cpu);
        assert!(cpu.registers.f.half_carry);
    }

    #[test]
    fn test_and_carry_flag() {
        let mut cpu = CPU::default();
        cpu.registers.f.carry = true;
        and(RegisterTarget::B)(&mut cpu);
        assert!(!cpu.registers.f.carry);
    }

    #[test]
    fn test_and_subtract_flag() {
        let mut cpu = CPU::default();
        cpu.registers.f.subtract = true;
        and(RegisterTarget::B)(&mut cpu);
        assert!(!cpu.registers.f.subtract);
    }
}
