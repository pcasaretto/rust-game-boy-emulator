use crate::cpu::{Register16bTarget, RegisterTarget};
use crate::gameboy::Gameboy;

pub fn ld_d16_r16(reg: Register16bTarget) -> impl Fn(&mut Gameboy) {
    move |gameboy: &mut Gameboy| {
        let low = gameboy.read_next_byte();
        let high = gameboy.read_next_byte();
        let addr = u16::from_be_bytes([high, low]);
        gameboy.cpu.registers.set_u16(reg, addr);
    }
}

pub fn ld_r_r(src: RegisterTarget, dest: RegisterTarget) -> impl Fn(&mut Gameboy) {
    move |gameboy: &mut Gameboy| {
        let value = gameboy.cpu.registers.get_u8(src);
        gameboy.cpu.registers.set_u8(dest, value);
    }
}

pub fn ld_sp_hl(gameboy: &mut Gameboy) {
    let hl = gameboy.cpu.registers.get_u16(Register16bTarget::HL);
    gameboy.cpu.registers.set_u16(Register16bTarget::SP, hl);
}

pub fn ld_r_mem_at_d16(src: RegisterTarget) -> impl Fn(&mut Gameboy) {
    move |gameboy: &mut Gameboy| {
        let addr = u16::from_be_bytes([gameboy.read_next_byte(), gameboy.read_next_byte()]);
        let value = gameboy.cpu.registers.get_u8(src);
        gameboy.bus.write_byte(addr, value);
    }
}

pub fn ld_r_mem_at_r16(reg: Register16bTarget, target: RegisterTarget) -> impl Fn(&mut Gameboy) {
    move |gameboy: &mut Gameboy| {
        let addr = gameboy.cpu.registers.get_u16(reg);
        let value = gameboy.cpu.registers.get_u8(target);
        gameboy.bus.write_byte(addr, value);
    }
}

pub fn ld_d8_mem_at_r16(reg: Register16bTarget) -> impl Fn(&mut Gameboy) {
    move |gameboy: &mut Gameboy| {
        let addr = gameboy.cpu.registers.get_u16(reg);
        let value = gameboy.read_next_byte();
        gameboy.bus.write_byte(addr, value);
    }
}

pub fn ld_d8_r(target: RegisterTarget) -> impl Fn(&mut Gameboy) {
    move |gameboy: &mut Gameboy| {
        let value = gameboy.read_next_byte();
        gameboy.cpu.registers.set_u8(target, value);
    }
}

pub fn ld_a_mem_at_d8() -> impl Fn(&mut Gameboy) {
    move |gameboy: &mut Gameboy| {
        let addr = 0xFF00 + gameboy.read_next_byte() as u16;
        let value = gameboy.cpu.registers.get_u8(RegisterTarget::A);
        gameboy.bus.write_byte(addr, value);
    }
}

pub fn ld_r_mem_at_hl(target: RegisterTarget) -> impl Fn(&mut Gameboy) {
    move |gameboy: &mut Gameboy| {
        let addr = gameboy.cpu.registers.get_u16(Register16bTarget::HL);
        let value = gameboy.cpu.registers.get_u8(target);
        gameboy.bus.write_byte(addr, value);
    }
}

pub fn ld_mem_at_hl_a_inc(gameboy: &mut Gameboy) {
    let hl = gameboy.cpu.registers.get_u16(Register16bTarget::HL);
    let value = gameboy.bus.read_byte(hl);
    gameboy.cpu.registers.set_u8(RegisterTarget::A, value);
    gameboy
        .cpu
        .registers
        .set_u16(Register16bTarget::HL, hl.wrapping_add(1));
}

pub fn ld_mem_at_hl_a_dec(gameboy: &mut Gameboy) {
    let hl = gameboy.cpu.registers.get_u16(Register16bTarget::HL);
    let value = gameboy.bus.read_byte(hl);
    gameboy.cpu.registers.set_u8(RegisterTarget::A, value);
    gameboy
        .cpu
        .registers
        .set_u16(Register16bTarget::HL, hl.wrapping_sub(1));
}

pub fn ld_a_mem_at_hl_inc(gameboy: &mut Gameboy) {
    let hl = gameboy.cpu.registers.get_u16(Register16bTarget::HL);
    let value = gameboy.bus.read_byte(hl);
    gameboy.cpu.registers.set_u8(RegisterTarget::A, value);
    gameboy
        .cpu
        .registers
        .set_u16(Register16bTarget::HL, hl.wrapping_add(1));
}

