use crate::gameboy::Gameboy;

pub fn call_a16(gameboy: &mut Gameboy) -> u8 {
    let [pc_high, pc_low] = gameboy.cpu.registers.pc.wrapping_add(3).to_be_bytes();
    gameboy.cpu.registers.sp = gameboy.cpu.registers.sp.wrapping_sub(1);
    gameboy.write_byte(gameboy.cpu.registers.sp, pc_high);
    gameboy.cpu.registers.sp = gameboy.cpu.registers.sp.wrapping_sub(1);
    gameboy.write_byte(gameboy.cpu.registers.sp, pc_low);

    let low = gameboy.read_next_byte();
    let high = gameboy.read_next_byte();

    gameboy.cpu.registers.pc = u16::from_le_bytes([low, high]);
    const TICKS: u8 = 24;
    TICKS
}

pub fn call_nz_a16(gameboy: &mut Gameboy) -> u8 {
    if !gameboy.cpu.registers.f.zero {
        call_a16(gameboy)
    } else {
        gameboy.cpu.registers.pc = gameboy.cpu.registers.pc.wrapping_add(3);
        12
    }
}

pub fn call_z_a16(gameboy: &mut Gameboy) -> u8 {
    if gameboy.cpu.registers.f.zero {
        call_a16(gameboy)
    } else {
        gameboy.cpu.registers.pc = gameboy.cpu.registers.pc.wrapping_add(3);
        12
    }
}

pub fn call_nc_a16(gameboy: &mut Gameboy) -> u8 {
    if !gameboy.cpu.registers.f.carry {
        call_a16(gameboy)
    } else {
        gameboy.cpu.registers.pc = gameboy.cpu.registers.pc.wrapping_add(3);
        12
    }
}

pub fn call_c_a16(gameboy: &mut Gameboy) -> u8 {
    if gameboy.cpu.registers.f.carry {
        call_a16(gameboy)
    } else {
        gameboy.cpu.registers.pc = gameboy.cpu.registers.pc.wrapping_add(3);
        12
    }
}

pub fn ret(gameboy: &mut Gameboy) -> u8 {
    let low = gameboy.read_byte(gameboy.cpu.registers.sp);
    gameboy.cpu.registers.sp = gameboy.cpu.registers.sp.wrapping_add(1);
    let high = gameboy.read_byte(gameboy.cpu.registers.sp);
    gameboy.cpu.registers.sp = gameboy.cpu.registers.sp.wrapping_add(1);

    gameboy.cpu.registers.pc = u16::from_le_bytes([low, high]);
    const TICKS: u8 = 16;
    TICKS
}

pub fn reti(gameboy: &mut Gameboy) -> u8 {
    gameboy.interrupts_enabled = true;
    ret(gameboy)
}

pub fn ret_nz(gameboy: &mut Gameboy) -> u8 {
    if !gameboy.cpu.registers.f.zero {
        ret(gameboy);
        const TICKS: u8 = 20;
        return TICKS;
    }
    gameboy.cpu.registers.pc = gameboy.cpu.registers.pc.wrapping_add(1);
    const TICKS: u8 = 8;
    TICKS
}

pub fn ret_z(gameboy: &mut Gameboy) -> u8 {
    if gameboy.cpu.registers.f.zero {
        ret(gameboy);
        const TICKS: u8 = 20;
        return TICKS;
    }
    gameboy.cpu.registers.pc = gameboy.cpu.registers.pc.wrapping_add(1);
    const TICKS: u8 = 8;
    TICKS
}

pub fn ret_nc(gameboy: &mut Gameboy) -> u8 {
    if !gameboy.cpu.registers.f.carry {
        ret(gameboy);
        const TICKS: u8 = 20;
        return TICKS;
    }
    gameboy.cpu.registers.pc = gameboy.cpu.registers.pc.wrapping_add(1);
    const TICKS: u8 = 8;
    TICKS
}

pub fn ret_c(gameboy: &mut Gameboy) -> u8 {
    if gameboy.cpu.registers.f.carry {
        ret(gameboy);
        const TICKS: u8 = 20;
        return TICKS;
    }
    gameboy.cpu.registers.pc = gameboy.cpu.registers.pc.wrapping_add(1);
    const TICKS: u8 = 8;
    TICKS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_call_a16() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.pc = 0xC050;
        gameboy.cpu.registers.sp = 0xFFFE;
        gameboy.bus.memory[gameboy.cpu.registers.pc as usize + 1] = 0xCD;
        gameboy.bus.memory[gameboy.cpu.registers.pc as usize + 2] = 0xAB;
        call_a16(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.pc, 0xABCD);
        assert_eq!(gameboy.cpu.registers.sp, 0xFFFC);
        assert_eq!(gameboy.bus.memory[0xFFFD], 0xC0);
        assert_eq!(gameboy.bus.memory[0xFFFC], 0x53); // address of next instruction
    }

    #[test]
    fn test_ret() {
        let mut gameboy = Gameboy::default();
        gameboy.cpu.registers.sp = 0xFFFC;
        gameboy.bus.memory[0xFFFC] = 0x01;
        gameboy.bus.memory[0xFFFD] = 0x03;
        ret(&mut gameboy);
        assert_eq!(gameboy.cpu.registers.pc, 0x0301);
        assert_eq!(gameboy.cpu.registers.sp, 0xFFFE);
    }

    macro_rules! conditional_ret {
        ($($name:ident: $function:ident $flag_name:ident $flag_value:expr, $should_jump:expr,)*) => {
           $(
            #[test]
            fn $name() {
                let mut gameboy = Gameboy::default();
                gameboy.cpu.registers.pc = 0xC050;
                gameboy.cpu.registers.sp = 0xFFFC;
                gameboy.bus.memory[0xFFFC] = 0x01;
                gameboy.bus.memory[0xFFFD] = 0x03;
                gameboy.cpu.registers.f.$flag_name = $flag_value;
                $function(&mut gameboy);
                if $should_jump {
                assert_eq!(gameboy.cpu.registers.pc, 0x0301);
                assert_eq!(gameboy.cpu.registers.sp, 0xFFFE);

                } else {

                assert_eq!(gameboy.cpu.registers.pc, 0xC051);
                assert_eq!(gameboy.cpu.registers.sp, 0xFFFC);
                }
            })*
        };
    }

    conditional_ret!(
        test_ret_nz: ret_nz zero false, true,
        test_ret_nz_zero_flag: ret_nz zero true, false,
        test_ret_nc: ret_nc carry false, true,
        test_ret_nc_zero_flag: ret_nc carry true, false,
        test_rec_z: ret_z zero true, true,
        test_rec_z_zero_flag: ret_z zero false, false,
        test_rec_c: ret_c carry true, true,
        test_rec_c_zero_flag: ret_c carry false, false,
    );
}
