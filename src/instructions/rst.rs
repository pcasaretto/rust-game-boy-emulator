use crate::cpu::Register16bTarget;
use crate::gameboy::Gameboy;

pub fn rst(offset: u8) -> impl Fn(&mut Gameboy) -> u8 {
    move |gameboy: &mut Gameboy| {
        let pc = gameboy
            .cpu
            .registers
            .get_u16(Register16bTarget::PC)
            .wrapping_add(1);
        let [high, low] = pc.to_be_bytes();
        gameboy.cpu.registers.sp = gameboy.cpu.registers.sp.wrapping_sub(1);
        gameboy.bus.write_byte(gameboy.cpu.registers.sp, high);
        gameboy.cpu.registers.sp = gameboy.cpu.registers.sp.wrapping_sub(1);
        gameboy.bus.write_byte(gameboy.cpu.registers.sp, low);
        gameboy.cpu.registers.pc = u16::from(offset);
        const TICKS: u8 = 16;
        TICKS
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::{Registers, CPU};

    #[test]
    fn test_rst() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    sp: 0xFFFE,
                    pc: 0x1234,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        rst(0x08)(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.sp, 0xFFFC);
        assert_eq!(gameboy.bus.memory[0xFFFD], 0x12);
        assert_eq!(gameboy.bus.memory[0xFFFC], 0x35);
        assert_eq!(gameboy.cpu.registers.pc, 0x08);
    }
}
