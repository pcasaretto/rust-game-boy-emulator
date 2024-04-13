use crate::cpu::{Register16bTarget, RegisterTarget};
use crate::gameboy::Gameboy;

pub fn xor(target: RegisterTarget) -> impl Fn(&mut Gameboy) {
    move |gameboy: &mut Gameboy| {
        let r = gameboy.cpu.registers.get_u8(target);
        let a = gameboy.cpu.registers.get_u8(RegisterTarget::A);

        let value = a ^ r;

        gameboy.cpu.registers.set_u8(RegisterTarget::A, value);

        gameboy.cpu.registers.f.zero = value == 0;
        gameboy.cpu.registers.f.subtract = false;
        gameboy.cpu.registers.f.half_carry = false;
        gameboy.cpu.registers.f.carry = false;
    }
}

pub fn xor_mem_at_r16(reg: Register16bTarget) -> impl Fn(&mut Gameboy) {
    move |gameboy: &mut Gameboy| {
        let addr = gameboy.cpu.registers.get_u16(reg);
        let value = gameboy.bus.read_byte(addr);
        let a = gameboy.cpu.registers.get_u8(RegisterTarget::A);

        let result = a ^ value;

        gameboy.cpu.registers.set_u8(RegisterTarget::A, result);

        gameboy.cpu.registers.f.zero = result == 0;
        gameboy.cpu.registers.f.subtract = false;
        gameboy.cpu.registers.f.half_carry = false;
        gameboy.cpu.registers.f.carry = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::{FlagsRegister, Registers, CPU};

    #[test]
    fn test_xor() {
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
        xor(RegisterTarget::B)(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.a, 0b0110_0000);
    }

    #[test]
    fn test_xor_zero_flag() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    a: 0xFF,
                    b: 0xFF,
                    f: FlagsRegister::from(0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        xor(RegisterTarget::B)(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.a, 0);
        assert!(gameboy.cpu.registers.f.zero);
    }

    #[test]
    fn test_xor_half_carry_flag() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.f.half_carry = true;
        xor(RegisterTarget::B)(&mut gameboy);
        assert!(!gameboy.cpu.registers.f.half_carry);
    }

    #[test]
    fn test_xor_carry_flag() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.f.carry = true;
        xor(RegisterTarget::B)(&mut gameboy);
        assert!(!gameboy.cpu.registers.f.carry);
    }

    #[test]
    fn test_xor_subtract_flag() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.f.subtract = true;
        xor(RegisterTarget::B)(&mut gameboy);
        assert!(!gameboy.cpu.registers.f.subtract);
    }
}
