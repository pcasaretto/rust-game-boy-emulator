mod instructions;
use log;

#[derive(Debug, Copy, Clone)]
pub enum RegisterTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Debug, Copy, Clone)]
pub enum Register16bTarget {
    BC,
    DE,
    HL,
    SP,
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
    sp: u16,
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
        }
    }

    pub fn get_u16(&self, target: Register16bTarget) -> u16 {
        match target {
            Register16bTarget::BC => u16::from_be_bytes([self.b, self.c]),
            Register16bTarget::DE => u16::from_be_bytes([self.d, self.e]),
            Register16bTarget::HL => u16::from_be_bytes([self.h, self.l]),
            Register16bTarget::SP => self.sp,
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
        }
    }
}

#[derive(Default)]
pub struct FlagsRegister {
    pub zero: bool,
    pub subtract: bool,
    pub half_carry: bool,
    pub carry: bool,
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

/// CPU
pub struct CPU {
    pub registers: Registers,
    pub pc: u16,
    pub bus: MemoryBus,
}

pub struct MemoryBus {
    pub memory: [u8; 0xFFFF],
}

impl MemoryBus {
    fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
}

impl Default for CPU {
    fn default() -> Self {
        CPU {
            registers: Registers::default(),
            pc: 0,
            bus: MemoryBus {
                memory: [0; 0xFFFF],
            },
        }
    }
}

impl CPU {
    pub fn step(&mut self) {
        let instruction_byte = self.bus.read_byte(self.pc);
        self.pc = self.pc.wrapping_add(1);
        let instruction = instructions::from_byte(instruction_byte);
        instruction(self);
        log::debug!(
            "Executed instruction {:2X}, new PC is {:4X}",
            instruction_byte,
            self.pc
        );
    }

    fn read_single_register(&self, target: RegisterTarget) -> u8 {
        match target {
            RegisterTarget::A => self.registers.a,
            RegisterTarget::B => self.registers.b,
            RegisterTarget::C => self.registers.c,
            RegisterTarget::D => self.registers.d,
            RegisterTarget::E => self.registers.e,
            RegisterTarget::H => self.registers.h,
            RegisterTarget::L => self.registers.l,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut cpu = CPU {
            registers: Registers {
                a: 3,
                c: 4,
                f: FlagsRegister::from(0),
                ..Default::default()
            },
            bus: MemoryBus {
                memory: [0x81; 0xFFFF],
            },
            pc: 1245,
            ..Default::default()
        };
        cpu.step();
        assert_eq!(cpu.registers.a, 7);
        assert_eq!(cpu.pc, 1246);
    }
}
