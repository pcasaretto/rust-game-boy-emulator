enum Instruction {
    ADD(ArithmeticTarget),
}

#[derive(Debug)]
enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

struct Registers {
    a: u8,
    c: u8,
    f: FlagsRegister,
}

impl Default for Registers {
    fn default() -> Self {
        Registers {
            a: 0,
            c: 0,
            f: FlagsRegister::from(0),
        }
    }
}

struct FlagsRegister {
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool,
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

struct CPU {
    registers: Registers,
}

impl Default for CPU {
    fn default() -> Self {
        CPU {
            registers: Registers::default(),
        }
    }
}

impl CPU {
    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADD(target) => self.add(target),
            _ => {
                panic!("Unsupported instruction {:?}", instruction)
            }
        }
    }

    fn add(&mut self, target: ArithmeticTarget) {
        let mut target_value = 0;
        let current_value = self.registers.a;
        match target {
            ArithmeticTarget::C => {
                target_value = self.registers.c;
            }
            other => {
                panic!("Unsupported target: {:?}", other)
            }
        }
        let (new_value, did_overflow) = current_value.overflowing_add(target_value);
        self.registers.a = new_value;
        self.registers.f.carry = did_overflow;
        self.registers.f.zero = new_value == 0;
        self.registers.f.half_carry = (current_value & 0xF) + (target_value & 0xF) > 0xF;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut cpu = CPU {
            registers: Registers {
                a: 0,
                c: 1,
                f: FlagsRegister::from(0),
                ..Default::default()
            },
            ..Default::default()
        };
        cpu.execute(Instruction::ADD(ArithmeticTarget::C));
        assert_eq!(cpu.registers.a, 1);
    }

    #[test]
    fn test_add_overflow() {
        let mut cpu = CPU {
            registers: Registers {
                a: 255,
                c: 1,
                f: FlagsRegister::from(0),
                ..Default::default()
            },
            ..Default::default()
        };
        cpu.execute(Instruction::ADD(ArithmeticTarget::C));
        assert_eq!(cpu.registers.a, 0);
    }

    #[test]
    fn test_add_carry_flag() {
        let mut cpu = CPU {
            registers: Registers {
                a: 255,
                c: 1,
                f: FlagsRegister::from(0),
                ..Default::default()
            },
            ..Default::default()
        };
        cpu.execute(Instruction::ADD(ArithmeticTarget::C));
        assert!(cpu.registers.f.carry);
    }

    #[test]
    fn test_add_zero_flag() {
        let mut cpu = CPU {
            registers: Registers {
                a: 0,
                c: 0,
                f: FlagsRegister::from(0),
                ..Default::default()
            },
            ..Default::default()
        };
        cpu.execute(Instruction::ADD(ArithmeticTarget::C));
        assert!(cpu.registers.f.zero);
    }

    #[test]
    fn test_add_substract_flag() {
        let mut cpu = CPU {
            registers: Registers {
                a: 0,
                c: 1,
                f: FlagsRegister::from(0),
                ..Default::default()
            },
            ..Default::default()
        };
        cpu.execute(Instruction::ADD(ArithmeticTarget::C));
        assert!(!cpu.registers.f.subtract);
    }

    #[test]
    fn test_add_half_carry_flag() {
        let mut cpu = CPU {
            registers: Registers {
                a: 0b00001111,
                c: 0b00000001,
                f: FlagsRegister::from(0),
                ..Default::default()
            },
            ..Default::default()
        };
        cpu.execute(Instruction::ADD(ArithmeticTarget::C));
        assert!(cpu.registers.f.half_carry);
    }
}
