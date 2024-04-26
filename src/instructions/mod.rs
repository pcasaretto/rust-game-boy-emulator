mod adc;
mod add;
mod and;
mod bit;
mod call;
mod cp;
mod dec;
mod inc;
mod int;
mod jmp;
mod ld;
mod misc;
mod nop;
mod or;
mod rot;
mod rst;
mod sbc;
mod shift;
mod stack;
mod sub;
mod swap;
mod xor;

use super::cpu::*;
use super::gameboy;

/// Represents an instruction that can be executed by the Gameboy.
/// The instruction is a function that takes a mutable reference to a Gameboy and returns
/// the number of t-states (system clock ticks) that the instruction took to execute.
pub type Instruction = dyn Fn(&mut gameboy::Gameboy) -> u8;

/// pai means post advance instruction
fn pai(instruction: impl Fn(&mut gameboy::Gameboy) -> u8) -> impl Fn(&mut gameboy::Gameboy) -> u8 {
    move |gameboy: &mut gameboy::Gameboy| {
        let ticks = instruction(gameboy);
        gameboy.cpu.registers.pc = gameboy.cpu.registers.pc.wrapping_add(1);
        ticks
    }
}

fn dpai(instruction: impl Fn(&mut gameboy::Gameboy) -> u8) -> impl Fn(&mut gameboy::Gameboy) -> u8 {
    move |gameboy: &mut gameboy::Gameboy| {
        let ticks = instruction(gameboy);
        gameboy.cpu.registers.pc = gameboy.cpu.registers.pc.wrapping_add(2);
        ticks
    }
}

