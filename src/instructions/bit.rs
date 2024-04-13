use crate::cpu::Register16bTarget;
use crate::gameboy::Gameboy;

use super::RegisterTarget;

pub fn set_mem_at_hl(bit_position: u8) -> impl Fn(&mut Gameboy) -> u8 {
    move |gameboy: &mut Gameboy| {
        let hl = gameboy.cpu.registers.get_u16(Register16bTarget::HL);
        let value = gameboy.bus.read_byte(hl);
        let result = value | (1 << bit_position);

        gameboy.bus.write_byte(hl, result);
        const CYCLES: u8 = 4;
        CYCLES
    }
}

pub fn set_r(target: RegisterTarget, bit_position: u8) -> impl Fn(&mut Gameboy) -> u8 {
    move |gameboy: &mut Gameboy| {
        let value = gameboy.cpu.registers.get_u8(target);
        let result = value | (1 << bit_position);

        gameboy.cpu.registers.set_u8(target, result);
        const CYCLES: u8 = 4;
        CYCLES
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_set_mem_at_hl() {
        let mut gameboy = Gameboy::default();
        let addr = 0xC050;
        gameboy.cpu.registers.set_u16(Register16bTarget::HL, addr);
        gameboy.bus.write_byte(addr, 0b00000000);

        set_mem_at_hl(3)(&mut gameboy);

        assert_eq!(gameboy.bus.read_byte(addr), 0b00001000);
    }

    #[test]
    fn test_set_r() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.set_u8(RegisterTarget::A, 0b00000000);
        set_r(RegisterTarget::A, 3)(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.get_u8(RegisterTarget::A), 0b00001000);
    }
}