pub fn ld_a_mem_at_hl_dec(gameboy: &mut Gameboy) {
    let hl = gameboy.cpu.registers.get_u16(Register16bTarget::HL);
    let value = gameboy.bus.read_byte(hl);
    gameboy.cpu.registers.set_u8(RegisterTarget::A, value);
    gameboy
        .cpu
        .registers
        .set_u16(Register16bTarget::HL, hl.wrapping_sub(1));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::{Registers, CPU};

    #[test]
    fn test_ld_d16_r16() {
        let mut gameboy = Gameboy::default();
        gameboy.bus.memory[0] = 0x01;
        gameboy.bus.memory[1] = 0x02;
        ld_d16_r16(Register16bTarget::BC)(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.get_u16(Register16bTarget::BC), 0x0201);
        assert_eq!(gameboy.cpu.registers.pc, 2);
    }

    #[test]
    fn test_ld_r_r() {
        let mut gameboy = Gameboy {
            cpu: CPU {
                registers: Registers {
                    a: 0,
                    b: 1,
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        };
        ld_r_r(RegisterTarget::B, RegisterTarget::A)(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.get_u8(RegisterTarget::A), 0x01);
    }

    #[test]
    fn test_ld_a_mem_at_hl_inc() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.set_u16(Register16bTarget::HL, 0x1000);
        gameboy.bus.memory[0x1000] = 0x01;
        ld_a_mem_at_hl_inc(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.get_u8(RegisterTarget::A), 0x01);
        assert_eq!(gameboy.cpu.registers.get_u16(Register16bTarget::HL), 0x1001);
    }

    #[test]
    fn test_ld_a_mem_at_hl_dec() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.set_u16(Register16bTarget::HL, 0x1000);
        gameboy.bus.memory[0x1000] = 0x01;
        ld_a_mem_at_hl_dec(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.get_u8(RegisterTarget::A), 0x01);
        assert_eq!(gameboy.cpu.registers.get_u16(Register16bTarget::HL), 0x0FFF);
    }

    #[test]
    fn test_ld_mem_at_hl_a_inc() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.set_u16(Register16bTarget::HL, 0x1000);
        gameboy.bus.write_byte(0x1000, 0x42);
        ld_mem_at_hl_a_inc(&mut gameboy);
        assert_eq!(gameboy.bus.read_byte(0x1000), 0x42);
        assert_eq!(gameboy.cpu.registers.get_u16(Register16bTarget::HL), 0x1001);
    }

    #[test]
    fn test_ld_mem_at_hl_a_dec() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.set_u16(Register16bTarget::HL, 0x1000);
        gameboy.bus.write_byte(0x1000, 0x42);
        ld_mem_at_hl_a_dec(&mut gameboy);
        assert_eq!(gameboy.bus.read_byte(0x1000), 0x42);
        assert_eq!(gameboy.cpu.registers.get_u16(Register16bTarget::HL), 0x0fff);
    }

    #[test]
    fn test_ld_r_mem_at_r16() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.set_u16(Register16bTarget::BC, 0x1000);
        gameboy.cpu.registers.set_u8(RegisterTarget::A, 0x34);
        ld_r_mem_at_r16(Register16bTarget::BC, RegisterTarget::A)(&mut gameboy);
        assert_eq!(gameboy.bus.read_byte(0x1000), 0x34);
    }

    #[test]
    fn test_ld_u_16_r() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.set_u8(RegisterTarget::A, 0x34);
        gameboy.bus.memory[0] = 0x01;
        gameboy.bus.memory[1] = 0x23;
        ld_r_mem_at_d16(RegisterTarget::A)(&mut gameboy);
        assert_eq!(gameboy.bus.memory[0x0123], 0x34);
    }

    #[test]
    fn test_ld_u_16_r_advances_pc() {
        let mut gameboy = Gameboy::default();
        ld_r_mem_at_d16(RegisterTarget::A)(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.pc, 2);
    }

    #[test]
    fn test_ld_d8_r() {
        let mut gameboy = Gameboy::default();
        gameboy.bus.memory[0] = 0x34;
        ld_d8_r(RegisterTarget::A)(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.get_u8(RegisterTarget::A), 0x34);
    }

    #[test]
    fn test_ld_d8_mem_at_r16() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.set_u16(Register16bTarget::HL, 0x1000);
        gameboy.bus.memory[0] = 0x34;
        ld_d8_mem_at_r16(Register16bTarget::HL)(&mut gameboy);
        assert_eq!(gameboy.bus.memory[0x1000], 0x34);
    }

    #[test]
    fn test_ld_a_mem_at_d8() {
        let mut gameboy = Gameboy::default();
        gameboy.bus.memory[0] = 0x34;
        gameboy.cpu.registers.set_u8(RegisterTarget::A, 0x42);
        ld_a_mem_at_d8()(&mut gameboy);
        assert_eq!(gameboy.bus.read_byte(0xFF34), 0x42);
    }

    #[test]
    fn test_sp_hl() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.set_u16(Register16bTarget::HL, 0x1234);
        ld_sp_hl(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.get_u16(Register16bTarget::SP), 0x1234);
    }

    #[test]
    fn test_ld_mem_at_hl_r() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.set_u16(Register16bTarget::HL, 0x1000);
        gameboy.cpu.registers.set_u8(RegisterTarget::A, 0x34);
        ld_r_mem_at_hl(RegisterTarget::A)(&mut gameboy);
        assert_eq!(gameboy.bus.read_byte(0x1000), 0x34);
    }
}