pub fn from_byte(byte: u8) -> Box<Instruction> {
    match byte {
        0x00 => Box::new(pai(nop::nop)),
        0x10 => Box::new(pai(misc::stop)),

        0x02 => Box::new(pai(ld::ld_mem_at_r16_r(
            Register16bTarget::BC,
            RegisterTarget::A,
        ))),
        0x08 => Box::new(pai(ld::ld_mem_at_d16_sp)),
        0x12 => Box::new(pai(ld::ld_mem_at_r16_r(
            Register16bTarget::DE,
            RegisterTarget::A,
        ))),

        0x01 => Box::new(pai(ld::ld_r16_n16(Register16bTarget::BC))),
        0x11 => Box::new(pai(ld::ld_r16_n16(Register16bTarget::DE))),
        0x21 => Box::new(pai(ld::ld_r16_n16(Register16bTarget::HL))),
        0x31 => Box::new(pai(ld::ld_r16_n16(Register16bTarget::SP))),

        0x0B => Box::new(pai(dec::dec_r16(Register16bTarget::BC))),
        0x1B => Box::new(pai(dec::dec_r16(Register16bTarget::DE))),
        0x2B => Box::new(pai(dec::dec_r16(Register16bTarget::HL))),
        0x3B => Box::new(pai(dec::dec_r16(Register16bTarget::SP))),

        0x09 => Box::new(pai(add::add_hl_r16(Register16bTarget::BC))),
        0x19 => Box::new(pai(add::add_hl_r16(Register16bTarget::DE))),
        0x29 => Box::new(pai(add::add_hl_r16(Register16bTarget::HL))),
        0x39 => Box::new(pai(add::add_hl_r16(Register16bTarget::SP))),

        0x07 => Box::new(pai(rot::rlc_a)),
        0x17 => Box::new(pai(rot::rl_a)),
        0x0F => Box::new(pai(rot::rrc_a)),
        0x1F => Box::new(pai(rot::rr_a)),

        0x0A => Box::new(pai(ld::ld_r_mem_at_r16(
            Register16bTarget::BC,
            RegisterTarget::A,
        ))),
        0x1A => Box::new(pai(ld::ld_r_mem_at_r16(
            Register16bTarget::DE,
            RegisterTarget::A,
        ))),

        0x03 => Box::new(pai(inc::inc_r16(Register16bTarget::BC))),
        0x13 => Box::new(pai(inc::inc_r16(Register16bTarget::DE))),
        0x23 => Box::new(pai(inc::inc_r16(Register16bTarget::HL))),
        0x33 => Box::new(pai(inc::inc_r16(Register16bTarget::SP))),

        0x04 => Box::new(pai(inc::inc_r(RegisterTarget::B))),
        0x14 => Box::new(pai(inc::inc_r(RegisterTarget::D))),
        0x24 => Box::new(pai(inc::inc_r(RegisterTarget::H))),
        0x0C => Box::new(pai(inc::inc_r(RegisterTarget::C))),
        0x1C => Box::new(pai(inc::inc_r(RegisterTarget::E))),
        0x2C => Box::new(pai(inc::inc_r(RegisterTarget::L))),
        0x3C => Box::new(pai(inc::inc_r(RegisterTarget::A))),

        0x27 => Box::new(pai(misc::daa)),

        0x35 => Box::new(pai(dec::dec_mem_at_hl)),
        0x05 => Box::new(pai(dec::dec_r(RegisterTarget::B))),
        0x15 => Box::new(pai(dec::dec_r(RegisterTarget::D))),
        0x25 => Box::new(pai(dec::dec_r(RegisterTarget::H))),
        0x0D => Box::new(pai(dec::dec_r(RegisterTarget::C))),
        0x1D => Box::new(pai(dec::dec_r(RegisterTarget::E))),
        0x2D => Box::new(pai(dec::dec_r(RegisterTarget::L))),
        0x3D => Box::new(pai(dec::dec_r(RegisterTarget::A))),

        0x18 => Box::new(jmp::jr),
        0x20 => Box::new(jmp::jr_nz),
        0x30 => Box::new(jmp::jr_nc),
        0x28 => Box::new(jmp::jr_z),
        0x38 => Box::new(jmp::jr_c),

        0x22 => Box::new(pai(ld::ld_mem_at_hl_a_inc)),
        0x32 => Box::new(pai(ld::ld_mem_at_hl_a_dec)),
        0x2A => Box::new(pai(ld::ld_a_mem_at_hl_inc)),
        0x3A => Box::new(pai(ld::ld_a_mem_at_hl_dec)),
        // 0x33 => Box::new(inc::inc_sp()),
        0x06 => Box::new(pai(ld::ld_d8_r(RegisterTarget::B))),
        0x16 => Box::new(pai(ld::ld_d8_r(RegisterTarget::D))),
        0x26 => Box::new(pai(ld::ld_d8_r(RegisterTarget::H))),
        0x36 => Box::new(pai(ld::ld_d8_mem_at_r16(Register16bTarget::HL))),
        0x0E => Box::new(pai(ld::ld_d8_r(RegisterTarget::C))),
        0x1E => Box::new(pai(ld::ld_d8_r(RegisterTarget::E))),
        0x2E => Box::new(pai(ld::ld_d8_r(RegisterTarget::L))),
        0x3E => Box::new(pai(ld::ld_d8_r(RegisterTarget::A))),
        0xC3 => Box::new(jmp::jmp_a16),
        0x40 => Box::new(pai(ld::ld_r_r(RegisterTarget::B, RegisterTarget::B))),
        0x41 => Box::new(pai(ld::ld_r_r(RegisterTarget::C, RegisterTarget::B))),
        0x42 => Box::new(pai(ld::ld_r_r(RegisterTarget::D, RegisterTarget::B))),
        0x43 => Box::new(pai(ld::ld_r_r(RegisterTarget::E, RegisterTarget::B))),
        0x44 => Box::new(pai(ld::ld_r_r(RegisterTarget::H, RegisterTarget::B))),
        0x45 => Box::new(pai(ld::ld_r_r(RegisterTarget::L, RegisterTarget::B))),
        0x46 => Box::new(pai(ld::ld_r_mem_at_hl(RegisterTarget::B))),
        0x47 => Box::new(pai(ld::ld_r_r(RegisterTarget::A, RegisterTarget::B))),
        0x48 => Box::new(pai(ld::ld_r_r(RegisterTarget::B, RegisterTarget::C))),
        0x49 => Box::new(pai(ld::ld_r_r(RegisterTarget::C, RegisterTarget::C))),
        0x4a => Box::new(pai(ld::ld_r_r(RegisterTarget::D, RegisterTarget::C))),
        0x4b => Box::new(pai(ld::ld_r_r(RegisterTarget::E, RegisterTarget::C))),
        0x4c => Box::new(pai(ld::ld_r_r(RegisterTarget::H, RegisterTarget::C))),
        0x4d => Box::new(pai(ld::ld_r_r(RegisterTarget::L, RegisterTarget::C))),
        0x4E => Box::new(pai(ld::ld_r_mem_at_hl(RegisterTarget::C))),
        0x4f => Box::new(pai(ld::ld_r_r(RegisterTarget::A, RegisterTarget::C))),
        0x50 => Box::new(pai(ld::ld_r_r(RegisterTarget::B, RegisterTarget::D))),
        0x51 => Box::new(pai(ld::ld_r_r(RegisterTarget::C, RegisterTarget::D))),
        0x52 => Box::new(pai(ld::ld_r_r(RegisterTarget::D, RegisterTarget::D))),
        0x53 => Box::new(pai(ld::ld_r_r(RegisterTarget::E, RegisterTarget::D))),
        0x54 => Box::new(pai(ld::ld_r_r(RegisterTarget::H, RegisterTarget::D))),
        0x55 => Box::new(pai(ld::ld_r_r(RegisterTarget::L, RegisterTarget::D))),
        0x56 => Box::new(pai(ld::ld_r_mem_at_hl(RegisterTarget::D))),
        0x57 => Box::new(pai(ld::ld_r_r(RegisterTarget::A, RegisterTarget::D))),
        0x58 => Box::new(pai(ld::ld_r_r(RegisterTarget::B, RegisterTarget::E))),
        0x59 => Box::new(pai(ld::ld_r_r(RegisterTarget::C, RegisterTarget::E))),
        0x5A => Box::new(pai(ld::ld_r_r(RegisterTarget::D, RegisterTarget::E))),
        0x5B => Box::new(pai(ld::ld_r_r(RegisterTarget::E, RegisterTarget::E))),
        0x5C => Box::new(pai(ld::ld_r_r(RegisterTarget::H, RegisterTarget::E))),
        0x5D => Box::new(pai(ld::ld_r_r(RegisterTarget::L, RegisterTarget::E))),
        0x5E => Box::new(pai(ld::ld_r_mem_at_hl(RegisterTarget::E))),
        0x5F => Box::new(pai(ld::ld_r_r(RegisterTarget::A, RegisterTarget::E))),
        0x60 => Box::new(pai(ld::ld_r_r(RegisterTarget::B, RegisterTarget::H))),
        0x61 => Box::new(pai(ld::ld_r_r(RegisterTarget::C, RegisterTarget::H))),
        0x62 => Box::new(pai(ld::ld_r_r(RegisterTarget::D, RegisterTarget::H))),
        0x63 => Box::new(pai(ld::ld_r_r(RegisterTarget::E, RegisterTarget::H))),
        0x64 => Box::new(pai(ld::ld_r_r(RegisterTarget::H, RegisterTarget::H))),
        0x65 => Box::new(pai(ld::ld_r_r(RegisterTarget::L, RegisterTarget::H))),
        0x66 => Box::new(pai(ld::ld_r_mem_at_hl(RegisterTarget::H))),
        0x67 => Box::new(pai(ld::ld_r_r(RegisterTarget::A, RegisterTarget::H))),
        0x68 => Box::new(pai(ld::ld_r_r(RegisterTarget::B, RegisterTarget::L))),
        0x69 => Box::new(pai(ld::ld_r_r(RegisterTarget::C, RegisterTarget::L))),
        0x6A => Box::new(pai(ld::ld_r_r(RegisterTarget::D, RegisterTarget::L))),
        0x6B => Box::new(pai(ld::ld_r_r(RegisterTarget::E, RegisterTarget::L))),
        0x6C => Box::new(pai(ld::ld_r_r(RegisterTarget::H, RegisterTarget::L))),
        0x6D => Box::new(pai(ld::ld_r_r(RegisterTarget::L, RegisterTarget::L))),
        0x6E => Box::new(pai(ld::ld_r_mem_at_hl(RegisterTarget::L))),
        0x6F => Box::new(pai(ld::ld_r_r(RegisterTarget::A, RegisterTarget::L))),
        0x70 => Box::new(pai(ld::ld_mem_at_hl_r(RegisterTarget::B))),
        0x71 => Box::new(pai(ld::ld_mem_at_hl_r(RegisterTarget::C))),
        0x72 => Box::new(pai(ld::ld_mem_at_hl_r(RegisterTarget::D))),
        0x73 => Box::new(pai(ld::ld_mem_at_hl_r(RegisterTarget::E))),
        0x74 => Box::new(pai(ld::ld_mem_at_hl_r(RegisterTarget::H))),
        0x75 => Box::new(pai(ld::ld_mem_at_hl_r(RegisterTarget::L))),
        0x77 => Box::new(pai(ld::ld_mem_at_hl_r(RegisterTarget::A))),
        0x78 => Box::new(pai(ld::ld_r_r(RegisterTarget::B, RegisterTarget::A))),
        0x79 => Box::new(pai(ld::ld_r_r(RegisterTarget::C, RegisterTarget::A))),
        0x7A => Box::new(pai(ld::ld_r_r(RegisterTarget::D, RegisterTarget::A))),
        0x7B => Box::new(pai(ld::ld_r_r(RegisterTarget::E, RegisterTarget::A))),
        0x7C => Box::new(pai(ld::ld_r_r(RegisterTarget::H, RegisterTarget::A))),
        0x7D => Box::new(pai(ld::ld_r_r(RegisterTarget::L, RegisterTarget::A))),
        0x7E => Box::new(pai(ld::ld_r_mem_at_hl(RegisterTarget::A))),
        0x7F => Box::new(pai(ld::ld_r_r(RegisterTarget::A, RegisterTarget::A))),
        0x80 => Box::new(pai(add::add(RegisterTarget::B))),
        0x81 => Box::new(pai(add::add(RegisterTarget::C))),
        0x82 => Box::new(pai(add::add(RegisterTarget::D))),
        0x83 => Box::new(pai(add::add(RegisterTarget::E))),
        0x84 => Box::new(pai(add::add(RegisterTarget::H))),
        0x85 => Box::new(pai(add::add(RegisterTarget::L))),
        0x86 => Box::new(pai(add::add_mem_at_hl)),
        0x87 => Box::new(pai(add::add(RegisterTarget::A))),
        0x88 => Box::new(pai(adc::adc(RegisterTarget::B))),
        0x89 => Box::new(pai(adc::adc(RegisterTarget::C))),
        0x8A => Box::new(pai(adc::adc(RegisterTarget::D))),
        0x8B => Box::new(pai(adc::adc(RegisterTarget::E))),
        0x8C => Box::new(pai(adc::adc(RegisterTarget::H))),
        0x8D => Box::new(pai(adc::adc(RegisterTarget::L))),
        0x8E => Box::new(pai(adc::adc_mem_at_hl)),
        0x8F => Box::new(pai(adc::adc(RegisterTarget::A))),
        0x90 => Box::new(pai(sub::sub_r_r_a(RegisterTarget::B))),
        0x91 => Box::new(pai(sub::sub_r_r_a(RegisterTarget::C))),
        0x92 => Box::new(pai(sub::sub_r_r_a(RegisterTarget::D))),
        0x93 => Box::new(pai(sub::sub_r_r_a(RegisterTarget::E))),
        0x94 => Box::new(pai(sub::sub_r_r_a(RegisterTarget::H))),
        0x95 => Box::new(pai(sub::sub_r_r_a(RegisterTarget::L))),
        //0x96
        0x97 => Box::new(pai(sub::sub_r_r_a(RegisterTarget::A))),
        0x98 => Box::new(pai(sbc::sbc_r_r_a(RegisterTarget::B))),
        0x99 => Box::new(pai(sbc::sbc_r_r_a(RegisterTarget::C))),
        0x9A => Box::new(pai(sbc::sbc_r_r_a(RegisterTarget::D))),
        0x9B => Box::new(pai(sbc::sbc_r_r_a(RegisterTarget::E))),
        0x9C => Box::new(pai(sbc::sbc_r_r_a(RegisterTarget::H))),
        0x9D => Box::new(pai(sbc::sbc_r_r_a(RegisterTarget::L))),
        // 0x9E => Box::new(sbc::sbc_r_r_a(RegisterTarget::A)),
        0x9F => Box::new(pai(sbc::sbc_r_r_a(RegisterTarget::A))),

        0xA0 => Box::new(pai(and::and(RegisterTarget::B))),
        0xA1 => Box::new(pai(and::and(RegisterTarget::C))),
        0xA2 => Box::new(pai(and::and(RegisterTarget::D))),
        0xA3 => Box::new(pai(and::and(RegisterTarget::E))),
        0xA4 => Box::new(pai(and::and(RegisterTarget::H))),
        0xA5 => Box::new(pai(and::and(RegisterTarget::L))),
        0xA6 => Box::new(pai(and::and_mem_at_r16)),
        0xA7 => Box::new(pai(and::and(RegisterTarget::A))),

        0xA8 => Box::new(pai(xor::xor(RegisterTarget::B))),
        0xA9 => Box::new(pai(xor::xor(RegisterTarget::C))),
        0xAA => Box::new(pai(xor::xor(RegisterTarget::D))),
        0xAB => Box::new(pai(xor::xor(RegisterTarget::E))),
        0xAC => Box::new(pai(xor::xor(RegisterTarget::H))),
        0xAD => Box::new(pai(xor::xor(RegisterTarget::L))),
        0xAE => Box::new(pai(xor::xor_mem_at_r16(Register16bTarget::HL))),
        0xAF => Box::new(pai(xor::xor(RegisterTarget::A))),

        0xB0 => Box::new(pai(or::or(RegisterTarget::B))),
        0xB1 => Box::new(pai(or::or(RegisterTarget::C))),
        0xB2 => Box::new(pai(or::or(RegisterTarget::D))),
        0xB3 => Box::new(pai(or::or(RegisterTarget::E))),
        0xB4 => Box::new(pai(or::or(RegisterTarget::H))),
        0xB5 => Box::new(pai(or::or(RegisterTarget::L))),
        0xB6 => Box::new(pai(or::or_mem_at_r16(Register16bTarget::HL))),
        0xB7 => Box::new(pai(or::or(RegisterTarget::A))),

        0xB8 => Box::new(pai(cp::cp(RegisterTarget::B))),
        0xB9 => Box::new(pai(cp::cp(RegisterTarget::C))),
        0xBA => Box::new(pai(cp::cp(RegisterTarget::D))),
        0xBB => Box::new(pai(cp::cp(RegisterTarget::E))),
        0xBC => Box::new(pai(cp::cp(RegisterTarget::H))),
        0xBD => Box::new(pai(cp::cp(RegisterTarget::L))),
        0xBE => Box::new(pai(cp::cp_mem_at_r16(Register16bTarget::HL))),
        0xBF => Box::new(pai(cp::cp(RegisterTarget::A))),

        0xC1 => Box::new(pai(stack::pop(Register16bTarget::BC))),
        0xD1 => Box::new(pai(stack::pop(Register16bTarget::DE))),
        0xE1 => Box::new(pai(stack::pop(Register16bTarget::HL))),
        0xF1 => Box::new(pai(stack::pop(Register16bTarget::AF))),

        0xC2 => Box::new(jmp::jp_nz_a16),
        0xD2 => Box::new(jmp::jp_nc_a16),
        0xCA => Box::new(jmp::jp_z_a16),
        0xDA => Box::new(jmp::jp_c_a16),

        0xC4 => Box::new(call::call_nz_a16),
        0xD4 => Box::new(call::call_nc_a16),

        0xC0 => Box::new(call::ret_nz),
        0xD0 => Box::new(call::ret_nc),
        0xC8 => Box::new(call::ret_z),
        0xD8 => Box::new(call::ret_c),

        0xC5 => Box::new(pai(stack::push(Register16bTarget::BC))),
        0xD5 => Box::new(pai(stack::push(Register16bTarget::DE))),
        0xE5 => Box::new(pai(stack::push(Register16bTarget::HL))),
        0xF5 => Box::new(pai(stack::push(Register16bTarget::AF))),

        0xCD => Box::new(call::call_a16),
        0xC9 => Box::new(call::ret),

        0xE9 => Box::new(jmp::jp_hl),

        0xC6 => Box::new(pai(add::add_d8)),
        0xD6 => Box::new(pai(sub::sub_d8)),
        0xE6 => Box::new(pai(and::and_d8)),
        0xF6 => Box::new(pai(or::or_d8)),

        0xE0 => Box::new(pai(ld::ld_mem_at_d8_a)),
        0xF0 => Box::new(pai(ld::ld_a_mem_at_d8)),
        0xE2 => Box::new(pai(ld::ld_mem_at_c_a)),
        0xEA => Box::new(pai(ld::ld_mem_at_d16_r(RegisterTarget::A))),
        0xFA => Box::new(pai(ld::ld_r_mem_at_d16(RegisterTarget::A))),

        0xF3 => Box::new(pai(int::di)),
        0xFB => Box::new(pai(int::ei)),

        0xE8 => Box::new(pai(add::add_sp_n8)),
        0xF8 => Box::new(pai(ld::ld_hl_sp_n8)),
        0xF9 => Box::new(pai(ld::ld_sp_hl)),

        0xC7 => Box::new(rst::rst(0x00)),
        0xCF => Box::new(rst::rst(0x08)),
        0xD7 => Box::new(rst::rst(0x10)),
        0xDF => Box::new(rst::rst(0x18)),
        0xE7 => Box::new(rst::rst(0x20)),
        0xEF => Box::new(rst::rst(0x28)),
        0xF7 => Box::new(rst::rst(0x30)),
        0xFF => Box::new(rst::rst(0x38)),

        0xCE => Box::new(pai(adc::adc_n8)),
        0xDE => Box::new(pai(sbc::sbc_n8)),
        0xEE => Box::new(pai(xor::xor_d8)),
        0xFE => Box::new(pai(cp::cp_d8)),

        other => {
            panic!("Unsupported instruction {:02X}", other)
        }
    }
}

