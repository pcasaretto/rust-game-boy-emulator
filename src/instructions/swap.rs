use crate::cpu::{RegisterTarget};
use crate::gameboy::Gameboy;

pub fn swap(target: RegisterTarget) -> impl Fn(&mut Gameboy) -> u8 {
    move |gameboy: &mut Gameboy| {
        let value = gameboy.cpu.registers.get_u8(target);
        let result = (value << 4) | (value >> 4);

        gameboy.cpu.registers.set_u8(target, result);

        gameboy.cpu.registers.f.zero = result == 0;
        gameboy.cpu.registers.f.subtract = false;
        gameboy.cpu.registers.f.half_carry = false;
        gameboy.cpu.registers.f.carry = false;
        const CYCLES: u8 = 2;
        CYCLES
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::{Registers, CPU};

    #[test]
    fn test_swap() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    a: 0b1100_1010,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        swap(RegisterTarget::A)(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.a, 0b1010_1100);
    }

    #[test]
    fn test_swap_zero_flag() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    a: 0,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        gameboy.cpu.registers.f.zero = false;
        swap(RegisterTarget::A)(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.f.zero, true);
    }

    #[test]
    fn test_swap_half_carry_flag() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.f.half_carry = true;
        swap(RegisterTarget::A)(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.f.half_carry, false);
    }

    #[test]
    fn test_swap_carry_flag() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.f.carry = true;
        swap(RegisterTarget::A)(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.f.carry, false);
    }

    #[test]
    fn test_swap_subtract_flag() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.f.subtract = true;
        swap(RegisterTarget::A)(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.f.subtract, false);
    }
}
