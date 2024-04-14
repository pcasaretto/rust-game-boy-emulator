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

fn pc_advancing_instruction(
    instruction: impl Fn(&mut gameboy::Gameboy) -> u8,
) -> impl Fn(&mut gameboy::Gameboy) -> u8 {
    move |gameboy: &mut gameboy::Gameboy| {
        let ticks = instruction(gameboy);
        gameboy.cpu.registers.pc += 1;
        ticks
    }
}

pub fn from_byte(byte: u8) -> Box<Instruction> {
    match byte {
        0x00 => Box::new(pc_advancing_instruction(nop::nop)),
        0x01 => Box::new(pc_advancing_instruction(ld::ld_r16_n16(
            Register16bTarget::BC,
        ))),
        0x02 => Box::new(pc_advancing_instruction(ld::ld_mem_at_r16_r(
            Register16bTarget::BC,
            RegisterTarget::A,
        ))),
        0x10 => Box::new(pc_advancing_instruction(misc::stop)),
        0x12 => Box::new(pc_advancing_instruction(ld::ld_mem_at_r16_r(
            Register16bTarget::DE,
            RegisterTarget::A,
        ))),
        0x11 => Box::new(pc_advancing_instruction(ld::ld_r16_n16(
            Register16bTarget::DE,
        ))),

        0x0A => Box::new(pc_advancing_instruction(ld::ld_r_mem_at_r16(
            Register16bTarget::BC,
            RegisterTarget::A,
        ))),
        0x1A => Box::new(pc_advancing_instruction(ld::ld_r_mem_at_r16(
            Register16bTarget::DE,
            RegisterTarget::A,
        ))),

        0x17 => Box::new(pc_advancing_instruction(rot::rl_a)),

        0x03 => Box::new(pc_advancing_instruction(inc::inc_r16(
            Register16bTarget::BC,
        ))),
        0x04 => Box::new(pc_advancing_instruction(inc::inc_r(RegisterTarget::B))),
        0x13 => Box::new(pc_advancing_instruction(inc::inc_r16(
            Register16bTarget::DE,
        ))),
        0x14 => Box::new(pc_advancing_instruction(inc::inc_r(RegisterTarget::D))),
        0x23 => Box::new(pc_advancing_instruction(inc::inc_r16(
            Register16bTarget::HL,
        ))),
        0x24 => Box::new(pc_advancing_instruction(inc::inc_r(RegisterTarget::H))),
        0x33 => Box::new(pc_advancing_instruction(inc::inc_r16(
            Register16bTarget::SP,
        ))),
        0x0C => Box::new(pc_advancing_instruction(inc::inc_r(RegisterTarget::C))),
        0x1C => Box::new(pc_advancing_instruction(inc::inc_r(RegisterTarget::E))),
        0x2C => Box::new(pc_advancing_instruction(inc::inc_r(RegisterTarget::L))),
        0x3C => Box::new(pc_advancing_instruction(inc::inc_r(RegisterTarget::A))),

        0x05 => Box::new(pc_advancing_instruction(dec::dec_r(RegisterTarget::B))),
        0x15 => Box::new(pc_advancing_instruction(dec::dec_r(RegisterTarget::D))),
        0x25 => Box::new(pc_advancing_instruction(dec::dec_r(RegisterTarget::H))),
        0x0D => Box::new(pc_advancing_instruction(dec::dec_r(RegisterTarget::C))),
        0x1D => Box::new(pc_advancing_instruction(dec::dec_r(RegisterTarget::E))),
        0x2D => Box::new(pc_advancing_instruction(dec::dec_r(RegisterTarget::L))),
        0x3D => Box::new(pc_advancing_instruction(dec::dec_r(RegisterTarget::A))),

        0x18 => Box::new(jmp::jr),
        0x20 => Box::new(jmp::jr_nz),
        0x30 => Box::new(jmp::jr_nc),
        0x28 => Box::new(jmp::jr_z),
        0x38 => Box::new(jmp::jr_c),

        0x31 => Box::new(pc_advancing_instruction(ld::ld_r16_n16(
            Register16bTarget::SP,
        ))),
        0x21 => Box::new(pc_advancing_instruction(ld::ld_r16_n16(
            Register16bTarget::HL,
        ))),
        0x22 => Box::new(pc_advancing_instruction(ld::ld_mem_at_hl_a_inc)),
        0x32 => Box::new(pc_advancing_instruction(ld::ld_mem_at_hl_a_dec)),
        0x2A => Box::new(pc_advancing_instruction(ld::ld_a_mem_at_hl_inc)),
        0x3A => Box::new(pc_advancing_instruction(ld::ld_a_mem_at_hl_dec)),
        // 0x33 => Box::new(inc::inc_sp()),
        0x06 => Box::new(pc_advancing_instruction(ld::ld_d8_r(RegisterTarget::B))),
        0x16 => Box::new(pc_advancing_instruction(ld::ld_d8_r(RegisterTarget::D))),
        0x26 => Box::new(pc_advancing_instruction(ld::ld_d8_r(RegisterTarget::H))),
        0x36 => Box::new(pc_advancing_instruction(ld::ld_d8_mem_at_r16(
            Register16bTarget::HL,
        ))),
        0x0E => Box::new(pc_advancing_instruction(ld::ld_d8_r(RegisterTarget::C))),
        0x1E => Box::new(pc_advancing_instruction(ld::ld_d8_r(RegisterTarget::E))),
        0x2E => Box::new(pc_advancing_instruction(ld::ld_d8_r(RegisterTarget::L))),
        0x3E => Box::new(pc_advancing_instruction(ld::ld_d8_r(RegisterTarget::A))),
        0xC3 => Box::new(jmp::jmp_a16),
        0x40 => Box::new(pc_advancing_instruction(ld::ld_r_r(
            RegisterTarget::B,
            RegisterTarget::B,
        ))),
        0x41 => Box::new(pc_advancing_instruction(ld::ld_r_r(
            RegisterTarget::B,
            RegisterTarget::C,
        ))),
        0x42 => Box::new(pc_advancing_instruction(ld::ld_r_r(
            RegisterTarget::B,
            RegisterTarget::D,
        ))),
        0x43 => Box::new(pc_advancing_instruction(ld::ld_r_r(
            RegisterTarget::B,
            RegisterTarget::E,
        ))),
        0x44 => Box::new(pc_advancing_instruction(ld::ld_r_r(
            RegisterTarget::B,
            RegisterTarget::H,
        ))),
        0x45 => Box::new(pc_advancing_instruction(ld::ld_r_r(
            RegisterTarget::B,
            RegisterTarget::L,
        ))),
        // 0x46 => Box::new(ld::ld_r_r(RegisterTarget::B, RegisterTarget::A)),
        0x47 => Box::new(pc_advancing_instruction(ld::ld_r_r(
            RegisterTarget::B,
            RegisterTarget::A,
        ))),
        0x48 => Box::new(pc_advancing_instruction(ld::ld_r_r(
            RegisterTarget::C,
            RegisterTarget::B,
        ))),
        0x49 => Box::new(pc_advancing_instruction(ld::ld_r_r(
            RegisterTarget::C,
            RegisterTarget::C,
        ))),
        0x4a => Box::new(pc_advancing_instruction(ld::ld_r_r(
            RegisterTarget::C,
            RegisterTarget::D,
        ))),
        0x4b => Box::new(pc_advancing_instruction(ld::ld_r_r(
            RegisterTarget::C,
            RegisterTarget::E,
        ))),
        0x4c => Box::new(pc_advancing_instruction(ld::ld_r_r(
            RegisterTarget::C,
            RegisterTarget::H,
        ))),
        0x4d => Box::new(pc_advancing_instruction(ld::ld_r_r(
            RegisterTarget::C,
            RegisterTarget::L,
        ))),
        // 0x4e => Box::new(ld::ld_r_r(RegisterTarget::C, RegisterTarget::A)),
        0x4f => Box::new(pc_advancing_instruction(ld::ld_r_r(
            RegisterTarget::C,
            RegisterTarget::A,
        ))),
        0x50 => Box::new(pc_advancing_instruction(ld::ld_r_r(
            RegisterTarget::D,
            RegisterTarget::B,
        ))),
        0x51 => Box::new(pc_advancing_instruction(ld::ld_r_r(
            RegisterTarget::D,
            RegisterTarget::C,
        ))),
        0x52 => Box::new(pc_advancing_instruction(ld::ld_r_r(
            RegisterTarget::D,
            RegisterTarget::D,
        ))),
        0x53 => Box::new(pc_advancing_instruction(ld::ld_r_r(
            RegisterTarget::D,
            RegisterTarget::E,
        ))),
        0x54 => Box::new(pc_advancing_instruction(ld::ld_r_r(
            RegisterTarget::D,
            RegisterTarget::H,
        ))),
        0x55 => Box::new(pc_advancing_instruction(ld::ld_r_r(
            RegisterTarget::D,
            RegisterTarget::L,
        ))),
        // 0x56 => Box::new(ld::ld_r_r(RegisterTarget::D, Register16bTarget::HL)),
        0x57 => Box::new(pc_advancing_instruction(ld::ld_r_r(
            RegisterTarget::D,
            RegisterTarget::A,
        ))),
        0x58 => Box::new(pc_advancing_instruction(ld::ld_r_r(
            RegisterTarget::E,
            RegisterTarget::B,
        ))),
        0x59 => Box::new(pc_advancing_instruction(ld::ld_r_r(
            RegisterTarget::E,
            RegisterTarget::C,
        ))),
        0x5A => Box::new(pc_advancing_instruction(ld::ld_r_r(
            RegisterTarget::E,
            RegisterTarget::D,
        ))),
        0x5B => Box::new(pc_advancing_instruction(ld::ld_r_r(
            RegisterTarget::E,
            RegisterTarget::E,
        ))),
        0x5C => Box::new(pc_advancing_instruction(ld::ld_r_r(
            RegisterTarget::E,
            RegisterTarget::H,
        ))),
        0x5D => Box::new(pc_advancing_instruction(ld::ld_r_r(
            RegisterTarget::E,
            RegisterTarget::L,
        ))),
        // 0x5E => Box::new(ld::ld_r_r(RegisterTarget::E, Register16bTarget::HL)),
        0x5F => Box::new(pc_advancing_instruction(ld::ld_r_r(
            RegisterTarget::E,
            RegisterTarget::A,
        ))),
        0x60 => Box::new(pc_advancing_instruction(ld::ld_r_r(
            RegisterTarget::H,
            RegisterTarget::B,
        ))),
        0x61 => Box::new(pc_advancing_instruction(ld::ld_r_r(
            RegisterTarget::H,
            RegisterTarget::C,
        ))),
        0x62 => Box::new(pc_advancing_instruction(ld::ld_r_r(
            RegisterTarget::H,
            RegisterTarget::D,
        ))),
        0x63 => Box::new(pc_advancing_instruction(ld::ld_r_r(
            RegisterTarget::H,
            RegisterTarget::E,
        ))),
        0x64 => Box::new(pc_advancing_instruction(ld::ld_r_r(
            RegisterTarget::H,
            RegisterTarget::H,
        ))),
        0x65 => Box::new(pc_advancing_instruction(ld::ld_r_r(
            RegisterTarget::H,
            RegisterTarget::L,
        ))),
        // 0x66 => Box::new(ld::ld_r_r(RegisterTarget::H, RegisterTarget::A)),
        0x67 => Box::new(pc_advancing_instruction(ld::ld_r_r(
            RegisterTarget::H,
            RegisterTarget::A,
        ))),
        0x68 => Box::new(pc_advancing_instruction(ld::ld_r_r(
            RegisterTarget::L,
            RegisterTarget::B,
        ))),
        0x69 => Box::new(pc_advancing_instruction(ld::ld_r_r(
            RegisterTarget::L,
            RegisterTarget::C,
        ))),
        0x6A => Box::new(pc_advancing_instruction(ld::ld_r_r(
            RegisterTarget::L,
            RegisterTarget::D,
        ))),
        0x6B => Box::new(pc_advancing_instruction(ld::ld_r_r(
            RegisterTarget::L,
            RegisterTarget::E,
        ))),
        0x6C => Box::new(pc_advancing_instruction(ld::ld_r_r(
            RegisterTarget::L,
            RegisterTarget::H,
        ))),
        0x6D => Box::new(pc_advancing_instruction(ld::ld_r_r(
            RegisterTarget::L,
            RegisterTarget::L,
        ))),
        // 0x6E => Box::new(ld::ld_r_r(RegisterTarget::L, Register16bTarget::HL)),
        0x6F => Box::new(pc_advancing_instruction(ld::ld_r_r(
            RegisterTarget::L,
            RegisterTarget::A,
        ))),
        // 0x70 => Box::new(ld::ld_r_r(RegisterTarget::H, RegisterTarget::B)),
        // 0x71 => Box::new(ld::ld_r_r(RegisterTarget::H, RegisterTarget::C)),
        // 0x72 => Box::new(ld::ld_r_r(RegisterTarget::H, RegisterTarget::D)),
        // 0x73 => Box::new(ld::ld_r_r(RegisterTarget::H, RegisterTarget::E)),
        // 0x74 => Box::new(ld::ld_r_r(RegisterTarget::H, RegisterTarget::H)),
        // 0x75 => Box::new(ld::ld_r_r(RegisterTarget::H, RegisterTarget::L)),
        // 0x76 => Box::new(ld::ld_r_r(RegisterTarget::H, RegisterTarget::A)),
        0x70 => Box::new(pc_advancing_instruction(ld::ld_r_mem_at_hl(
            RegisterTarget::B,
        ))),
        0x71 => Box::new(pc_advancing_instruction(ld::ld_r_mem_at_hl(
            RegisterTarget::C,
        ))),
        0x72 => Box::new(pc_advancing_instruction(ld::ld_r_mem_at_hl(
            RegisterTarget::D,
        ))),
        0x73 => Box::new(pc_advancing_instruction(ld::ld_r_mem_at_hl(
            RegisterTarget::E,
        ))),
        0x74 => Box::new(pc_advancing_instruction(ld::ld_r_mem_at_hl(
            RegisterTarget::H,
        ))),
        0x75 => Box::new(pc_advancing_instruction(ld::ld_r_mem_at_hl(
            RegisterTarget::L,
        ))),
        0x77 => Box::new(pc_advancing_instruction(ld::ld_r_mem_at_hl(
            RegisterTarget::A,
        ))),
        0x78 => Box::new(pc_advancing_instruction(ld::ld_r_r(
            RegisterTarget::B,
            RegisterTarget::A,
        ))),
        0x79 => Box::new(pc_advancing_instruction(ld::ld_r_r(
            RegisterTarget::C,
            RegisterTarget::A,
        ))),
        0x7A => Box::new(pc_advancing_instruction(ld::ld_r_r(
            RegisterTarget::D,
            RegisterTarget::A,
        ))),
        0x7B => Box::new(pc_advancing_instruction(ld::ld_r_r(
            RegisterTarget::D,
            RegisterTarget::A,
        ))),
        0x7C => Box::new(pc_advancing_instruction(ld::ld_r_r(
            RegisterTarget::H,
            RegisterTarget::A,
        ))),
        0x7D => Box::new(pc_advancing_instruction(ld::ld_r_r(
            RegisterTarget::L,
            RegisterTarget::A,
        ))),
        // 0x7E => Box::new(ld::ld_r_r(RegisterTarget::A, Register16bTarget::HL)),
        0x7F => Box::new(pc_advancing_instruction(ld::ld_r_r(
            RegisterTarget::A,
            RegisterTarget::A,
        ))),
        0x87 => Box::new(pc_advancing_instruction(add::add(RegisterTarget::A))),
        0x80 => Box::new(pc_advancing_instruction(add::add(RegisterTarget::B))),
        0x81 => Box::new(pc_advancing_instruction(add::add(RegisterTarget::C))),
        0x82 => Box::new(pc_advancing_instruction(add::add(RegisterTarget::D))),
        0x83 => Box::new(pc_advancing_instruction(add::add(RegisterTarget::E))),
        0x84 => Box::new(pc_advancing_instruction(add::add(RegisterTarget::H))),
        0x85 => Box::new(pc_advancing_instruction(add::add(RegisterTarget::L))),
        0x8E => Box::new(pc_advancing_instruction(adc::adc_mem_at_hl())),
        0x8F => Box::new(pc_advancing_instruction(adc::adc(RegisterTarget::A))),
        0x88 => Box::new(pc_advancing_instruction(adc::adc(RegisterTarget::B))),
        0x89 => Box::new(pc_advancing_instruction(adc::adc(RegisterTarget::C))),
        0x8A => Box::new(pc_advancing_instruction(adc::adc(RegisterTarget::D))),
        0x8B => Box::new(pc_advancing_instruction(adc::adc(RegisterTarget::E))),
        0x8C => Box::new(pc_advancing_instruction(adc::adc(RegisterTarget::H))),
        0x8D => Box::new(pc_advancing_instruction(adc::adc(RegisterTarget::L))),
        0x90 => Box::new(pc_advancing_instruction(sub::sub_r_r_a(RegisterTarget::B))),
        0x91 => Box::new(pc_advancing_instruction(sub::sub_r_r_a(RegisterTarget::C))),
        0x92 => Box::new(pc_advancing_instruction(sub::sub_r_r_a(RegisterTarget::D))),
        0x93 => Box::new(pc_advancing_instruction(sub::sub_r_r_a(RegisterTarget::E))),
        0x94 => Box::new(pc_advancing_instruction(sub::sub_r_r_a(RegisterTarget::H))),
        0x95 => Box::new(pc_advancing_instruction(sub::sub_r_r_a(RegisterTarget::L))),
        0x97 => Box::new(pc_advancing_instruction(sub::sub_r_r_a(RegisterTarget::A))),
        0x98 => Box::new(pc_advancing_instruction(sbc::sbc_r_r_a(RegisterTarget::B))),
        0x99 => Box::new(pc_advancing_instruction(sbc::sbc_r_r_a(RegisterTarget::C))),
        0x9A => Box::new(pc_advancing_instruction(sbc::sbc_r_r_a(RegisterTarget::D))),
        0x9B => Box::new(pc_advancing_instruction(sbc::sbc_r_r_a(RegisterTarget::E))),
        0x9C => Box::new(pc_advancing_instruction(sbc::sbc_r_r_a(RegisterTarget::H))),
        0x9D => Box::new(pc_advancing_instruction(sbc::sbc_r_r_a(RegisterTarget::L))),
        // 0x9E => Box::new(sbc::sbc_r_r_a(RegisterTarget::A)),
        0x9F => Box::new(pc_advancing_instruction(sbc::sbc_r_r_a(RegisterTarget::A))),

        0xA0 => Box::new(pc_advancing_instruction(and::and(RegisterTarget::B))),
        0xA1 => Box::new(pc_advancing_instruction(and::and(RegisterTarget::C))),
        0xA2 => Box::new(pc_advancing_instruction(and::and(RegisterTarget::D))),
        0xA3 => Box::new(pc_advancing_instruction(and::and(RegisterTarget::E))),
        0xA4 => Box::new(pc_advancing_instruction(and::and(RegisterTarget::H))),
        0xA5 => Box::new(pc_advancing_instruction(and::and(RegisterTarget::L))),
        0xA6 => Box::new(pc_advancing_instruction(and::and_mem_at_r16(
            Register16bTarget::HL,
        ))),
        0xA7 => Box::new(pc_advancing_instruction(and::and(RegisterTarget::A))),

        0xA8 => Box::new(pc_advancing_instruction(xor::xor(RegisterTarget::B))),
        0xA9 => Box::new(pc_advancing_instruction(xor::xor(RegisterTarget::C))),
        0xAA => Box::new(pc_advancing_instruction(xor::xor(RegisterTarget::D))),
        0xAB => Box::new(pc_advancing_instruction(xor::xor(RegisterTarget::E))),
        0xAC => Box::new(pc_advancing_instruction(xor::xor(RegisterTarget::H))),
        0xAD => Box::new(pc_advancing_instruction(xor::xor(RegisterTarget::L))),
        0xAE => Box::new(pc_advancing_instruction(xor::xor_mem_at_r16(
            Register16bTarget::HL,
        ))),
        0xAF => Box::new(pc_advancing_instruction(xor::xor(RegisterTarget::A))),

        0xB0 => Box::new(pc_advancing_instruction(or::or(RegisterTarget::B))),
        0xB1 => Box::new(pc_advancing_instruction(or::or(RegisterTarget::C))),
        0xB2 => Box::new(pc_advancing_instruction(or::or(RegisterTarget::D))),
        0xB3 => Box::new(pc_advancing_instruction(or::or(RegisterTarget::E))),
        0xB4 => Box::new(pc_advancing_instruction(or::or(RegisterTarget::H))),
        0xB5 => Box::new(pc_advancing_instruction(or::or(RegisterTarget::L))),
        0xB6 => Box::new(pc_advancing_instruction(or::or_mem_at_r16(
            Register16bTarget::HL,
        ))),
        0xB7 => Box::new(pc_advancing_instruction(or::or(RegisterTarget::A))),

        0xC1 => Box::new(pc_advancing_instruction(stack::pop(Register16bTarget::BC))),
        0xD1 => Box::new(pc_advancing_instruction(stack::pop(Register16bTarget::DE))),
        0xE1 => Box::new(pc_advancing_instruction(stack::pop(Register16bTarget::HL))),
        0xF1 => Box::new(pc_advancing_instruction(stack::pop(Register16bTarget::AF))),

        0xC5 => Box::new(pc_advancing_instruction(stack::push(Register16bTarget::BC))),
        0xD5 => Box::new(pc_advancing_instruction(stack::push(Register16bTarget::DE))),
        0xE5 => Box::new(pc_advancing_instruction(stack::push(Register16bTarget::HL))),
        0xF5 => Box::new(pc_advancing_instruction(stack::push(Register16bTarget::AF))),

        0xCD => Box::new(call::call_a16),
        0xC9 => Box::new(call::ret),

        0xD6 => Box::new(pc_advancing_instruction(sub::sub_d8())),

        0xE0 => Box::new(pc_advancing_instruction(ld::ld_a_mem_at_d8())),
        0xE2 => Box::new(pc_advancing_instruction(ld::ld_mem_at_c_a)),
        0xEA => Box::new(pc_advancing_instruction(ld::ld_mem_at_d16_r(
            RegisterTarget::A,
        ))),

        0xF3 => Box::new(pc_advancing_instruction(int::di)),
        0xF9 => Box::new(pc_advancing_instruction(ld::ld_sp_hl)),
        0xFB => Box::new(pc_advancing_instruction(int::ei)),

        0xC7 => Box::new(rst::rst(0x00)),
        0xCF => Box::new(rst::rst(0x08)),
        0xD7 => Box::new(rst::rst(0x10)),
        0xDF => Box::new(rst::rst(0x18)),
        0xE7 => Box::new(rst::rst(0x20)),
        0xEF => Box::new(rst::rst(0x08)),
        0xF7 => Box::new(rst::rst(0x30)),
        0xFF => Box::new(rst::rst(0x38)),
        0xFE => Box::new(pc_advancing_instruction(cp::cp_d8)),

        other => {
            panic!("Unsupported instruction {:X}", other)
        }
    }
}

