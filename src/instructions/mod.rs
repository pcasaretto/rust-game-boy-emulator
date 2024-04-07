mod adc;
mod add;
mod inc;
mod jmp;
mod ld;
mod nop;
mod sub;

use super::*;

pub fn from_byte(byte: u8) -> Box<dyn Fn(&mut CPU)> {
    match byte {
        0x00 => Box::new(nop::nop()),
        0x21 => Box::new(ld::ld_d16_u16(Register16bTarget::HL)),
        0x33 => Box::new(inc::inc_sp()),
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
        0x97 => Box::new(sub::sub(RegisterTarget::A)),
        0x90 => Box::new(sub::sub(RegisterTarget::B)),
        0x91 => Box::new(sub::sub(RegisterTarget::C)),
        0x92 => Box::new(sub::sub(RegisterTarget::D)),
        0x93 => Box::new(sub::sub(RegisterTarget::E)),
        0x94 => Box::new(sub::sub(RegisterTarget::H)),
        0x95 => Box::new(sub::sub(RegisterTarget::L)),
        0xC3 => Box::new(jmp::jmp_a16()),

        other => {
            panic!("Unsupported instruction {:X}", other)
        }
    }
}
