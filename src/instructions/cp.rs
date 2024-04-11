use crate::cpu::{RegisterTarget, CPU};

pub fn cp_d8() -> impl Fn(&mut CPU) {
    move |cpu: &mut CPU| {
        let value = cpu.read_next_byte();
        let a = cpu.registers.get_u8(RegisterTarget::A);
        cpu.registers.f.zero = a == value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::{FlagsRegister, Registers};

    #[test]
    fn test_cp() {
        let mut cpu = CPU {
            registers: Registers {
                a: 13,
                ..Default::default()
            },
            ..Default::default()
        };
        cpu.bus.write_byte(cpu.pc, 13);
        cp_d8()(&mut cpu);
        assert!(cpu.registers.f.zero);
    }

    #[test]
    fn test_cp_not_equal() {
        let mut cpu = CPU {
            registers: Registers {
                a: 13,
                ..Default::default()
            },
            ..Default::default()
        };
        cpu.bus.write_byte(cpu.pc, 14);
        cp_d8()(&mut cpu);
        assert!(!cpu.registers.f.zero);
    }
}
