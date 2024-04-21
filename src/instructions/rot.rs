use crate::cpu::Register16bTarget;
use crate::cpu::RegisterTarget;
use crate::gameboy::Gameboy;

pub fn rl_a(gameboy: &mut Gameboy) -> u8 {
    //     Rotate A left through Carry flag.
    // Flags affected:
    // Z - Set if result is zero.
    // N - Reset.
    // H - Reset.
    // C - Contains old bit 7 data.
    let value = gameboy.cpu.registers.a;
    let carry = gameboy.cpu.registers.f.carry;
    let new_carry = value & 0x80 != 0;
    let new_value = (value << 1) | (carry as u8);
    gameboy.cpu.registers.a = new_value;
    gameboy.cpu.registers.f.zero = new_value == 0;
    gameboy.cpu.registers.f.subtract = false;
    gameboy.cpu.registers.f.half_carry = false;
    gameboy.cpu.registers.f.carry = new_carry;
    const TICKS: u8 = 4;
    TICKS
}

pub fn rlc_a(gameboy: &mut Gameboy) -> u8 {
    // Rotate A left. Old bit 7 to Carry flag.
    //     Flags affected:
    // Z - Set if result is zero.
    // N - Reset.
    // H - Reset.
    // C - Contains old bit 7 data.
    let value = gameboy.cpu.registers.a;
    let new_carry = value & 0x80 != 0;
    let new_value = (value << 1) | (value >> 7);
    gameboy.cpu.registers.a = new_value;
    gameboy.cpu.registers.f.zero = new_value == 0;
    gameboy.cpu.registers.f.subtract = false;
    gameboy.cpu.registers.f.half_carry = false;
    gameboy.cpu.registers.f.carry = new_carry;

    const TICKS: u8 = 4;
    TICKS
}

pub fn rr_a(gameboy: &mut Gameboy) -> u8 {
    let value = gameboy.cpu.registers.a;
    let carry = gameboy.cpu.registers.f.carry;
    let new_carry = value & 0x01 != 0;
    let new_value = (value >> 1) | ((carry as u8) << 7);
    gameboy.cpu.registers.a = new_value;
    gameboy.cpu.registers.f.zero = false;
    gameboy.cpu.registers.f.subtract = false;
    gameboy.cpu.registers.f.half_carry = false;
    gameboy.cpu.registers.f.carry = new_carry;
    const TICKS: u8 = 4;
    TICKS
}

pub fn rrc_a(gameboy: &mut Gameboy) -> u8 {
    let value = gameboy.cpu.registers.a;
    let new_carry = value & 0x01 != 0;
    let new_value = (value >> 1) | (value << 7);
    gameboy.cpu.registers.a = new_value;
    gameboy.cpu.registers.f.zero = false;
    gameboy.cpu.registers.f.subtract = false;
    gameboy.cpu.registers.f.half_carry = false;
    gameboy.cpu.registers.f.carry = new_carry;

    const TICKS: u8 = 4;
    TICKS
}

pub fn rl_r(target: RegisterTarget) -> impl Fn(&mut Gameboy) -> u8 {
    move |gameboy: &mut Gameboy| {
        let value = gameboy.cpu.registers.get_u8(target);
        let carry = gameboy.cpu.registers.f.carry;
        let new_carry = value & 0x80 != 0;
        let new_value = (value << 1) | (carry as u8);
        gameboy.cpu.registers.f.carry = new_carry;
        gameboy.cpu.registers.f.zero = new_value == 0;
        gameboy.cpu.registers.f.subtract = false;
        gameboy.cpu.registers.f.half_carry = false;
        gameboy.cpu.registers.set_u8(target, new_value);
        const TICKS: u8 = 8;
        TICKS
    }
}

pub fn rl_mem_at_hl(gameboy: &mut Gameboy) -> u8 {
    let addr = gameboy.cpu.registers.get_u16(Register16bTarget::HL);
    let value = gameboy.bus.read_byte(addr);
    let carry = gameboy.cpu.registers.f.carry;
    let new_carry = value & 0x80 != 0;
    let new_value = (value << 1) | (carry as u8);
    gameboy.cpu.registers.f.carry = new_carry;
    gameboy.cpu.registers.f.zero = new_value == 0;
    gameboy.cpu.registers.f.subtract = false;
    gameboy.cpu.registers.f.half_carry = false;
    gameboy.bus.write_byte(addr, new_value);
    const TICKS: u8 = 16;
    TICKS
}

pub fn rr_r(target: RegisterTarget) -> impl Fn(&mut Gameboy) -> u8 {
    move |gameboy: &mut Gameboy| {
        let value = gameboy.cpu.registers.get_u8(target);
        let carry = gameboy.cpu.registers.f.carry;
        let new_carry = value & 0x01 != 0;
        let new_value = (value >> 1) | ((carry as u8) << 7);
        gameboy.cpu.registers.f.carry = new_carry;
        gameboy.cpu.registers.f.zero = new_value == 0;
        gameboy.cpu.registers.f.subtract = false;
        gameboy.cpu.registers.f.half_carry = false;
        gameboy.cpu.registers.set_u8(target, new_value);
        const TICKS: u8 = 8;
        TICKS
    }
}

