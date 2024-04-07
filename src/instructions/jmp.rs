use crate::CPU;

pub fn jmp_a16() -> impl Fn(&mut CPU) {
    move |cpu: &mut CPU| {
        let low = cpu.bus.memory[(cpu.pc) as usize];
        let high = cpu.bus.memory[(cpu.pc + 1) as usize];
        cpu.pc = u16::from_le_bytes([low, high]);
    }
}

pub fn jr_nz() -> impl Fn(&mut CPU) {
    move |cpu: &mut CPU| {
        if !cpu.registers.f.zero {
            return;
        }
        let offset = cpu.bus.memory[cpu.pc as usize];
        cpu.pc = cpu.pc.wrapping_add(offset as i8 as u16);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jmp_a16() {
        let mut cpu = CPU::default();
        cpu.bus.memory[0] = 0x01;
        cpu.bus.memory[1] = 0x02;
        jmp_a16()(&mut cpu);
        assert_eq!(cpu.pc, 0x0201);
    }

    #[test]
    fn test_jr_nz_flag_unset() {
        let mut cpu = CPU::default();
        cpu.pc = 0x1000;
        cpu.registers.f.zero = false;
        cpu.bus.memory[0x1000] = 0x01;
        jr_nz()(&mut cpu);
        assert_eq!(cpu.pc, 0x1000);
    }

    #[test]
    fn test_jr_nz_flag_set() {
        let mut cpu = CPU::default();
        cpu.pc = 0x1000;
        cpu.registers.f.zero = true;
        cpu.bus.memory[0x1000] = 0x05;
        jr_nz()(&mut cpu);
        assert_eq!(cpu.pc, 0x1005);
    }

    #[test]
    fn test_jr_nz_flag_set_signed_negative() {
        let mut cpu = CPU::default();
        cpu.pc = 0x1000;
        cpu.registers.f.zero = true;
        cpu.bus.memory[0x1005] = -5i8 as u8;
        jr_nz()(&mut cpu);
        assert_eq!(cpu.pc, 0x1000);
    }
}
