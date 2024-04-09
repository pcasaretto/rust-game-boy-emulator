mod adc;
mod add;
mod and;
mod call;
mod dec;
mod inc;
mod int;
mod jmp;
mod ld;
mod nop;
mod stack;
mod sub;

use super::*;

pub fn from_byte(byte: u8) -> Box<dyn Fn(&mut CPU)> {
    match byte {
        0x00 => Box::new(nop::nop()),
        0x01 => Box::new(ld::ld_d16_r16(Register16bTarget::BC)),
        0x02 => Box::new(ld::ld_r_mem_at_r16(
            Register16bTarget::BC,
            RegisterTarget::A,
        )),
        0x12 => Box::new(ld::ld_r_mem_at_r16(
            Register16bTarget::DE,
            RegisterTarget::A,
        )),
        0x11 => Box::new(ld::ld_d16_r16(Register16bTarget::DE)),

        0x04 => Box::new(inc::inc_r(RegisterTarget::B)),
        0x14 => Box::new(inc::inc_r(RegisterTarget::D)),
        0x23 => Box::new(inc::inc_r16(Register16bTarget::HL)),
        0x24 => Box::new(inc::inc_r(RegisterTarget::H)),
        0x0C => Box::new(inc::inc_r(RegisterTarget::C)),
        0x1C => Box::new(inc::inc_r(RegisterTarget::E)),
        0x2C => Box::new(inc::inc_r(RegisterTarget::L)),
        0x3C => Box::new(inc::inc_r(RegisterTarget::A)),

        0x05 => Box::new(inc::inc_r(RegisterTarget::B)),
        0x15 => Box::new(inc::inc_r(RegisterTarget::D)),
        0x25 => Box::new(inc::inc_r(RegisterTarget::H)),
        0x0D => Box::new(dec::dec_r(RegisterTarget::C)),
        0x1D => Box::new(dec::dec_r(RegisterTarget::E)),
        0x2D => Box::new(dec::dec_r(RegisterTarget::L)),
        0x3D => Box::new(dec::dec_r(RegisterTarget::A)),

        0x18 => Box::new(jmp::jr()),
        0x20 => Box::new(jmp::jr_nz()),

        0x31 => Box::new(ld::ld_d16_r16(Register16bTarget::SP)),
        0x21 => Box::new(ld::ld_d16_r16(Register16bTarget::HL)),
        0x2A => Box::new(ld::ld_hl_inc()),
        // 0x33 => Box::new(inc::inc_sp()),
        0x06 => Box::new(ld::ld_d8_r(RegisterTarget::B)),
        0x16 => Box::new(ld::ld_d8_r(RegisterTarget::D)),
        0x26 => Box::new(ld::ld_d8_r(RegisterTarget::H)),
        0x36 => Box::new(ld::ld_d8_mem_at_r16(Register16bTarget::HL)),
        0x0E => Box::new(ld::ld_d8_r(RegisterTarget::C)),
        0x1E => Box::new(ld::ld_d8_r(RegisterTarget::E)),
        0x2E => Box::new(ld::ld_d8_r(RegisterTarget::L)),
        0x3E => Box::new(ld::ld_d8_r(RegisterTarget::A)),
        0xC3 => Box::new(jmp::jmp_a16()),
        0x40 => Box::new(ld::ld_r_r(RegisterTarget::B, RegisterTarget::B)),
        0x41 => Box::new(ld::ld_r_r(RegisterTarget::B, RegisterTarget::C)),
        0x42 => Box::new(ld::ld_r_r(RegisterTarget::B, RegisterTarget::D)),
        0x43 => Box::new(ld::ld_r_r(RegisterTarget::B, RegisterTarget::E)),
        0x44 => Box::new(ld::ld_r_r(RegisterTarget::B, RegisterTarget::H)),
        0x45 => Box::new(ld::ld_r_r(RegisterTarget::B, RegisterTarget::L)),
        // 0x46 => Box::new(ld::ld_r_r(RegisterTarget::B, RegisterTarget::A)),
        0x47 => Box::new(ld::ld_r_r(RegisterTarget::B, RegisterTarget::A)),
        0x48 => Box::new(ld::ld_r_r(RegisterTarget::C, RegisterTarget::B)),
        0x49 => Box::new(ld::ld_r_r(RegisterTarget::C, RegisterTarget::C)),
        0x4a => Box::new(ld::ld_r_r(RegisterTarget::C, RegisterTarget::D)),
        0x4b => Box::new(ld::ld_r_r(RegisterTarget::C, RegisterTarget::E)),
        0x4c => Box::new(ld::ld_r_r(RegisterTarget::C, RegisterTarget::H)),
        0x4d => Box::new(ld::ld_r_r(RegisterTarget::C, RegisterTarget::L)),
        // 0x4e => Box::new(ld::ld_r_r(RegisterTarget::C, RegisterTarget::A)),
        0x4f => Box::new(ld::ld_r_r(RegisterTarget::C, RegisterTarget::A)),
        0x50 => Box::new(ld::ld_r_r(RegisterTarget::D, RegisterTarget::B)),
        0x51 => Box::new(ld::ld_r_r(RegisterTarget::D, RegisterTarget::C)),
        0x52 => Box::new(ld::ld_r_r(RegisterTarget::D, RegisterTarget::D)),
        0x53 => Box::new(ld::ld_r_r(RegisterTarget::D, RegisterTarget::E)),
        0x54 => Box::new(ld::ld_r_r(RegisterTarget::D, RegisterTarget::H)),
        0x55 => Box::new(ld::ld_r_r(RegisterTarget::D, RegisterTarget::L)),
        // 0x56 => Box::new(ld::ld_r_r(RegisterTarget::D, Register16bTarget::HL)),
        0x57 => Box::new(ld::ld_r_r(RegisterTarget::D, RegisterTarget::A)),
        0x58 => Box::new(ld::ld_r_r(RegisterTarget::E, RegisterTarget::B)),
        0x59 => Box::new(ld::ld_r_r(RegisterTarget::E, RegisterTarget::C)),
        0x5A => Box::new(ld::ld_r_r(RegisterTarget::E, RegisterTarget::D)),
        0x5B => Box::new(ld::ld_r_r(RegisterTarget::E, RegisterTarget::E)),
        0x5C => Box::new(ld::ld_r_r(RegisterTarget::E, RegisterTarget::H)),
        0x5D => Box::new(ld::ld_r_r(RegisterTarget::E, RegisterTarget::L)),
        // 0x5E => Box::new(ld::ld_r_r(RegisterTarget::E, Register16bTarget::HL)),
        0x5F => Box::new(ld::ld_r_r(RegisterTarget::E, RegisterTarget::A)),
        0x60 => Box::new(ld::ld_r_r(RegisterTarget::H, RegisterTarget::B)),
        0x61 => Box::new(ld::ld_r_r(RegisterTarget::H, RegisterTarget::C)),
        0x62 => Box::new(ld::ld_r_r(RegisterTarget::H, RegisterTarget::D)),
        0x63 => Box::new(ld::ld_r_r(RegisterTarget::H, RegisterTarget::E)),
        0x64 => Box::new(ld::ld_r_r(RegisterTarget::H, RegisterTarget::H)),
        0x65 => Box::new(ld::ld_r_r(RegisterTarget::H, RegisterTarget::L)),
        // 0x66 => Box::new(ld::ld_r_r(RegisterTarget::H, RegisterTarget::A)),
        0x67 => Box::new(ld::ld_r_r(RegisterTarget::H, RegisterTarget::A)),
        0x68 => Box::new(ld::ld_r_r(RegisterTarget::L, RegisterTarget::B)),
        0x69 => Box::new(ld::ld_r_r(RegisterTarget::L, RegisterTarget::C)),
        0x6A => Box::new(ld::ld_r_r(RegisterTarget::L, RegisterTarget::D)),
        0x6B => Box::new(ld::ld_r_r(RegisterTarget::L, RegisterTarget::E)),
        0x6C => Box::new(ld::ld_r_r(RegisterTarget::L, RegisterTarget::H)),
        0x6D => Box::new(ld::ld_r_r(RegisterTarget::L, RegisterTarget::L)),
        // 0x6E => Box::new(ld::ld_r_r(RegisterTarget::L, Register16bTarget::HL)),
        0x6F => Box::new(ld::ld_r_r(RegisterTarget::L, RegisterTarget::A)),
        // 0x70 => Box::new(ld::ld_r_r(RegisterTarget::H, RegisterTarget::B)),
        // 0x71 => Box::new(ld::ld_r_r(RegisterTarget::H, RegisterTarget::C)),
        // 0x72 => Box::new(ld::ld_r_r(RegisterTarget::H, RegisterTarget::D)),
        // 0x73 => Box::new(ld::ld_r_r(RegisterTarget::H, RegisterTarget::E)),
        // 0x74 => Box::new(ld::ld_r_r(RegisterTarget::H, RegisterTarget::H)),
        // 0x75 => Box::new(ld::ld_r_r(RegisterTarget::H, RegisterTarget::L)),
        // 0x76 => Box::new(ld::ld_r_r(RegisterTarget::H, RegisterTarget::A)),
        // 0x77 => Box::new(ld::ld_r_r(RegisterTarget::H, RegisterTarget::A)),
        0x78 => Box::new(ld::ld_r_r(RegisterTarget::A, RegisterTarget::B)),
        0x79 => Box::new(ld::ld_r_r(RegisterTarget::A, RegisterTarget::C)),
        0x7A => Box::new(ld::ld_r_r(RegisterTarget::A, RegisterTarget::D)),
        0x7B => Box::new(ld::ld_r_r(RegisterTarget::A, RegisterTarget::E)),
        0x7C => Box::new(ld::ld_r_r(RegisterTarget::A, RegisterTarget::H)),
        0x7D => Box::new(ld::ld_r_r(RegisterTarget::A, RegisterTarget::L)),
        // 0x7E => Box::new(ld::ld_r_r(RegisterTarget::A, Register16bTarget::HL)),
        0x7F => Box::new(ld::ld_r_r(RegisterTarget::A, RegisterTarget::A)),
        0x87 => Box::new(add::add(RegisterTarget::A)),
        0x80 => Box::new(add::add(RegisterTarget::B)),
        0x81 => Box::new(add::add(RegisterTarget::C)),
        0x82 => Box::new(add::add(RegisterTarget::D)),
        0x83 => Box::new(add::add(RegisterTarget::E)),
        0x84 => Box::new(add::add(RegisterTarget::H)),
        0x85 => Box::new(add::add(RegisterTarget::L)),
        0x8F => Box::new(adc::adc(RegisterTarget::A)),
        0x88 => Box::new(adc::adc(RegisterTarget::B)),
        0x89 => Box::new(adc::adc(RegisterTarget::C)),
        0x8A => Box::new(adc::adc(RegisterTarget::D)),
        0x8B => Box::new(adc::adc(RegisterTarget::E)),
        0x8C => Box::new(adc::adc(RegisterTarget::H)),
        0x8D => Box::new(adc::adc(RegisterTarget::L)),
        0x97 => Box::new(sub::sub_r_r_a(RegisterTarget::A)),
        0x90 => Box::new(sub::sub_r_r_a(RegisterTarget::B)),
        0x91 => Box::new(sub::sub_r_r_a(RegisterTarget::C)),
        0x92 => Box::new(sub::sub_r_r_a(RegisterTarget::D)),
        0x93 => Box::new(sub::sub_r_r_a(RegisterTarget::E)),
        0x94 => Box::new(sub::sub_r_r_a(RegisterTarget::H)),
        0x95 => Box::new(sub::sub_r_r_a(RegisterTarget::L)),

        0xA0 => Box::new(and::and(RegisterTarget::B)),
        0xA1 => Box::new(and::and(RegisterTarget::C)),
        0xA2 => Box::new(and::and(RegisterTarget::D)),
        0xA3 => Box::new(and::and(RegisterTarget::E)),
        0xA4 => Box::new(and::and(RegisterTarget::H)),
        0xA5 => Box::new(and::and(RegisterTarget::L)),
        0xA6 => Box::new(and::and_mem_at_r16(Register16bTarget::HL)),
        0xA7 => Box::new(and::and(RegisterTarget::A)),

        0xC1 => Box::new(stack::pop(Register16bTarget::BC)),
        0xD1 => Box::new(stack::pop(Register16bTarget::DE)),
        0xE1 => Box::new(stack::pop(Register16bTarget::HL)),
        0xF1 => Box::new(stack::pop(Register16bTarget::AF)),

        0xC5 => Box::new(stack::push(Register16bTarget::BC)),
        0xD5 => Box::new(stack::push(Register16bTarget::DE)),
        0xE5 => Box::new(stack::push(Register16bTarget::HL)),
        0xF5 => Box::new(stack::push(Register16bTarget::AF)),

        0xCD => Box::new(call::call_a16()),
        0xC9 => Box::new(call::ret()),

        0xD6 => Box::new(sub::sub_d8()),

        0xE0 => Box::new(ld::ld_a_mem_at_d8()),
        0xEA => Box::new(ld::ld_r_mem_at_d16(RegisterTarget::A)),

        0xF3 => Box::new(int::di()),
        0xFB => Box::new(int::ei()),

        other => {
            panic!("Unsupported instruction {:X}", other)
        }
    }
}
