use crate::cpu::{Register16bTarget, RegisterTarget};
use crate::gameboy::Gameboy;

pub fn ld_r16_n16(reg: Register16bTarget) -> impl Fn(&mut Gameboy) -> u8 {
    move |gameboy: &mut Gameboy| {
        let low = gameboy.read_next_byte();
        let high = gameboy.read_next_byte();
        let addr = u16::from_be_bytes([high, low]);
        gameboy.cpu.registers.set_u16(reg, addr);
        const TICKS: u8 = 12;
        TICKS
    }
}

pub fn ld_mem_at_d16_sp(gameboy: &mut Gameboy) -> u8 {
    let sp = gameboy.cpu.registers.get_u16(Register16bTarget::SP);
    let addr = u16::from_le_bytes([gameboy.read_next_byte(), gameboy.read_next_byte()]);
    let [low, high] = sp.to_le_bytes();
    gameboy.write_byte(addr, low);
    gameboy.write_byte(addr.wrapping_add(1), high);
    const TICKS: u8 = 20;
    TICKS
}

pub fn ld_r_r(src: RegisterTarget, dest: RegisterTarget) -> impl Fn(&mut Gameboy) -> u8 {
    move |gameboy: &mut Gameboy| {
        let value = gameboy.cpu.registers.get_u8(src);
        gameboy.cpu.registers.set_u8(dest, value);
        const TICKS: u8 = 4;
        TICKS
    }
}

pub fn ld_sp_hl(gameboy: &mut Gameboy) -> u8 {
    const CYCLES: u8 = 2;
    let hl = gameboy.cpu.registers.get_u16(Register16bTarget::HL);
    gameboy.cpu.registers.set_u16(Register16bTarget::SP, hl);
    CYCLES
}

pub fn ld_mem_at_d16_r(src: RegisterTarget) -> impl Fn(&mut Gameboy) -> u8 {
    move |gameboy: &mut Gameboy| {
        let low = gameboy.read_next_byte();
        let high = gameboy.read_next_byte();
        let addr = u16::from_be_bytes([high, low]);
        let value = gameboy.cpu.registers.get_u8(src);
        gameboy.write_byte(addr, value);
        const TICKS: u8 = 16;
        TICKS
    }
}

pub fn ld_r_mem_at_d16(target: RegisterTarget) -> impl Fn(&mut Gameboy) -> u8 {
    move |gameboy: &mut Gameboy| {
        let low = gameboy.read_next_byte();
        let high = gameboy.read_next_byte();
        let addr = u16::from_be_bytes([high, low]);
        let value = gameboy.read_byte(addr);
        gameboy.cpu.registers.set_u8(target, value);
        const TICKS: u8 = 16;
        TICKS
    }
}

pub fn ld_r_mem_at_r16(
    src: Register16bTarget,
    dest: RegisterTarget,
) -> impl Fn(&mut Gameboy) -> u8 {
    move |gameboy: &mut Gameboy| {
        let addr = gameboy.cpu.registers.get_u16(src);
        let value = gameboy.read_byte(addr);
        gameboy.cpu.registers.set_u8(dest, value);
        const TICKS: u8 = 8;
        TICKS
    }
}

pub fn ld_mem_at_r16_r(
    reg: Register16bTarget,
    target: RegisterTarget,
) -> impl Fn(&mut Gameboy) -> u8 {
    const TICKS: u8 = 8;
    move |gameboy: &mut Gameboy| {
        let addr = gameboy.cpu.registers.get_u16(reg);
        let value = gameboy.cpu.registers.get_u8(target);
        gameboy.write_byte(addr, value);
        TICKS
    }
}

pub fn ld_d8_mem_at_r16(reg: Register16bTarget) -> impl Fn(&mut Gameboy) -> u8 {
    move |gameboy: &mut Gameboy| {
        let addr = gameboy.cpu.registers.get_u16(reg);
        let value = gameboy.read_next_byte();
        gameboy.write_byte(addr, value);
        const TICKS: u8 = 12;
        TICKS
    }
}

