mod adc;
mod add;
mod nop;
mod sub;

use super::*;

impl Instruction {
    pub fn from_byte(byte: u8) -> Instruction {
        match byte {
            0x00 => Instruction::NOP,
            0x87 => Instruction::ADD(ArithmeticTarget::A),
            0x80 => Instruction::ADD(ArithmeticTarget::B),
            0x81 => Instruction::ADD(ArithmeticTarget::C),
            0x82 => Instruction::ADD(ArithmeticTarget::D),
            0x83 => Instruction::ADD(ArithmeticTarget::E),
            0x84 => Instruction::ADD(ArithmeticTarget::H),
            0x85 => Instruction::ADD(ArithmeticTarget::L),
            // 0x86 => Instruction::ADD(ArithmeticTarget::HLIndirect),
            0x8F => Instruction::ADC(ArithmeticTarget::A),
            0x88 => Instruction::ADC(ArithmeticTarget::B),
            0x89 => Instruction::ADC(ArithmeticTarget::C),
            0x8A => Instruction::ADC(ArithmeticTarget::D),
            0x8B => Instruction::ADC(ArithmeticTarget::E),
            0x8C => Instruction::ADC(ArithmeticTarget::H),
            0x8D => Instruction::ADC(ArithmeticTarget::L),
            // 0x8E => Instruction::ADC(ArithmeticTarget::HLIndirect),
            0x97 => Instruction::SUB(ArithmeticTarget::A),
            0x90 => Instruction::SUB(ArithmeticTarget::B),
            0x91 => Instruction::SUB(ArithmeticTarget::C),
            0x92 => Instruction::SUB(ArithmeticTarget::D),
            0x93 => Instruction::SUB(ArithmeticTarget::E),
            0x94 => Instruction::SUB(ArithmeticTarget::H),
            0x95 => Instruction::SUB(ArithmeticTarget::L),
            _ => panic!("Unknown instruction: 0x{:x}", byte),
        }
    }
}