pub fn rr_mem_at_hl(gameboy: &mut Gameboy) -> u8 {
    let addr = gameboy.cpu.registers.get_u16(Register16bTarget::HL);
    let value = gameboy.bus.read_byte(addr);
    let carry = gameboy.cpu.registers.f.carry;
    let new_carry = value & 0x01 != 0;
    let new_value = (value >> 1) | ((carry as u8) << 7);
    gameboy.cpu.registers.f.carry = new_carry;
    gameboy.cpu.registers.f.zero = new_value == 0;
    gameboy.cpu.registers.f.subtract = false;
    gameboy.cpu.registers.f.half_carry = false;
    gameboy.bus.write_byte(addr, new_value);
    const TICKS: u8 = 16;
    TICKS
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! rr_r_tests {
        ($($name:ident: $value:expr,)*) => {
           $(
            #[test]
            fn $name() {
                let (register_value, carry_flag, expected_value, expected_carry) = $value;
                let mut gameboy = Gameboy::default();
                gameboy.cpu.registers.f.carry = carry_flag;
                gameboy.cpu.registers.set_u8(RegisterTarget::A, register_value);
                rr_r(RegisterTarget::A)(&mut gameboy);
                assert_eq!(gameboy.cpu.registers.get_u8(RegisterTarget::A), expected_value);
                assert_eq!(gameboy.cpu.registers.f.carry, expected_carry);
                assert_eq!(gameboy.cpu.registers.f.zero, expected_value == 0);
                assert!(!gameboy.cpu.registers.f.subtract);
                assert!(!gameboy.cpu.registers.f.half_carry);
            })*
        };
    }

    rr_r_tests! {
      rr_r_test_0: (0b00000001, false, 0b00000000, true),
      rr_r_test_1: (0b00000001, true, 0b10000000, true),
      rr_r_test_2: (0b10000000, true, 0b11000000, false),
      rr_r_test_3: (0b10000000, false, 0b01000000, false),
    }

    macro_rules! rl_r_tests {
        ($($name:ident: $value:expr,)*) => {
           $(
            #[test]
            fn $name() {
                let (register_value, carry_flag, expected_value, expected_carry) = $value;
                let mut gameboy = Gameboy::default();
                gameboy.cpu.registers.f.carry = carry_flag;
                gameboy.cpu.registers.set_u8(RegisterTarget::A, register_value);
                rl_r(RegisterTarget::A)(&mut gameboy);
                assert_eq!(gameboy.cpu.registers.get_u8(RegisterTarget::A), expected_value);
                assert_eq!(gameboy.cpu.registers.f.carry, expected_carry);
                assert_eq!(gameboy.cpu.registers.f.zero, expected_value == 0);
                assert!(!gameboy.cpu.registers.f.subtract);
                assert!(!gameboy.cpu.registers.f.half_carry);
            })*
        };
    }

    rl_r_tests! {
      rl_r_test_0: (0b10000000, false, 0b00000000, true),
      rl_r_test_1: (0b10000000, true, 0b00000001, true),
      rl_r_test_2: (0b00000001, true, 0b00000011, false),
      rl_r_test_3: (0b00000001, false, 0b00000010, false),
    }

    macro_rules! rr_mem_at_hl_tests {
        ($($name:ident: $value:expr,)*) => {
           $(
            #[test]
            fn $name() {
                let (memory_value, carry_flag, expected_value, expected_carry) = $value;
                let mut gameboy = Gameboy::default();
                gameboy.cpu.registers.f.carry = carry_flag;
                gameboy.cpu.registers.set_u16(Register16bTarget::HL, 0xC050);
                gameboy.bus.write_byte(0xC050, memory_value);
                rr_mem_at_hl(&mut gameboy);
                assert_eq!(gameboy.bus.read_byte(0xC050), expected_value);
                assert_eq!(gameboy.cpu.registers.f.carry, expected_carry);
                assert_eq!(gameboy.cpu.registers.f.zero, expected_value == 0);
                assert!(!gameboy.cpu.registers.f.subtract);
                assert!(!gameboy.cpu.registers.f.half_carry);
            })*
        };
    }

    rr_mem_at_hl_tests! {
      rr_mem_at_hl_test_0: (0b00000001, false, 0b00000000, true),
      rr_mem_at_hl_test_1: (0b00000001, true, 0b10000000, true),
      rr_mem_at_hl_test_2: (0b10000000, true, 0b11000000, false),
      rr_mem_at_hl_test_3: (0b10000000, false, 0b01000000, false),
    }

    macro_rules! rl_mem_at_hl_tests {
        ($($name:ident: $value:expr,)*) => {
           $(
            #[test]
            fn $name() {
                let (memory_value, carry_flag, expected_value, expected_carry) = $value;
                let mut gameboy = Gameboy::default();
                gameboy.cpu.registers.f.carry = carry_flag;
                gameboy.cpu.registers.set_u16(Register16bTarget::HL, 0xC050);
                gameboy.bus.write_byte(0xC050, memory_value);
                rl_mem_at_hl(&mut gameboy);
                assert_eq!(gameboy.bus.read_byte(0xC050), expected_value);
                assert_eq!(gameboy.cpu.registers.f.carry, expected_carry);
                assert_eq!(gameboy.cpu.registers.f.zero, expected_value == 0);
                assert!(!gameboy.cpu.registers.f.subtract);
                assert!(!gameboy.cpu.registers.f.half_carry);
            })*
        };
    }

    rl_mem_at_hl_tests! {
      rl_mem_at_hl_test_0: (0b10000000, false, 0b00000000, true),
      rl_mem_at_hl_test_1: (0b10000000, true, 0b00000001, true),
      rl_mem_at_hl_test_2: (0b00000001, true, 0b00000011, false),
      rl_mem_at_hl_test_3: (0b00000001, false, 0b00000010, false),
    }
}