pub fn ld_d8_r(target: RegisterTarget) -> impl Fn(&mut Gameboy) -> u8 {
    move |gameboy: &mut Gameboy| {
        let value = gameboy.read_next_byte();
        gameboy.cpu.registers.set_u8(target, value);
        const TICKS: u8 = 8;
        TICKS
    }
}

pub fn ld_mem_at_d8_a(gameboy: &mut Gameboy) -> u8 {
    let addr = 0xFF00 + gameboy.read_next_byte() as u16;
    let value = gameboy.cpu.registers.get_u8(RegisterTarget::A);
    gameboy.write_byte(addr, value);
    const TICKS: u8 = 12;
    TICKS
}

pub fn ld_a_mem_at_d8(gameboy: &mut Gameboy) -> u8 {
    let addr = 0xFF00 + gameboy.read_next_byte() as u16;
    let value = gameboy.read_byte(addr);
    gameboy.cpu.registers.set_u8(RegisterTarget::A, value);
    const TICKS: u8 = 12;
    TICKS
}

pub fn ld_mem_at_hl_r(target: RegisterTarget) -> impl Fn(&mut Gameboy) -> u8 {
    move |gameboy: &mut Gameboy| {
        let addr = gameboy.cpu.registers.get_u16(Register16bTarget::HL);
        let value = gameboy.cpu.registers.get_u8(target);
        gameboy.write_byte(addr, value);
        const TICKS: u8 = 8;
        TICKS
    }
}

pub fn ld_r_mem_at_hl(target: RegisterTarget) -> impl Fn(&mut Gameboy) -> u8 {
    move |gameboy: &mut Gameboy| {
        let addr = gameboy.cpu.registers.get_u16(Register16bTarget::HL);
        let value = gameboy.read_byte(addr);
        gameboy.cpu.registers.set_u8(target, value);
        const TICKS: u8 = 8;
        TICKS
    }
}

pub fn ld_mem_at_hl_a_inc(gameboy: &mut Gameboy) -> u8 {
    let addr = gameboy.cpu.registers.get_u16(Register16bTarget::HL);
    let value = gameboy.cpu.registers.get_u8(RegisterTarget::A);
    gameboy.write_byte(addr, value);
    gameboy
        .cpu
        .registers
        .set_u16(Register16bTarget::HL, addr.wrapping_add(1));
    const TICKS: u8 = 8;
    TICKS
}

pub fn ld_mem_at_hl_a_dec(gameboy: &mut Gameboy) -> u8 {
    let addr = gameboy.cpu.registers.get_u16(Register16bTarget::HL);
    let value = gameboy.cpu.registers.get_u8(RegisterTarget::A);
    gameboy.write_byte(addr, value);
    gameboy
        .cpu
        .registers
        .set_u16(Register16bTarget::HL, addr.wrapping_sub(1));
    const TICKS: u8 = 8;
    TICKS
}

pub fn ld_a_mem_at_hl_inc(gameboy: &mut Gameboy) -> u8 {
    let addr = gameboy.cpu.registers.get_u16(Register16bTarget::HL);
    let value = gameboy.read_byte(addr);
    gameboy.cpu.registers.set_u8(RegisterTarget::A, value);
    gameboy
        .cpu
        .registers
        .set_u16(Register16bTarget::HL, addr.wrapping_add(1));
    const TICKS: u8 = 8;
    TICKS
}

pub fn ld_a_mem_at_hl_dec(gameboy: &mut Gameboy) -> u8 {
    let hl = gameboy.cpu.registers.get_u16(Register16bTarget::HL);
    let value = gameboy.read_byte(hl);
    gameboy.cpu.registers.set_u8(RegisterTarget::A, value);
    gameboy
        .cpu
        .registers
        .set_u16(Register16bTarget::HL, hl.wrapping_sub(1));
    const TICKS: u8 = 8;
    TICKS
}

