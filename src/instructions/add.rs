use super::super::*;

impl CPU {
    pub fn add(&mut self, target: ArithmeticTarget) {
        let mut target_value = self.value_from_register(target);
        let current_value = self.registers.a;
        let (new_value, did_overflow) = current_value.overflowing_add(target_value);
        self.registers.a = new_value;
        self.registers.f.carry = did_overflow;
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
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
                f: FlagsRegister {
                    subtract: true,
                    ..Default::default()
                },
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
