#[derive(Debug)]
enum Instruction {
    ADD(ArithmeticTarget),
    ADC(ArithmeticTarget),
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

impl Default for FlagsRegister {
    fn default() -> Self {
        FlagsRegister {
            zero: false,
            subtract: false,
            half_carry: false,
            carry: false,
        }
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

struct CPU {
    registers: Registers,
    pc: u16,
    bus: MemoryBus,
}

struct MemoryBus {
    memory: [u8; 0xFFFF],
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
    fn step(&mut self) {
        let instruction_byte = self.bus.read_byte(self.pc);
        let instruction = Instruction::from_byte(instruction_byte);
        self.execute(instruction) // each instruction is responsible for updating the program counter
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADD(target) => self.add(target),
            Instruction::ADC(target) => self.adc(target),
            _ => {
                panic!("Unsupported instruction {:?}", instruction)
            }
        }
    }

    fn read_single_register(&self, target: ArithmeticTarget) -> u8 {
        match target {
            ArithmeticTarget::C => self.registers.c,
            other => panic!("Unsupported target: {:?}", other),
        }
    }
}

mod instructions;

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