const LD_HIGH_OFFSET: u16 = 0xFF00;
pub fn ld_mem_at_c_a(gameboy: &mut Gameboy) -> u8 {
    let addr = LD_HIGH_OFFSET + gameboy.cpu.registers.get_u8(RegisterTarget::C) as u16;
    let value = gameboy.cpu.registers.get_u8(RegisterTarget::A);
    gameboy.write_byte(addr, value);
    const TICKS: u8 = 8;
    TICKS
}

pub fn ld_a_mem_at_c(gameboy: &mut Gameboy) -> u8 {
    let addr = LD_HIGH_OFFSET + gameboy.cpu.registers.get_u8(RegisterTarget::C) as u16;
    let value = gameboy.read_byte(addr);
    gameboy.cpu.registers.set_u8(RegisterTarget::A, value);
    const TICKS: u8 = 8;
    TICKS
}

pub fn ld_hl_sp_n8(gameboy: &mut Gameboy) -> u8 {
    let n = gameboy.read_next_byte() as i8;
    let sp = gameboy.cpu.registers.get_u16(Register16bTarget::SP);

    let (result, carry, half_carry) = if n < 0 {
        let t = n.unsigned_abs() as u16;
        let carry = sp & 0xFF < t;
        let half_carry = sp & 0xF < t & 0xF;
        (sp.wrapping_sub(t), !carry, !half_carry)
    } else {
        let carry = sp as u8 > 0xFF - n as u8;
        let a = sp as u8 & 0xF;
        let b = n as u8 & 0xF;
        let half_carry = a + b > 0xF;
        (sp.wrapping_add(n as u16), carry, half_carry)
    };
    gameboy.cpu.registers.set_u16(Register16bTarget::HL, result);
    gameboy.cpu.registers.f.zero = false;
    gameboy.cpu.registers.f.subtract = false;
    gameboy.cpu.registers.f.half_carry = half_carry;
    gameboy.cpu.registers.f.carry = carry;
    const TICKS: u8 = 12;
    TICKS
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::{Registers, CPU};

    #[test]
    fn test_ld_hl_sp_n8() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.pc = 0xC050;
        gameboy.cpu.registers.sp = 0xFFC0;
        gameboy.write_byte(0xC051, 0x03);

        ld_hl_sp_n8(&mut gameboy);

        assert_eq!(gameboy.cpu.registers.get_u16(Register16bTarget::HL), 0xFFC3);
    }

    #[test]
    fn test_ld_hl_sp_n8_negative() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.pc = 0xC050;
        gameboy.cpu.registers.sp = 0xFFC0;
        gameboy.write_byte(0xC051, 0b1111_1101_u8);

        ld_hl_sp_n8(&mut gameboy);

        assert_eq!(gameboy.cpu.registers.get_u16(Register16bTarget::HL), 0xFFBD);
    }

    // #[test]
    // fn test_ld_hl_sp_n8_carry_flag() {
    //     let mut gameboy = Gameboy::default();

    //     gameboy.cpu.registers.pc = 0xC050;
    //     gameboy.cpu.registers.sp = 0xDFFD;
    //     gameboy.write_byte(0xC051, 0x03);
    //     ld_hl_sp_n8(&mut gameboy);
    //     assert!(gameboy.cpu.registers.f.carry);

    //     gameboy.cpu.registers.pc = 0xC050;
    //     gameboy.cpu.registers.sp = 0xDF01;
    //     gameboy.write_byte(0xC051, 0xFE);
    //     ld_hl_sp_n8(&mut gameboy);
    //     assert!(gameboy.cpu.registers.f.carry);
    // }

    #[test]
    fn test_ld_hl_sp_n8_half_carry_flag() {
        let mut gameboy = Gameboy::default();

        // gameboy.cpu.registers.pc = 0xC050;
        // gameboy.cpu.registers.sp = 0x0FFE;
        // gameboy.write_byte(0xC051, 0x01);
        // ld_hl_sp_n8(&mut gameboy);
        // assert!(!gameboy.cpu.registers.f.half_carry);

        // gameboy.cpu.registers.pc = 0xC050;
        // gameboy.cpu.registers.sp = 0x0FFF;
        // gameboy.write_byte(0xC051, 0x01);
        // ld_hl_sp_n8(&mut gameboy);
        // assert!(gameboy.cpu.registers.f.half_carry);

        // gameboy.cpu.registers.pc = 0xC050;
        // gameboy.cpu.registers.sp = 0xDFFD;
        // gameboy.write_byte(0xC051, 0xFE);
        // ld_hl_sp_n8(&mut gameboy);
        // assert!(!gameboy.cpu.registers.f.half_carry);

        // gameboy.cpu.registers.pc = 0xC050;
        // gameboy.cpu.registers.sp = 0x000F;
        // gameboy.write_byte(0xC051, 0x01);
        // ld_hl_sp_n8(&mut gameboy);
        // assert!(gameboy.cpu.registers.f.half_carry);

        gameboy.cpu.registers.pc = 0xC050;
        gameboy.cpu.registers.sp = 0x0000;
        gameboy.write_byte(0xC051, 0xFF);
        ld_hl_sp_n8(&mut gameboy);
        assert!(!gameboy.cpu.registers.f.half_carry);
    }

    #[test]
    fn test_ld_r16_n16() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.pc = 0xC050;
        gameboy.bus.memory[0xC051] = 0x01;
        gameboy.bus.memory[0xC052] = 0x02;
        ld_r16_n16(Register16bTarget::BC)(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.get_u16(Register16bTarget::BC), 0x0201);
        assert_eq!(gameboy.cpu.registers.pc, 0xC052);
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
        gameboy.cpu.registers.set_u16(Register16bTarget::HL, 0xC050);
        gameboy.bus.memory[0xC050] = 0x01;
        ld_a_mem_at_hl_inc(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.get_u8(RegisterTarget::A), 0x01);
        assert_eq!(gameboy.cpu.registers.get_u16(Register16bTarget::HL), 0xC051);
    }

    #[test]
    fn test_ld_a_mem_at_hl_dec() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.set_u16(Register16bTarget::HL, 0xC050);
        gameboy.bus.memory[0xC050] = 0x01;
        ld_a_mem_at_hl_dec(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.get_u8(RegisterTarget::A), 0x01);
        assert_eq!(gameboy.cpu.registers.get_u16(Register16bTarget::HL), 0xC04F);
    }

    #[test]
    fn test_ld_mem_at_hl_a_inc() {
        let mut gameboy = Gameboy::default();
        let addr = 0xC050;
        gameboy.cpu.registers.set_u16(Register16bTarget::HL, addr);
        gameboy.cpu.registers.set_u8(RegisterTarget::A, 0x42);
        ld_mem_at_hl_a_inc(&mut gameboy);
        assert_eq!(gameboy.read_byte(addr), 0x42);
        assert_eq!(
            gameboy.cpu.registers.get_u16(Register16bTarget::HL),
            addr + 1
        );
    }

    #[test]
    fn test_ld_mem_at_hl_a_dec() {
        let mut gameboy = Gameboy::default();
        let addr = 0xC050;
        gameboy.cpu.registers.set_u16(Register16bTarget::HL, addr);
        gameboy.cpu.registers.set_u8(RegisterTarget::A, 0x42);
        ld_mem_at_hl_a_dec(&mut gameboy);
        assert_eq!(gameboy.read_byte(addr), 0x42);
        assert_eq!(
            gameboy.cpu.registers.get_u16(Register16bTarget::HL),
            addr - 1
        );
    }

    #[test]
    fn test_ld_r_mem_at_r16() {
        let mut gameboy = Gameboy::default();
        let addr = 0xC050;
        gameboy.cpu.registers.set_u16(Register16bTarget::BC, addr);
        gameboy.cpu.registers.set_u8(RegisterTarget::A, 0x34);
        ld_mem_at_r16_r(Register16bTarget::BC, RegisterTarget::A)(&mut gameboy);
        assert_eq!(gameboy.read_byte(addr), 0x34);
    }

    #[test]
    fn test_ld_mem_at_d16_r() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.pc = 0xC050;
        gameboy.cpu.registers.set_u8(RegisterTarget::A, 0x34);
        gameboy.bus.memory[0xC051] = 0x01;
        gameboy.bus.memory[0xC052] = 0xC1;
        ld_mem_at_d16_r(RegisterTarget::A)(&mut gameboy);
        assert_eq!(gameboy.bus.memory[0xC101], 0x34);
    }

    #[test]
    fn test_ld_r_mem_at_d16() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.pc = 0xC050;
        gameboy.bus.memory[0xC051] = 0x01;
        gameboy.bus.memory[0xC052] = 0xC1;
        gameboy.bus.memory[0xC101] = 0x34;
        ld_r_mem_at_d16(RegisterTarget::A)(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.get_u8(RegisterTarget::A), 0x34);
    }

    #[test]
    fn test_ld_u_16_r_advances_pc() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.pc = 0xC050;
        ld_mem_at_d16_r(RegisterTarget::A)(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.pc, 0xC052);
    }

    #[test]
    fn test_ld_d8_r() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.pc = 0xC050;
        gameboy.bus.memory[0xC051] = 0x34;
        ld_d8_r(RegisterTarget::A)(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.get_u8(RegisterTarget::A), 0x34);
    }

    #[test]
    fn test_ld_d8_mem_at_r16() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.set_u16(Register16bTarget::HL, 0xC100);
        gameboy.cpu.registers.pc = 0xC050;
        gameboy.bus.memory[0xC051] = 0x34;
        ld_d8_mem_at_r16(Register16bTarget::HL)(&mut gameboy);
        assert_eq!(gameboy.bus.memory[0xC100], 0x34);
    }

    #[test]
    fn test_ld_a_mem_at_d8() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.pc = 0xC050;
        gameboy.bus.memory[0xC051] = 0x34;
        gameboy.bus.memory[0xFF34] = 0x43;
        ld_a_mem_at_d8(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.get_u8(RegisterTarget::A), 0x43);
    }

    #[test]
    fn test_ld_mem_at_d8_a() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.pc = 0xC050;
        gameboy.bus.memory[0xC051] = 0x34;
        gameboy.cpu.registers.set_u8(RegisterTarget::A, 0x43);
        ld_mem_at_d8_a(&mut gameboy);
        assert_eq!(gameboy.read_byte(0xFF34), 0x43);
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
        let addr = 0xC050;
        gameboy.cpu.registers.set_u16(Register16bTarget::HL, addr);
        gameboy.cpu.registers.set_u8(RegisterTarget::A, 0x34);
        ld_mem_at_hl_r(RegisterTarget::A)(&mut gameboy);
        assert_eq!(gameboy.read_byte(addr), 0x34);
    }

    #[test]
    fn test_ld_r_mem_at_hl() {
        let mut gameboy = Gameboy::default();
        let addr = 0xC050;
        gameboy.cpu.registers.set_u16(Register16bTarget::HL, addr);
        gameboy.write_byte(addr, 0x34);
        ld_r_mem_at_hl(RegisterTarget::A)(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.get_u8(RegisterTarget::A), 0x34);
    }

    #[test]
    fn test_ld_mem_at_c_a() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.set_u8(RegisterTarget::C, 0x34);
        gameboy.cpu.registers.set_u8(RegisterTarget::A, 0x42);
        ld_mem_at_c_a(&mut gameboy);
        assert_eq!(gameboy.read_byte(0xFF34), 0x42);
    }

    #[test]
    fn test_ld_mem_at_d16_sp() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.pc = 0xC050;
        gameboy.bus.memory[0xC051] = 0x02;
        gameboy.bus.memory[0xC052] = 0xDD;
        gameboy.cpu.registers.sp = 0xDF7E;
        ld_mem_at_d16_sp(&mut gameboy);
        assert_eq!(gameboy.read_byte(0xDD02), 0x7E);
        assert_eq!(gameboy.read_byte(0xDD03), 0xDF);
        assert_eq!(gameboy.cpu.registers.pc, 0xC052);
    }
}