pub fn from_prefixed_byte(byte: u8) -> Box<Instruction> {
    match byte {
        0x30 => Box::new(dpai(swap::swap(RegisterTarget::B))),
        0x31 => Box::new(dpai(swap::swap(RegisterTarget::C))),
        0x32 => Box::new(dpai(swap::swap(RegisterTarget::D))),
        0x33 => Box::new(dpai(swap::swap(RegisterTarget::E))),
        0x34 => Box::new(dpai(swap::swap(RegisterTarget::H))),
        0x35 => Box::new(dpai(swap::swap(RegisterTarget::L))),
        // 0x36 => Box::new(swap::swap(RegisterTarget::C)),
        0x37 => Box::new(dpai(swap::swap(RegisterTarget::A))),

        0x10 => Box::new(dpai(rot::rl_r(RegisterTarget::B))),
        0x11 => Box::new(dpai(rot::rl_r(RegisterTarget::C))),
        0x12 => Box::new(dpai(rot::rl_r(RegisterTarget::D))),
        0x13 => Box::new(dpai(rot::rl_r(RegisterTarget::E))),
        0x14 => Box::new(dpai(rot::rl_r(RegisterTarget::H))),
        0x15 => Box::new(dpai(rot::rl_r(RegisterTarget::L))),
        0x16 => Box::new(dpai(rot::rl_mem_at_hl)),
        0x17 => Box::new(dpai(rot::rl_r(RegisterTarget::A))),
        0x18 => Box::new(dpai(rot::rr_r(RegisterTarget::B))),
        0x19 => Box::new(dpai(rot::rr_r(RegisterTarget::C))),
        0x1A => Box::new(dpai(rot::rr_r(RegisterTarget::D))),
        0x1B => Box::new(dpai(rot::rr_r(RegisterTarget::E))),
        0x1C => Box::new(dpai(rot::rr_r(RegisterTarget::H))),
        0x1D => Box::new(dpai(rot::rr_r(RegisterTarget::L))),
        0x1E => Box::new(dpai(rot::rr_mem_at_hl)),
        0x1F => Box::new(dpai(rot::rr_r(RegisterTarget::A))),

        0x20 => Box::new(dpai(shift::sla(RegisterTarget::B))),
        0x21 => Box::new(dpai(shift::sla(RegisterTarget::C))),
        0x22 => Box::new(dpai(shift::sla(RegisterTarget::D))),
        0x23 => Box::new(dpai(shift::sla(RegisterTarget::E))),
        0x24 => Box::new(dpai(shift::sla(RegisterTarget::H))),
        0x25 => Box::new(dpai(shift::sla(RegisterTarget::L))),
        0x26 => Box::new(dpai(shift::sla_mem_at_hl)),
        0x27 => Box::new(dpai(shift::sla(RegisterTarget::A))),

        0x28 => Box::new(dpai(shift::sra(RegisterTarget::B))),
        0x29 => Box::new(dpai(shift::sra(RegisterTarget::C))),
        0x2a => Box::new(dpai(shift::sra(RegisterTarget::D))),
        0x2b => Box::new(dpai(shift::sra(RegisterTarget::E))),
        0x2c => Box::new(dpai(shift::sra(RegisterTarget::H))),
        0x2d => Box::new(dpai(shift::sra(RegisterTarget::L))),
        0x2e => Box::new(dpai(shift::sra_mem_at_hl)),
        0x2f => Box::new(dpai(shift::sra(RegisterTarget::A))),

        0x38 => Box::new(dpai(shift::srl(RegisterTarget::B))),
        0x39 => Box::new(dpai(shift::srl(RegisterTarget::C))),
        0x3a => Box::new(dpai(shift::srl(RegisterTarget::D))),
        0x3b => Box::new(dpai(shift::srl(RegisterTarget::E))),
        0x3c => Box::new(dpai(shift::srl(RegisterTarget::H))),
        0x3d => Box::new(dpai(shift::srl(RegisterTarget::L))),
        0x3e => Box::new(dpai(shift::srl_mem_at_hl)),
        0x3f => Box::new(dpai(shift::srl(RegisterTarget::A))),

        0x40 => Box::new(dpai(bit::bit_r(RegisterTarget::B, 0))),
        0x41 => Box::new(dpai(bit::bit_r(RegisterTarget::C, 0))),
        0x42 => Box::new(dpai(bit::bit_r(RegisterTarget::D, 0))),
        0x43 => Box::new(dpai(bit::bit_r(RegisterTarget::E, 0))),
        0x44 => Box::new(dpai(bit::bit_r(RegisterTarget::H, 0))),
        0x45 => Box::new(dpai(bit::bit_r(RegisterTarget::L, 0))),
        0x46 => Box::new(dpai(bit::bit_mem_at_hl(0))),
        0x47 => Box::new(dpai(bit::bit_r(RegisterTarget::A, 0))),
        0x48 => Box::new(dpai(bit::bit_r(RegisterTarget::B, 1))),
        0x49 => Box::new(dpai(bit::bit_r(RegisterTarget::C, 1))),
        0x4A => Box::new(dpai(bit::bit_r(RegisterTarget::D, 1))),
        0x4B => Box::new(dpai(bit::bit_r(RegisterTarget::E, 1))),
        0x4C => Box::new(dpai(bit::bit_r(RegisterTarget::H, 1))),
        0x4D => Box::new(dpai(bit::bit_r(RegisterTarget::L, 1))),
        0x4E => Box::new(dpai(bit::bit_mem_at_hl(1))),
        0x4F => Box::new(dpai(bit::bit_r(RegisterTarget::A, 1))),
        0x50 => Box::new(dpai(bit::bit_r(RegisterTarget::B, 2))),
        0x51 => Box::new(dpai(bit::bit_r(RegisterTarget::C, 2))),
        0x52 => Box::new(dpai(bit::bit_r(RegisterTarget::D, 2))),
        0x53 => Box::new(dpai(bit::bit_r(RegisterTarget::E, 2))),
        0x54 => Box::new(dpai(bit::bit_r(RegisterTarget::H, 2))),
        0x55 => Box::new(dpai(bit::bit_r(RegisterTarget::L, 2))),
        0x56 => Box::new(dpai(bit::bit_mem_at_hl(2))),
        0x57 => Box::new(dpai(bit::bit_r(RegisterTarget::A, 2))),
        0x58 => Box::new(dpai(bit::bit_r(RegisterTarget::B, 3))),
        0x59 => Box::new(dpai(bit::bit_r(RegisterTarget::C, 3))),
        0x5A => Box::new(dpai(bit::bit_r(RegisterTarget::D, 3))),
        0x5B => Box::new(dpai(bit::bit_r(RegisterTarget::E, 3))),
        0x5C => Box::new(dpai(bit::bit_r(RegisterTarget::H, 3))),
        0x5D => Box::new(dpai(bit::bit_r(RegisterTarget::L, 3))),
        0x5E => Box::new(dpai(bit::bit_mem_at_hl(3))),
        0x5F => Box::new(dpai(bit::bit_r(RegisterTarget::A, 3))),
        0x60 => Box::new(dpai(bit::bit_r(RegisterTarget::B, 4))),
        0x61 => Box::new(dpai(bit::bit_r(RegisterTarget::C, 4))),
        0x62 => Box::new(dpai(bit::bit_r(RegisterTarget::D, 4))),
        0x63 => Box::new(dpai(bit::bit_r(RegisterTarget::E, 4))),
        0x64 => Box::new(dpai(bit::bit_r(RegisterTarget::H, 4))),
        0x65 => Box::new(dpai(bit::bit_r(RegisterTarget::L, 4))),
        0x66 => Box::new(dpai(bit::bit_mem_at_hl(4))),
        0x67 => Box::new(dpai(bit::bit_r(RegisterTarget::A, 4))),
        0x68 => Box::new(dpai(bit::bit_r(RegisterTarget::B, 5))),
        0x69 => Box::new(dpai(bit::bit_r(RegisterTarget::C, 5))),
        0x6A => Box::new(dpai(bit::bit_r(RegisterTarget::D, 5))),
        0x6B => Box::new(dpai(bit::bit_r(RegisterTarget::E, 5))),
        0x6C => Box::new(dpai(bit::bit_r(RegisterTarget::H, 5))),
        0x6D => Box::new(dpai(bit::bit_r(RegisterTarget::L, 5))),
        0x6E => Box::new(dpai(bit::bit_mem_at_hl(5))),
        0x6F => Box::new(dpai(bit::bit_r(RegisterTarget::A, 5))),
        0x70 => Box::new(dpai(bit::bit_r(RegisterTarget::B, 6))),
        0x71 => Box::new(dpai(bit::bit_r(RegisterTarget::C, 6))),
        0x72 => Box::new(dpai(bit::bit_r(RegisterTarget::D, 6))),
        0x73 => Box::new(dpai(bit::bit_r(RegisterTarget::E, 6))),
        0x74 => Box::new(dpai(bit::bit_r(RegisterTarget::H, 6))),
        0x75 => Box::new(dpai(bit::bit_r(RegisterTarget::L, 6))),
        0x76 => Box::new(dpai(bit::bit_mem_at_hl(6))),
        0x77 => Box::new(dpai(bit::bit_r(RegisterTarget::A, 6))),
        0x78 => Box::new(dpai(bit::bit_r(RegisterTarget::B, 7))),
        0x79 => Box::new(dpai(bit::bit_r(RegisterTarget::C, 7))),
        0x7A => Box::new(dpai(bit::bit_r(RegisterTarget::D, 7))),
        0x7B => Box::new(dpai(bit::bit_r(RegisterTarget::E, 7))),
        0x7C => Box::new(dpai(bit::bit_r(RegisterTarget::H, 7))),
        0x7D => Box::new(dpai(bit::bit_r(RegisterTarget::L, 7))),
        0x7E => Box::new(dpai(bit::bit_mem_at_hl(7))),
        0x7F => Box::new(dpai(bit::bit_r(RegisterTarget::A, 7))),

        0xC0 => Box::new(dpai(bit::set_r(RegisterTarget::B, 0))),
        0xC1 => Box::new(dpai(bit::set_r(RegisterTarget::C, 0))),
        0xC2 => Box::new(dpai(bit::set_r(RegisterTarget::D, 0))),
        0xC3 => Box::new(dpai(bit::set_r(RegisterTarget::E, 0))),
        0xC4 => Box::new(dpai(bit::set_r(RegisterTarget::H, 0))),
        0xC5 => Box::new(dpai(bit::set_r(RegisterTarget::L, 0))),
        0xC6 => Box::new(dpai(bit::set_mem_at_hl(0))),
        0xC7 => Box::new(dpai(bit::set_r(RegisterTarget::A, 0))),
        0xC8 => Box::new(dpai(bit::set_r(RegisterTarget::B, 1))),
        0xC9 => Box::new(dpai(bit::set_r(RegisterTarget::C, 1))),
        0xCA => Box::new(dpai(bit::set_r(RegisterTarget::D, 1))),
        0xCB => Box::new(dpai(bit::set_r(RegisterTarget::E, 1))),
        0xCC => Box::new(dpai(bit::set_r(RegisterTarget::H, 1))),
        0xCD => Box::new(dpai(bit::set_r(RegisterTarget::L, 1))),
        0xCE => Box::new(dpai(bit::set_mem_at_hl(1))),
        0xCF => Box::new(dpai(bit::set_r(RegisterTarget::A, 1))),
        0xD0 => Box::new(dpai(bit::set_r(RegisterTarget::B, 2))),
        0xD1 => Box::new(dpai(bit::set_r(RegisterTarget::C, 2))),
        0xD2 => Box::new(dpai(bit::set_r(RegisterTarget::D, 2))),
        0xD3 => Box::new(dpai(bit::set_r(RegisterTarget::E, 2))),
        0xD4 => Box::new(dpai(bit::set_r(RegisterTarget::H, 2))),
        0xD5 => Box::new(dpai(bit::set_r(RegisterTarget::L, 2))),
        0xD6 => Box::new(dpai(bit::set_mem_at_hl(2))),
        0xD7 => Box::new(dpai(bit::set_r(RegisterTarget::A, 2))),
        0xD8 => Box::new(dpai(bit::set_r(RegisterTarget::B, 3))),
        0xD9 => Box::new(dpai(bit::set_r(RegisterTarget::C, 3))),
        0xDA => Box::new(dpai(bit::set_r(RegisterTarget::D, 3))),
        0xDB => Box::new(dpai(bit::set_r(RegisterTarget::E, 3))),
        0xDC => Box::new(dpai(bit::set_r(RegisterTarget::H, 3))),
        0xDD => Box::new(dpai(bit::set_r(RegisterTarget::L, 3))),
        0xDE => Box::new(dpai(bit::set_mem_at_hl(3))),
        0xDF => Box::new(dpai(bit::set_r(RegisterTarget::A, 3))),
        0xE0 => Box::new(dpai(bit::set_r(RegisterTarget::B, 4))),
        0xE1 => Box::new(dpai(bit::set_r(RegisterTarget::C, 4))),
        0xE2 => Box::new(dpai(bit::set_r(RegisterTarget::D, 4))),
        0xE3 => Box::new(dpai(bit::set_r(RegisterTarget::E, 4))),
        0xE4 => Box::new(dpai(bit::set_r(RegisterTarget::H, 4))),
        0xE5 => Box::new(dpai(bit::set_r(RegisterTarget::L, 4))),
        0xE6 => Box::new(dpai(bit::set_mem_at_hl(4))),
        0xE7 => Box::new(dpai(bit::set_r(RegisterTarget::A, 4))),
        0xE8 => Box::new(dpai(bit::set_r(RegisterTarget::B, 5))),
        0xE9 => Box::new(dpai(bit::set_r(RegisterTarget::C, 5))),
        0xEA => Box::new(dpai(bit::set_r(RegisterTarget::D, 5))),
        0xEB => Box::new(dpai(bit::set_r(RegisterTarget::E, 5))),
        0xEC => Box::new(dpai(bit::set_r(RegisterTarget::H, 5))),
        0xED => Box::new(dpai(bit::set_r(RegisterTarget::L, 5))),
        0xEE => Box::new(dpai(bit::set_mem_at_hl(5))),
        0xEF => Box::new(dpai(bit::set_r(RegisterTarget::A, 5))),
        0xF0 => Box::new(dpai(bit::set_r(RegisterTarget::B, 6))),
        0xF1 => Box::new(dpai(bit::set_r(RegisterTarget::C, 6))),
        0xF2 => Box::new(dpai(bit::set_r(RegisterTarget::D, 6))),
        0xF3 => Box::new(dpai(bit::set_r(RegisterTarget::E, 6))),
        0xF4 => Box::new(dpai(bit::set_r(RegisterTarget::H, 6))),
        0xF5 => Box::new(dpai(bit::set_r(RegisterTarget::L, 6))),
        0xF6 => Box::new(dpai(bit::set_mem_at_hl(6))),
        0xF7 => Box::new(dpai(bit::set_r(RegisterTarget::A, 6))),
        0xF8 => Box::new(dpai(bit::set_r(RegisterTarget::B, 7))),
        0xF9 => Box::new(dpai(bit::set_r(RegisterTarget::C, 7))),
        0xFA => Box::new(dpai(bit::set_r(RegisterTarget::D, 7))),
        0xFB => Box::new(dpai(bit::set_r(RegisterTarget::E, 7))),
        0xFC => Box::new(dpai(bit::set_r(RegisterTarget::H, 7))),
        0xFD => Box::new(dpai(bit::set_r(RegisterTarget::L, 7))),
        0xFE => Box::new(dpai(bit::set_mem_at_hl(7))),
        0xFF => Box::new(dpai(bit::set_r(RegisterTarget::A, 7))),
        other => {
            panic!("Unsupported prefixed instruction {:02X}", other)
        }
    }
}
