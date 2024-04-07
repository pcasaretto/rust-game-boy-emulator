use crate::{Register16bTarget, RegisterTarget, CPU};

pub fn ld_d16_u16(reg: Register16bTarget) -> impl Fn(&mut CPU) {
    move |cpu: &mut CPU| {
        let low = cpu.bus.memory[(cpu.pc + 1) as usize];
        let high = cpu.bus.memory[(cpu.pc + 2) as usize];
        cpu.registers.set_u16(reg, u16::from_le_bytes([low, high]));
        cpu.pc += 3;
    }
}

pub fn ld_d8_u8(reg: RegisterTarget) -> impl Fn(&mut CPU) {
    move |cpu: &mut CPU| {
        let value = cpu.bus.memory[(cpu.pc + 1) as usize];
        cpu.registers.set_u8(reg, value);
        cpu.pc += 2;
    }
}

pub fn ld_r_r(src: RegisterTarget, dest: RegisterTarget) -> impl Fn(&mut CPU) {
    move |cpu: &mut CPU| {
        let value = cpu.registers.get_u8(src);
        cpu.registers.set_u8(dest, value);
        cpu.pc += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Registers;

    #[test]
    fn test_ld_d16_u16() {
        let mut cpu = CPU::default();
        cpu.bus.memory[1] = 0x01;
        cpu.bus.memory[2] = 0x02;
        ld_d16_u16(Register16bTarget::BC)(&mut cpu);
        assert_eq!(cpu.registers.get_u16(Register16bTarget::BC), 0x0201);
        assert_eq!(cpu.pc, 3);
    }

    #[test]
    fn test_ld_r_r() {
        let mut cpu = CPU {
            registers: Registers {
                a: 0,
                b: 1,
                ..Default::default()
            },
            ..Default::default()
        };
        ld_r_r(RegisterTarget::B, RegisterTarget::A)(&mut cpu);
        assert_eq!(cpu.registers.get_u8(RegisterTarget::A), 0x01);
        assert_eq!(cpu.pc, 1);
    }

    #[test]
    fn test_d8_u8() {
        let mut cpu = CPU::default();
        cpu.bus.memory[1] = 0x01;
        ld_d8_u8(RegisterTarget::B)(&mut cpu);
        assert_eq!(cpu.registers.get_u8(RegisterTarget::B), 0x01);
        assert_eq!(cpu.pc, 2);
    }
}
