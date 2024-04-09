use crate::{Register16bTarget, CPU};

pub fn push(reg: Register16bTarget) -> impl Fn(&mut CPU) {
    move |cpu: &mut CPU| {
        let value = cpu.registers.get_u16(reg);
        let [high, low] = value.to_be_bytes();
        cpu.registers.sp = cpu.registers.sp.wrapping_sub(1);
        cpu.bus.write_byte(cpu.registers.sp, high);
        cpu.registers.sp = cpu.registers.sp.wrapping_sub(1);
        cpu.bus.write_byte(cpu.registers.sp, low);
    }
}

pub fn pop(reg: Register16bTarget) -> impl Fn(&mut CPU) {
    move |cpu: &mut CPU| {
        let low = cpu.bus.read_byte(cpu.registers.sp);
        cpu.registers.sp = cpu.registers.sp.wrapping_add(1);
        let high = cpu.bus.read_byte(cpu.registers.sp);
        cpu.registers.sp = cpu.registers.sp.wrapping_add(1);
        cpu.registers.set_u16(reg, u16::from_be_bytes([high, low]));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FlagsRegister, Registers};

    #[test]
    fn test_push() {
        let mut cpu = CPU {
            registers: Registers {
                sp: 0xFFFE,
                b: 0x01,
                c: 0x02,
                ..Default::default()
            },
            ..Default::default()
        };
        push(Register16bTarget::BC)(&mut cpu);
        assert_eq!(cpu.registers.sp, 0xFFFC);
        assert_eq!(cpu.bus.memory[0xFFFD], 0x01);
        assert_eq!(cpu.bus.memory[0xFFFC], 0x02);
    }

    #[test]
    fn test_pop() {
        let mut cpu = CPU {
            registers: Registers {
                sp: 0xFFFC,
                b: 0x00,
                c: 0x00,
                ..Default::default()
            },
            ..Default::default()
        };
        cpu.bus.memory[0xFFFC] = 0x01;
        cpu.bus.memory[0xFFFD] = 0x02;
        pop(Register16bTarget::BC)(&mut cpu);
        assert_eq!(cpu.registers.get_u16(Register16bTarget::BC), 0x0201);
        assert_eq!(cpu.registers.sp, 0xFFFE);
    }
}
