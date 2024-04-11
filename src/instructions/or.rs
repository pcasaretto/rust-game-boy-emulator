use crate::cpu::{Register16bTarget, RegisterTarget, CPU};
use crate::instructions::binary;

pub fn or(target: RegisterTarget) -> impl Fn(&mut CPU) {
    binary::operation_on_r_a(target, |left, right| left | right)
}

pub fn or_mem_at_r16(reg: Register16bTarget) -> impl Fn(&mut CPU) {
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
    use crate::cpu::{FlagsRegister, Registers};

    #[test]
    fn test_or() {
        let mut cpu = CPU {
            registers: Registers {
                a: 0b1100_1010,
                b: 0b1010_1010,
                ..Default::default()
            },
            ..Default::default()
        };
        or(RegisterTarget::B)(&mut cpu);
        assert_eq!(cpu.registers.a, 0b1110_1010);
    }

    #[test]
    fn test_or_zero_flag() {
        let mut cpu = CPU {
            registers: Registers {
                a: 0,
                b: 0,
                f: FlagsRegister::from(0),
                ..Default::default()
            },
            ..Default::default()
        };
        or(RegisterTarget::B)(&mut cpu);
        assert_eq!(cpu.registers.a, 0);
        assert!(cpu.registers.f.zero);
    }

    #[test]
    fn test_or_half_carry_flag() {
        let mut cpu = CPU::default();
        cpu.registers.f.half_carry = false;
        or(RegisterTarget::B)(&mut cpu);
        assert!(cpu.registers.f.half_carry);
    }

    #[test]
    fn test_or_carry_flag() {
        let mut cpu = CPU::default();
        cpu.registers.f.carry = true;
        or(RegisterTarget::B)(&mut cpu);
        assert!(!cpu.registers.f.carry);
    }

    #[test]
    fn test_or_subtract_flag() {
        let mut cpu = CPU::default();
        cpu.registers.f.subtract = true;
        or(RegisterTarget::B)(&mut cpu);
        assert!(!cpu.registers.f.subtract);
    }
}
