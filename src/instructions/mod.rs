mod adc;
mod add;
mod jmp;
mod nop;
mod sub;

use super::*;

pub fn from_byte(byte: u8) -> Box<dyn Fn(&mut CPU)> {
    match byte {
        0x00 => Box::new(nop::nop()),
        0x87 => Box::new(add::add(ArithmeticTarget::A)),
        0x80 => Box::new(add::add(ArithmeticTarget::B)),
        0x81 => Box::new(add::add(ArithmeticTarget::C)),
        0x82 => Box::new(add::add(ArithmeticTarget::D)),
        0x83 => Box::new(add::add(ArithmeticTarget::E)),
        0x84 => Box::new(add::add(ArithmeticTarget::H)),
        0x85 => Box::new(add::add(ArithmeticTarget::L)),
        0x8F => Box::new(adc::adc(ArithmeticTarget::A)),
        0x88 => Box::new(adc::adc(ArithmeticTarget::B)),
        0x89 => Box::new(adc::adc(ArithmeticTarget::C)),
        0x8A => Box::new(adc::adc(ArithmeticTarget::D)),
        0x8B => Box::new(adc::adc(ArithmeticTarget::E)),
        0x8C => Box::new(adc::adc(ArithmeticTarget::H)),
        0x8D => Box::new(adc::adc(ArithmeticTarget::L)),
        0x97 => Box::new(sub::sub(ArithmeticTarget::A)),
        0x90 => Box::new(sub::sub(ArithmeticTarget::B)),
        0x91 => Box::new(sub::sub(ArithmeticTarget::C)),
        0x92 => Box::new(sub::sub(ArithmeticTarget::D)),
        0x93 => Box::new(sub::sub(ArithmeticTarget::E)),
        0x94 => Box::new(sub::sub(ArithmeticTarget::H)),
        0x95 => Box::new(sub::sub(ArithmeticTarget::L)),
        0xC3 => Box::new(jmp::jmp_a16()),

        other => {
            panic!("Unsupported instruction {:?}", other)
        }
    }
}
