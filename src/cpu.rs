use std::fmt::Debug;
use std::fmt::Formatter;

#[derive(Debug, Copy, Clone)]
pub enum RegisterTarget {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
}

#[derive(Debug, Copy, Clone)]
pub enum Register16bTarget {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

#[derive(Default)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub f: FlagsRegister,
    pub sp: u16,
    pub pc: u16,
}

impl Debug for Registers {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Registers")
            .field("a", &format_args!("{:02X}", self.a))
            .field("b", &format_args!("{:02X}", self.b))
            .field("c", &format_args!("{:02X}", self.c))
            .field("d", &format_args!("{:02X}", self.d))
            .field("e", &format_args!("{:02X}", self.e))
            .field("h", &format_args!("{:02X}", self.h))
            .field("l", &format_args!("{:02X}", self.l))
            .field("f", &self.f)
            .field("sp", &format_args!("{:04X}", self.sp))
            .field("pc", &format_args!("{:04X}", self.pc))
            .finish()
    }
}

impl Registers {
    pub fn get_u8(&self, target: RegisterTarget) -> u8 {
        match target {
            RegisterTarget::A => self.a,
            RegisterTarget::B => self.b,
            RegisterTarget::C => self.c,
            RegisterTarget::D => self.d,
            RegisterTarget::E => self.e,
            RegisterTarget::H => self.h,
            RegisterTarget::L => self.l,
            RegisterTarget::F => u8::from(self.f),
        }
    }

    pub fn set_u8(&mut self, target: RegisterTarget, value: u8) {
        match target {
            RegisterTarget::A => self.a = value,
            RegisterTarget::B => self.b = value,
            RegisterTarget::C => self.c = value,
            RegisterTarget::D => self.d = value,
            RegisterTarget::E => self.e = value,
            RegisterTarget::H => self.h = value,
            RegisterTarget::L => self.l = value,
            RegisterTarget::F => self.f = FlagsRegister::from(value),
        }
    }

    pub fn get_u16(&self, target: Register16bTarget) -> u16 {
        match target {
            Register16bTarget::AF => u16::from_be_bytes([self.a, u8::from(self.f)]),
            Register16bTarget::BC => u16::from_be_bytes([self.b, self.c]),
            Register16bTarget::DE => u16::from_be_bytes([self.d, self.e]),
            Register16bTarget::HL => u16::from_be_bytes([self.h, self.l]),
            Register16bTarget::SP => self.sp,
            Register16bTarget::PC => self.pc,
        }
    }

    pub fn set_u16(&mut self, target: Register16bTarget, value: u16) {
        let [high, low] = value.to_be_bytes();
        match target {
            Register16bTarget::BC => {
                self.b = high;
                self.c = low;
            }
            Register16bTarget::DE => {
                self.d = high;
                self.e = low;
            }
            Register16bTarget::HL => {
                self.h = high;
                self.l = low;
            }
            Register16bTarget::SP => self.sp = value,
            Register16bTarget::PC => self.pc = value,
            Register16bTarget::AF => {
                self.a = high;
                self.f = FlagsRegister::from(low);
            }
        }
    }
}

#[derive(Default, Copy, Clone)]
pub struct FlagsRegister {
    pub zero: bool,
    pub subtract: bool,
    pub half_carry: bool,
    pub carry: bool,
}

impl Debug for FlagsRegister {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FlagsRegister")
            .field("Z", &self.zero)
            .field("S", &self.subtract)
            .field("H", &self.half_carry)
            .field("C", &self.carry)
            .finish()
    }
}

const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;

impl std::convert::From<FlagsRegister> for u8 {
    fn from(flag: FlagsRegister) -> u8 {
        (if flag.zero { 1 } else { 0 }) << ZERO_FLAG_BYTE_POSITION
            | (if flag.subtract { 1 } else { 0 }) << SUBTRACT_FLAG_BYTE_POSITION
            | (if flag.half_carry { 1 } else { 0 }) << HALF_CARRY_FLAG_BYTE_POSITION
            | (if flag.carry { 1 } else { 0 }) << CARRY_FLAG_BYTE_POSITION
    }
}

impl std::convert::From<u8> for FlagsRegister {
    fn from(byte: u8) -> Self {
        let zero = ((byte >> ZERO_FLAG_BYTE_POSITION) & 0b1) != 0;
        let subtract = ((byte >> SUBTRACT_FLAG_BYTE_POSITION) & 0b1) != 0;
        let half_carry = ((byte >> HALF_CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;
        let carry = ((byte >> CARRY_FLAG_BYTE_POSITION) & 0b1) != 0;

        FlagsRegister {
            zero,
            subtract,
            half_carry,
            carry,
        }
    }
}

#[derive(Default)]
pub struct CPU {
    pub registers: Registers,
    pub halted: bool,
}
