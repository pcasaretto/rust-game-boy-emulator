use crate::cpu::{Register16bTarget, RegisterTarget};
use crate::gameboy::Gameboy;

pub fn and(target: RegisterTarget) -> impl Fn(&mut Gameboy) -> u8 {
    move |gameboy: &mut Gameboy| {
        let r = gameboy.cpu.registers.get_u8(target);
        let a = gameboy.cpu.registers.get_u8(RegisterTarget::A);

        let value = a & r;

        gameboy.cpu.registers.set_u8(RegisterTarget::A, value);

        gameboy.cpu.registers.f.zero = value == 0;
        gameboy.cpu.registers.f.subtract = false;
        gameboy.cpu.registers.f.half_carry = true;
        gameboy.cpu.registers.f.carry = false;
        const TICKS: u8 = 4;
        TICKS
    }
}

pub fn and_mem_at_r16(reg: Register16bTarget) -> impl Fn(&mut Gameboy) -> u8 {
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
        const TICKS: u8 = 8;
        TICKS
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::{FlagsRegister, Registers, CPU};

    #[test]
    fn test_and() {
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
        and(RegisterTarget::B)(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.a, 0b1000_1010);
    }

    #[test]
    fn test_and_zero_flag() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    a: 31,
                    b: 0b0000_0000,
                    f: FlagsRegister::from(0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        and(RegisterTarget::B)(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.a, 0b0000_0000);
        assert!(gameboy.cpu.registers.f.zero);
    }

    #[test]
    fn test_and_half_carry_flag() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.f.half_carry = false;
        and(RegisterTarget::B)(&mut gameboy);
        assert!(gameboy.cpu.registers.f.half_carry);
    }

    #[test]
    fn test_and_carry_flag() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.f.carry = true;
        and(RegisterTarget::B)(&mut gameboy);
        assert!(!gameboy.cpu.registers.f.carry);
    }

    #[test]
    fn test_and_subtract_flag() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.f.subtract = true;
        and(RegisterTarget::B)(&mut gameboy);
        assert!(!gameboy.cpu.registers.f.subtract);
    }
}
