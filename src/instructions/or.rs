use crate::cpu::{Register16bTarget, RegisterTarget};
use crate::gameboy::Gameboy;
use crate::instructions::binary;

pub fn or(target: RegisterTarget) -> impl Fn(&mut Gameboy) {
    binary::operation_on_r_a(target, |left, right| left | right)
}

pub fn or_mem_at_r16(reg: Register16bTarget) -> impl Fn(&mut Gameboy) {
    move |gameboy: &mut Gameboy| {
        let addr = gameboy.cpu.registers.get_u16(reg);
        let value = gameboy.bus.read_byte(addr);
        let a = gameboy.cpu.registers.get_u8(RegisterTarget::A);

        let result = a & value;

        gameboy.cpu.registers.set_u8(RegisterTarget::A, result);

        gameboy.cpu.registers.f.zero = result == 0;
        gameboy.cpu.registers.f.subtract = false;
        gameboy.cpu.registers.f.half_carry = true;
        gameboy.cpu.registers.f.carry = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::{FlagsRegister, Registers, CPU};

    #[test]
    fn test_or() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    a: 0b1100_1010,
                    b: 0b1010_1010,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        or(RegisterTarget::B)(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.a, 0b1110_1010);
    }

    #[test]
    fn test_or_zero_flag() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    a: 0,
                    b: 0,
                    f: FlagsRegister::from(0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        or(RegisterTarget::B)(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.a, 0);
        assert!(gameboy.cpu.registers.f.zero);
    }

    #[test]
    fn test_or_half_carry_flag() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.f.half_carry = false;
        or(RegisterTarget::B)(&mut gameboy);
        assert!(gameboy.cpu.registers.f.half_carry);
    }

    #[test]
    fn test_or_carry_flag() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.f.carry = true;
        or(RegisterTarget::B)(&mut gameboy);
        assert!(!gameboy.cpu.registers.f.carry);
    }

    #[test]
    fn test_or_subtract_flag() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.f.subtract = true;
        or(RegisterTarget::B)(&mut gameboy);
        assert!(!gameboy.cpu.registers.f.subtract);
    }
}
