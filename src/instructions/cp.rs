use crate::cpu::RegisterTarget;
use crate::gameboy::Gameboy;

pub fn cp_d8() -> impl Fn(&mut Gameboy) {
    move |gameboy: &mut Gameboy| {
        let value = gameboy.read_next_byte();
        let a = gameboy.cpu.registers.get_u8(RegisterTarget::A);
        gameboy.cpu.registers.f.zero = a == value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::{FlagsRegister, Registers, CPU};

    #[test]
    fn test_cp() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    a: 13,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        gameboy.bus.write_byte(gameboy.cpu.registers.pc, 13);
        cp_d8()(&mut gameboy);
        assert!(gameboy.cpu.registers.f.zero);
    }

    #[test]
    fn test_cp_not_equal() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    a: 13,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        gameboy.bus.write_byte(gameboy.cpu.registers.pc, 14);
        cp_d8()(&mut gameboy);
        assert!(!gameboy.cpu.registers.f.zero);
    }
}