pub fn from_prefixed_byte(byte: u8) -> Box<Instruction> {
    match byte {
        0x30 => Box::new(pc_advancing_instruction(swap::swap(RegisterTarget::B))),
        0x31 => Box::new(pc_advancing_instruction(swap::swap(RegisterTarget::C))),
        0x32 => Box::new(pc_advancing_instruction(swap::swap(RegisterTarget::D))),
        0x33 => Box::new(pc_advancing_instruction(swap::swap(RegisterTarget::E))),
        0x34 => Box::new(pc_advancing_instruction(swap::swap(RegisterTarget::H))),
        0x35 => Box::new(pc_advancing_instruction(swap::swap(RegisterTarget::L))),
        // 0x36 => Box::new(swap::swap(RegisterTarget::C)),
        0x37 => Box::new(pc_advancing_instruction(swap::swap(RegisterTarget::A))),

        0x10 => Box::new(pc_advancing_instruction(rot::rl_r(RegisterTarget::B))),
        0x11 => Box::new(pc_advancing_instruction(rot::rl_r(RegisterTarget::C))),
        0x12 => Box::new(pc_advancing_instruction(rot::rl_r(RegisterTarget::D))),
        0x13 => Box::new(pc_advancing_instruction(rot::rl_r(RegisterTarget::E))),
        0x14 => Box::new(pc_advancing_instruction(rot::rl_r(RegisterTarget::H))),
        0x15 => Box::new(pc_advancing_instruction(rot::rl_r(RegisterTarget::L))),
        0x16 => Box::new(pc_advancing_instruction(rot::rl_mem_at_hl)),
        0x17 => Box::new(pc_advancing_instruction(rot::rl_r(RegisterTarget::A))),
        0x18 => Box::new(pc_advancing_instruction(rot::rr_r(RegisterTarget::B))),
        0x19 => Box::new(pc_advancing_instruction(rot::rr_r(RegisterTarget::C))),
        0x1A => Box::new(pc_advancing_instruction(rot::rr_r(RegisterTarget::D))),
        0x1B => Box::new(pc_advancing_instruction(rot::rr_r(RegisterTarget::E))),
        0x1C => Box::new(pc_advancing_instruction(rot::rr_r(RegisterTarget::H))),
        0x1D => Box::new(pc_advancing_instruction(rot::rr_r(RegisterTarget::L))),
        0x1E => Box::new(pc_advancing_instruction(rot::rr_mem_at_hl)),
        0x1F => Box::new(pc_advancing_instruction(rot::rr_r(RegisterTarget::A))),

        0x40 => Box::new(pc_advancing_instruction(bit::bit_r(RegisterTarget::B, 0))),
        0x41 => Box::new(pc_advancing_instruction(bit::bit_r(RegisterTarget::C, 0))),
        0x42 => Box::new(pc_advancing_instruction(bit::bit_r(RegisterTarget::D, 0))),
        0x43 => Box::new(pc_advancing_instruction(bit::bit_r(RegisterTarget::E, 0))),
        0x44 => Box::new(pc_advancing_instruction(bit::bit_r(RegisterTarget::H, 0))),
        0x45 => Box::new(pc_advancing_instruction(bit::bit_r(RegisterTarget::L, 0))),
        0x46 => Box::new(pc_advancing_instruction(bit::bit_mem_at_hl(0))),
        0x47 => Box::new(pc_advancing_instruction(bit::bit_r(RegisterTarget::A, 0))),
        0x48 => Box::new(pc_advancing_instruction(bit::bit_r(RegisterTarget::B, 1))),
        0x49 => Box::new(pc_advancing_instruction(bit::bit_r(RegisterTarget::C, 1))),
        0x4A => Box::new(pc_advancing_instruction(bit::bit_r(RegisterTarget::D, 1))),
        0x4B => Box::new(pc_advancing_instruction(bit::bit_r(RegisterTarget::E, 1))),
        0x4C => Box::new(pc_advancing_instruction(bit::bit_r(RegisterTarget::H, 1))),
        0x4D => Box::new(pc_advancing_instruction(bit::bit_r(RegisterTarget::L, 1))),
        0x4E => Box::new(pc_advancing_instruction(bit::bit_mem_at_hl(1))),
        0x4F => Box::new(pc_advancing_instruction(bit::bit_r(RegisterTarget::A, 1))),
        0x50 => Box::new(pc_advancing_instruction(bit::bit_r(RegisterTarget::B, 2))),
        0x51 => Box::new(pc_advancing_instruction(bit::bit_r(RegisterTarget::C, 2))),
        0x52 => Box::new(pc_advancing_instruction(bit::bit_r(RegisterTarget::D, 2))),
        0x53 => Box::new(pc_advancing_instruction(bit::bit_r(RegisterTarget::E, 2))),
        0x54 => Box::new(pc_advancing_instruction(bit::bit_r(RegisterTarget::H, 2))),
        0x55 => Box::new(pc_advancing_instruction(bit::bit_r(RegisterTarget::L, 2))),
        0x56 => Box::new(pc_advancing_instruction(bit::bit_mem_at_hl(2))),
        0x57 => Box::new(pc_advancing_instruction(bit::bit_r(RegisterTarget::A, 2))),
        0x58 => Box::new(pc_advancing_instruction(bit::bit_r(RegisterTarget::B, 3))),
        0x59 => Box::new(pc_advancing_instruction(bit::bit_r(RegisterTarget::C, 3))),
        0x5A => Box::new(pc_advancing_instruction(bit::bit_r(RegisterTarget::D, 3))),
        0x5B => Box::new(pc_advancing_instruction(bit::bit_r(RegisterTarget::E, 3))),
        0x5C => Box::new(pc_advancing_instruction(bit::bit_r(RegisterTarget::H, 3))),
        0x5D => Box::new(pc_advancing_instruction(bit::bit_r(RegisterTarget::L, 3))),
        0x5E => Box::new(pc_advancing_instruction(bit::bit_mem_at_hl(3))),
        0x5F => Box::new(pc_advancing_instruction(bit::bit_r(RegisterTarget::A, 3))),
        0x60 => Box::new(pc_advancing_instruction(bit::bit_r(RegisterTarget::B, 4))),
        0x61 => Box::new(pc_advancing_instruction(bit::bit_r(RegisterTarget::C, 4))),
        0x62 => Box::new(pc_advancing_instruction(bit::bit_r(RegisterTarget::D, 4))),
        0x63 => Box::new(pc_advancing_instruction(bit::bit_r(RegisterTarget::E, 4))),
        0x64 => Box::new(pc_advancing_instruction(bit::bit_r(RegisterTarget::H, 4))),
        0x65 => Box::new(pc_advancing_instruction(bit::bit_r(RegisterTarget::L, 4))),
        0x66 => Box::new(pc_advancing_instruction(bit::bit_mem_at_hl(4))),
        0x67 => Box::new(pc_advancing_instruction(bit::bit_r(RegisterTarget::A, 4))),
        0x68 => Box::new(pc_advancing_instruction(bit::bit_r(RegisterTarget::B, 5))),
        0x69 => Box::new(pc_advancing_instruction(bit::bit_r(RegisterTarget::C, 5))),
        0x6A => Box::new(pc_advancing_instruction(bit::bit_r(RegisterTarget::D, 5))),
        0x6B => Box::new(pc_advancing_instruction(bit::bit_r(RegisterTarget::E, 5))),
        0x6C => Box::new(pc_advancing_instruction(bit::bit_r(RegisterTarget::H, 5))),
        0x6D => Box::new(pc_advancing_instruction(bit::bit_r(RegisterTarget::L, 5))),
        0x6E => Box::new(pc_advancing_instruction(bit::bit_mem_at_hl(5))),
        0x6F => Box::new(pc_advancing_instruction(bit::bit_r(RegisterTarget::A, 5))),
        0x70 => Box::new(pc_advancing_instruction(bit::bit_r(RegisterTarget::B, 6))),
        0x71 => Box::new(pc_advancing_instruction(bit::bit_r(RegisterTarget::C, 6))),
        0x72 => Box::new(pc_advancing_instruction(bit::bit_r(RegisterTarget::D, 6))),
        0x73 => Box::new(pc_advancing_instruction(bit::bit_r(RegisterTarget::E, 6))),
        0x74 => Box::new(pc_advancing_instruction(bit::bit_r(RegisterTarget::H, 6))),
        0x75 => Box::new(pc_advancing_instruction(bit::bit_r(RegisterTarget::L, 6))),
        0x76 => Box::new(pc_advancing_instruction(bit::bit_mem_at_hl(6))),
        0x77 => Box::new(pc_advancing_instruction(bit::bit_r(RegisterTarget::A, 6))),
        0x78 => Box::new(pc_advancing_instruction(bit::bit_r(RegisterTarget::B, 7))),
        0x79 => Box::new(pc_advancing_instruction(bit::bit_r(RegisterTarget::C, 7))),
        0x7A => Box::new(pc_advancing_instruction(bit::bit_r(RegisterTarget::D, 7))),
        0x7B => Box::new(pc_advancing_instruction(bit::bit_r(RegisterTarget::E, 7))),
        0x7C => Box::new(pc_advancing_instruction(bit::bit_r(RegisterTarget::H, 7))),
        0x7D => Box::new(pc_advancing_instruction(bit::bit_r(RegisterTarget::L, 7))),
        0x7E => Box::new(pc_advancing_instruction(bit::bit_mem_at_hl(7))),
        0x7F => Box::new(pc_advancing_instruction(bit::bit_r(RegisterTarget::A, 7))),

        0xC0 => Box::new(pc_advancing_instruction(bit::set_r(RegisterTarget::B, 0))),
        0xC1 => Box::new(pc_advancing_instruction(bit::set_r(RegisterTarget::C, 0))),
        0xC2 => Box::new(pc_advancing_instruction(bit::set_r(RegisterTarget::D, 0))),
        0xC3 => Box::new(pc_advancing_instruction(bit::set_r(RegisterTarget::E, 0))),
        0xC4 => Box::new(pc_advancing_instruction(bit::set_r(RegisterTarget::H, 0))),
        0xC5 => Box::new(pc_advancing_instruction(bit::set_r(RegisterTarget::L, 0))),
        0xC6 => Box::new(pc_advancing_instruction(bit::set_mem_at_hl(0))),
        0xC7 => Box::new(pc_advancing_instruction(bit::set_r(RegisterTarget::A, 0))),
        0xC8 => Box::new(pc_advancing_instruction(bit::set_r(RegisterTarget::B, 1))),
        0xC9 => Box::new(pc_advancing_instruction(bit::set_r(RegisterTarget::C, 1))),
        0xCA => Box::new(pc_advancing_instruction(bit::set_r(RegisterTarget::D, 1))),
        0xCB => Box::new(pc_advancing_instruction(bit::set_r(RegisterTarget::E, 1))),
        0xCC => Box::new(pc_advancing_instruction(bit::set_r(RegisterTarget::H, 1))),
        0xCD => Box::new(pc_advancing_instruction(bit::set_r(RegisterTarget::L, 1))),
        0xCE => Box::new(pc_advancing_instruction(bit::set_mem_at_hl(1))),
        0xCF => Box::new(pc_advancing_instruction(bit::set_r(RegisterTarget::A, 1))),
        0xD0 => Box::new(pc_advancing_instruction(bit::set_r(RegisterTarget::B, 2))),
        0xD1 => Box::new(pc_advancing_instruction(bit::set_r(RegisterTarget::C, 2))),
        0xD2 => Box::new(pc_advancing_instruction(bit::set_r(RegisterTarget::D, 2))),
        0xD3 => Box::new(pc_advancing_instruction(bit::set_r(RegisterTarget::E, 2))),
        0xD4 => Box::new(pc_advancing_instruction(bit::set_r(RegisterTarget::H, 2))),
        0xD5 => Box::new(pc_advancing_instruction(bit::set_r(RegisterTarget::L, 2))),
        0xD6 => Box::new(pc_advancing_instruction(bit::set_mem_at_hl(2))),
        0xD7 => Box::new(pc_advancing_instruction(bit::set_r(RegisterTarget::A, 2))),
        0xD8 => Box::new(pc_advancing_instruction(bit::set_r(RegisterTarget::B, 3))),
        0xD9 => Box::new(pc_advancing_instruction(bit::set_r(RegisterTarget::C, 3))),
        0xDA => Box::new(pc_advancing_instruction(bit::set_r(RegisterTarget::D, 3))),
        0xDB => Box::new(pc_advancing_instruction(bit::set_r(RegisterTarget::E, 3))),
        0xDC => Box::new(pc_advancing_instruction(bit::set_r(RegisterTarget::H, 3))),
        0xDD => Box::new(pc_advancing_instruction(bit::set_r(RegisterTarget::L, 3))),
        0xDE => Box::new(pc_advancing_instruction(bit::set_mem_at_hl(3))),
        0xDF => Box::new(pc_advancing_instruction(bit::set_r(RegisterTarget::A, 3))),
        0xE0 => Box::new(pc_advancing_instruction(bit::set_r(RegisterTarget::B, 4))),
        0xE1 => Box::new(pc_advancing_instruction(bit::set_r(RegisterTarget::C, 4))),
        0xE2 => Box::new(pc_advancing_instruction(bit::set_r(RegisterTarget::D, 4))),
        0xE3 => Box::new(pc_advancing_instruction(bit::set_r(RegisterTarget::E, 4))),
        0xE4 => Box::new(pc_advancing_instruction(bit::set_r(RegisterTarget::H, 4))),
        0xE5 => Box::new(pc_advancing_instruction(bit::set_r(RegisterTarget::L, 4))),
        0xE6 => Box::new(pc_advancing_instruction(bit::set_mem_at_hl(4))),
        0xE7 => Box::new(pc_advancing_instruction(bit::set_r(RegisterTarget::A, 4))),
        0xE8 => Box::new(pc_advancing_instruction(bit::set_r(RegisterTarget::B, 5))),
        0xE9 => Box::new(pc_advancing_instruction(bit::set_r(RegisterTarget::C, 5))),
        0xEA => Box::new(pc_advancing_instruction(bit::set_r(RegisterTarget::D, 5))),
        0xEB => Box::new(pc_advancing_instruction(bit::set_r(RegisterTarget::E, 5))),
        0xEC => Box::new(pc_advancing_instruction(bit::set_r(RegisterTarget::H, 5))),
        0xED => Box::new(pc_advancing_instruction(bit::set_r(RegisterTarget::L, 5))),
        0xEE => Box::new(pc_advancing_instruction(bit::set_mem_at_hl(5))),
        0xEF => Box::new(pc_advancing_instruction(bit::set_r(RegisterTarget::A, 5))),
        0xF0 => Box::new(pc_advancing_instruction(bit::set_r(RegisterTarget::B, 6))),
        0xF1 => Box::new(pc_advancing_instruction(bit::set_r(RegisterTarget::C, 6))),
        0xF2 => Box::new(pc_advancing_instruction(bit::set_r(RegisterTarget::D, 6))),
        0xF3 => Box::new(pc_advancing_instruction(bit::set_r(RegisterTarget::E, 6))),
        0xF4 => Box::new(pc_advancing_instruction(bit::set_r(RegisterTarget::H, 6))),
        0xF5 => Box::new(pc_advancing_instruction(bit::set_r(RegisterTarget::L, 6))),
        0xF6 => Box::new(pc_advancing_instruction(bit::set_mem_at_hl(6))),
        0xF7 => Box::new(pc_advancing_instruction(bit::set_r(RegisterTarget::A, 6))),
        0xF8 => Box::new(pc_advancing_instruction(bit::set_r(RegisterTarget::B, 7))),
        0xF9 => Box::new(pc_advancing_instruction(bit::set_r(RegisterTarget::C, 7))),
        0xFA => Box::new(pc_advancing_instruction(bit::set_r(RegisterTarget::D, 7))),
        0xFB => Box::new(pc_advancing_instruction(bit::set_r(RegisterTarget::E, 7))),
        0xFC => Box::new(pc_advancing_instruction(bit::set_r(RegisterTarget::H, 7))),
        0xFD => Box::new(pc_advancing_instruction(bit::set_r(RegisterTarget::L, 7))),
        0xFE => Box::new(pc_advancing_instruction(bit::set_mem_at_hl(7))),
        0xFF => Box::new(pc_advancing_instruction(bit::set_r(RegisterTarget::A, 7))),
        other => {
            panic!("Unsupported prefixed instruction {:X}", other)
        }
    }
}
