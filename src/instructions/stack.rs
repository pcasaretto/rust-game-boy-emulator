use crate::cpu::Register16bTarget;
use crate::gameboy::Gameboy;

pub fn push(reg: Register16bTarget) -> impl Fn(&mut Gameboy) -> u8 {
    move |gameboy: &mut Gameboy| {
        let value = gameboy.cpu.registers.get_u16(reg);
        gameboy.stack_push(value);
        const TICKS: u8 = 16;
        TICKS
    }
}

pub fn pop(reg: Register16bTarget) -> impl Fn(&mut Gameboy) -> u8 {
    move |gameboy: &mut Gameboy| {
        let low = gameboy.read_byte(gameboy.cpu.registers.sp);
        gameboy.cpu.registers.sp = gameboy.cpu.registers.sp.wrapping_add(1);
        let high = gameboy.read_byte(gameboy.cpu.registers.sp);
        gameboy.cpu.registers.sp = gameboy.cpu.registers.sp.wrapping_add(1);
        gameboy
            .cpu
            .registers
            .set_u16(reg, u16::from_be_bytes([high, low]));
        const TICKS: u8 = 12;
        TICKS
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::{Registers, CPU};

    #[test]
    fn test_push() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    sp: 0xFFFE,
                    b: 0x01,
                    c: 0x02,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        push(Register16bTarget::BC)(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.sp, 0xFFFC);
        assert_eq!(gameboy.bus.memory[0xFFFD], 0x01);
        assert_eq!(gameboy.bus.memory[0xFFFC], 0x02);
    }

    #[test]
    fn test_pop() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    sp: 0xFFFC,
                    b: 0x00,
                    c: 0x00,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        gameboy.bus.memory[0xFFFC] = 0x01;
        gameboy.bus.memory[0xFFFD] = 0x02;
        pop(Register16bTarget::BC)(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.get_u16(Register16bTarget::BC), 0x0201);
        assert_eq!(gameboy.cpu.registers.sp, 0xFFFE);
    }
}
