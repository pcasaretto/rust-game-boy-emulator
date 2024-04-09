use crate::{Register16bTarget, RegisterTarget, CPU};

pub fn ld_d16_r16(reg: Register16bTarget) -> impl Fn(&mut CPU) {
    move |cpu: &mut CPU| {
        let low = cpu.read_next_byte();
        let high = cpu.read_next_byte();
        let addr = u16::from_be_bytes([high, low]);
        cpu.registers.set_u16(reg, addr);
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
        let value = cpu.bus.read_byte(hl);
        cpu.registers.set_u8(RegisterTarget::A, value);
        cpu.registers
            .set_u16(Register16bTarget::HL, hl.wrapping_add(1));
    }
}

pub fn ld_r_mem_at_d16(src: RegisterTarget) -> impl Fn(&mut CPU) {
    move |cpu: &mut CPU| {
        let addr = u16::from_be_bytes([cpu.read_next_byte(), cpu.read_next_byte()]);
        let value = cpu.registers.get_u8(src);
        cpu.bus.write_byte(addr, value);
    }
}

pub fn ld_r_mem_at_r16(reg: Register16bTarget, target: RegisterTarget) -> impl Fn(&mut CPU) {
    move |cpu: &mut CPU| {
        let addr = cpu.registers.get_u16(reg);
        let value = cpu.registers.get_u8(target);
        cpu.bus.write_byte(addr, value);
    }
}

pub fn ld_d8_mem_at_r16(reg: Register16bTarget) -> impl Fn(&mut CPU) {
    move |cpu: &mut CPU| {
        let addr = cpu.registers.get_u16(reg);
        let value = cpu.read_next_byte();
        cpu.bus.write_byte(addr, value);
    }
}

pub fn ld_d8_r(target: RegisterTarget) -> impl Fn(&mut CPU) {
    move |cpu: &mut CPU| {
        let value = cpu.read_next_byte();
        cpu.registers.set_u8(target, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Registers;

    #[test]
    fn test_ld_d16_r16() {
        let mut cpu = CPU::default();
        cpu.bus.memory[0] = 0x01;
        cpu.bus.memory[1] = 0x02;
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
    fn test_ld_hl_inc() {
        let mut cpu = CPU::default();
        cpu.registers.set_u16(Register16bTarget::HL, 0x1000);
        cpu.bus.memory[0x1000] = 0x01;
        ld_hl_inc()(&mut cpu);
        assert_eq!(cpu.registers.get_u8(RegisterTarget::A), 0x01);
        assert_eq!(cpu.registers.get_u16(Register16bTarget::HL), 0x1001);
    }

    #[test]
    fn test_ld_r_mem_at_r16() {
        let mut cpu = CPU::default();
        cpu.registers.set_u16(Register16bTarget::BC, 0x1000);
        cpu.registers.set_u8(RegisterTarget::A, 0x34);
        ld_r_mem_at_r16(Register16bTarget::BC, RegisterTarget::A)(&mut cpu);
        assert_eq!(cpu.bus.read_byte(0x1000), 0x34);
    }

    #[test]
    fn test_ld_u_16_r() {
        let mut cpu = CPU::default();
        cpu.registers.set_u8(RegisterTarget::A, 0x34);
        cpu.bus.memory[0] = 0x01;
        cpu.bus.memory[1] = 0x23;
        ld_r_mem_at_d16(RegisterTarget::A)(&mut cpu);
        assert_eq!(cpu.bus.memory[0x0123], 0x34);
    }

    #[test]
    fn test_ld_u_16_r_advances_pc() {
        let mut cpu = CPU::default();
        ld_r_mem_at_d16(RegisterTarget::A)(&mut cpu);
        assert_eq!(cpu.pc, 2);
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
