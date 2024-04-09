use crate::{Register16bTarget, RegisterTarget, CPU};

pub fn ld_d16_r16(reg: Register16bTarget) -> impl Fn(&mut CPU) {
    move |cpu: &mut CPU| {
        let low = cpu.bus.memory[(cpu.pc + 1) as usize];
        let high = cpu.bus.memory[(cpu.pc + 2) as usize];
        cpu.registers.set_u16(reg, u16::from_le_bytes([low, high]));
        cpu.pc = cpu.pc.wrapping_add(2);
    }
}

pub fn ld_d8_u8(reg: RegisterTarget) -> impl Fn(&mut CPU) {
    move |cpu: &mut CPU| {
        let value = cpu.bus.memory[(cpu.pc + 1) as usize];
        cpu.registers.set_u8(reg, value);
        cpu.pc = cpu.pc.wrapping_add(1);
    }
}

pub fn ld_r_r(src: RegisterTarget, dest: RegisterTarget) -> impl Fn(&mut CPU) {
    move |cpu: &mut CPU| {
        let value = cpu.registers.get_u8(src);
        cpu.registers.set_u8(dest, value);
    }
}

pub fn ld_hl_inc() -> impl Fn(&mut CPU) {
    move |cpu: &mut CPU| {
        let hl = cpu.registers.get_u16(Register16bTarget::HL);
        let value = cpu.bus.memory[hl as usize];
        cpu.registers.set_u8(RegisterTarget::A, value);
        cpu.registers
            .set_u16(Register16bTarget::HL, hl.wrapping_add(1));
    }
}

pub fn ld_u_16_r(src: RegisterTarget) -> impl Fn(&mut CPU) {
    move |cpu: &mut CPU| {
        let addr = u16::from_be_bytes([
            cpu.bus.memory[cpu.pc as usize],
            cpu.bus.memory[(cpu.pc + 1) as usize],
        ]);
        let value = cpu.registers.get_u8(src);
        cpu.bus.memory[addr as usize] = value;
        cpu.pc = cpu.pc.wrapping_add(1);
    }
}

pub fn ld_mem_at_r16_r(reg: Register16bTarget, target: RegisterTarget) -> impl Fn(&mut CPU) {
    move |cpu: &mut CPU| {
        let addr = cpu.registers.get_u16(reg);
        let value = cpu.registers.get_u8(target);
        cpu.bus.memory[addr as usize] = value;
    }
}

pub fn ld_d8_mem_at_r16(reg: Register16bTarget) -> impl Fn(&mut CPU) {
    move |cpu: &mut CPU| {
        let addr = cpu.registers.get_u16(reg);
        let value = cpu.bus.memory[cpu.pc as usize];
        cpu.bus.memory[addr as usize] = value;
    }
}

pub fn ld_d8_r(target: RegisterTarget) -> impl Fn(&mut CPU) {
    move |cpu: &mut CPU| {
        let value = cpu.bus.memory[cpu.pc as usize];
        cpu.registers.set_u8(target, value);
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
        ld_d16_r16(Register16bTarget::BC)(&mut cpu);
        assert_eq!(cpu.registers.get_u16(Register16bTarget::BC), 0x0201);
        assert_eq!(cpu.pc, 2);
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
    }

    #[test]
    fn test_d8_u8() {
        let mut cpu = CPU::default();
        cpu.bus.memory[1] = 0x01;
        ld_d8_u8(RegisterTarget::B)(&mut cpu);
        assert_eq!(cpu.registers.get_u8(RegisterTarget::B), 0x01);
        assert_eq!(cpu.pc, 1);
    }

    #[test]
    fn test_ld_hl_inc() {
        let mut cpu = CPU::default();
        cpu.registers.set_u16(Register16bTarget::HL, 0x1000);
        cpu.bus.memory[0x1000] = 0x01;
        ld_hl_inc()(&mut cpu);
        assert_eq!(cpu.registers.get_u8(RegisterTarget::A), 0x01);
        assert_eq!(cpu.registers.get_u16(Register16bTarget::HL), 0x1001);
    }

    #[test]
    fn test_ld_mem_at_u16_r() {
        let mut cpu = CPU::default();
        cpu.registers.set_u16(Register16bTarget::BC, 0x1000);
        cpu.registers.set_u8(RegisterTarget::A, 0x34);
        ld_mem_at_r16_r(Register16bTarget::BC, RegisterTarget::A)(&mut cpu);
        assert_eq!(cpu.bus.memory[0x1000], 0x34);
    }

    #[test]
    fn test_ld_u_16_r() {
        let mut cpu = CPU::default();
        cpu.registers.set_u8(RegisterTarget::A, 0x34);
        cpu.bus.memory[0] = 0x01;
        cpu.bus.memory[1] = 0x23;
        ld_u_16_r(RegisterTarget::A)(&mut cpu);
        assert_eq!(cpu.bus.memory[0x0123], 0x34);
    }

    #[test]
    fn test_ld_u_16_r_advances_pc() {
        let mut cpu = CPU::default();
        ld_u_16_r(RegisterTarget::A)(&mut cpu);
        assert_eq!(cpu.pc, 1);
    }

    #[test]
    fn test_ld_d8_r() {
        let mut cpu = CPU::default();
        cpu.bus.memory[0] = 0x34;
        ld_d8_r(RegisterTarget::A)(&mut cpu);
        assert_eq!(cpu.registers.get_u8(RegisterTarget::A), 0x34);
    }

    #[test]
    fn test_ld_d8_mem_at_r16() {
        let mut cpu = CPU::default();
        cpu.registers.set_u16(Register16bTarget::HL, 0x1000);
        cpu.bus.memory[0] = 0x34;
        ld_d8_mem_at_r16(Register16bTarget::HL)(&mut cpu);
        assert_eq!(cpu.bus.memory[0x1000], 0x34);
    }
}
