use crate::cpu::RegisterTarget;
use crate::gameboy::Gameboy;

pub fn cp_d8(gameboy: &mut Gameboy) -> u8 {
    let value = gameboy.read_next_byte();
    let a = gameboy.cpu.registers.get_u8(RegisterTarget::A);
    gameboy.cpu.registers.f.zero = a == value;
    gameboy.cpu.registers.f.subtract = true;
    gameboy.cpu.registers.f.half_carry = (a & 0x0F) < (value & 0x0F);
    gameboy.cpu.registers.f.carry = a < value;
    const TICKS: u8 = 8;
    TICKS
}

pub fn cp(target: RegisterTarget) -> impl Fn(&mut Gameboy) -> u8 {
    move |gameboy| {
        let a = gameboy.cpu.registers.get_u8(RegisterTarget::A);
        let value = gameboy.cpu.registers.get_u8(target);
        gameboy.cpu.registers.f.zero = a == value;
        gameboy.cpu.registers.f.subtract = true;
        gameboy.cpu.registers.f.half_carry = (a & 0x0F) < (value & 0x0F);
        gameboy.cpu.registers.f.carry = a < value;
        const TICKS: u8 = 4;
        TICKS
    }
}

pub(crate) fn cp_mem_at_r16(hl: super::Register16bTarget) -> impl Fn(&mut Gameboy) -> u8 {
    move |gameboy| {
        let a = gameboy.cpu.registers.get_u8(RegisterTarget::A);
        let addr = gameboy.cpu.registers.get_u16(hl);
        let value = gameboy.read_byte(addr);
        gameboy.cpu.registers.f.zero = a == value;
        gameboy.cpu.registers.f.subtract = true;
        gameboy.cpu.registers.f.half_carry = (a & 0x0F) < (value & 0x0F);
        gameboy.cpu.registers.f.carry = a < value;
        const TICKS: u8 = 8;
        TICKS
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::{Register16bTarget, Registers, CPU};

    #[test]
    fn test_cp_d8() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    a: 13,
                    pc: 0xC050,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        gameboy.write_byte(gameboy.cpu.registers.pc + 1, 13);
        cp_d8(&mut gameboy);
        assert!(gameboy.cpu.registers.f.zero);
    }

    #[test]
    fn test_cp_d8_subtract_flag() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.a = 0x01;
        gameboy.cpu.registers.pc = 0xC050;
        gameboy.write_byte(0xC051, 0x01);
        cp_d8(&mut gameboy);
        assert!(gameboy.cpu.registers.f.subtract);
    }

    #[test]
    fn test_cp_d8_half_carry_flag() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.a = 0x10;
        gameboy.cpu.registers.pc = 0xC050;
        gameboy.write_byte(0xC051, 0x01);
        cp_d8(&mut gameboy);
        assert!(gameboy.cpu.registers.f.half_carry);
    }

    #[test]
    fn test_cp_d8_carry_flag() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.a = 0x00;
        gameboy.cpu.registers.pc = 0xC050;
        gameboy.write_byte(0xC051, 0x01);
        cp_d8(&mut gameboy);
        assert!(gameboy.cpu.registers.f.carry);
    }

    #[test]
    fn test_cp_d8_not_equal() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.a = 13;
        gameboy.cpu.registers.pc = 0xC050;
        gameboy.write_byte(0xC051, 14);
        cp_d8(&mut gameboy);
        assert!(!gameboy.cpu.registers.f.zero);
    }

    #[test]
    fn test_cp() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    a: 13,
                    b: 13,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        cp(RegisterTarget::B)(&mut gameboy);
        assert!(gameboy.cpu.registers.f.zero);
    }

    #[test]
    fn test_cp_not_equal() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    a: 13,
                    b: 14,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        cp(RegisterTarget::B)(&mut gameboy);
        assert!(!gameboy.cpu.registers.f.zero);
    }

    #[test]
    fn test_cp_mem_at_r16() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    a: 13,
                    h: 0xC0,
                    l: 0x00,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        gameboy.write_byte(0xC000, 13);
        cp_mem_at_r16(Register16bTarget::HL)(&mut gameboy);
        assert!(gameboy.cpu.registers.f.zero);
    }

    #[test]
    fn test_cp_mem_at_r16_not_equal() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    a: 14,
                    h: 0xC0,
                    l: 0x00,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        gameboy.write_byte(0xC000, 13);
        cp_mem_at_r16(Register16bTarget::HL)(&mut gameboy);
        assert!(!gameboy.cpu.registers.f.zero);
    }
}
