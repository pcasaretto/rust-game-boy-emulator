use crate::{Register16bTarget, CPU};

pub fn ld_d16_u16(reg: Register16bTarget) -> impl Fn(&mut CPU) {
    move |cpu: &mut CPU| {
        let low = cpu.bus.memory[(cpu.pc + 1) as usize];
        let high = cpu.bus.memory[(cpu.pc + 2) as usize];
        cpu.registers.set_u16(reg, u16::from_le_bytes([low, high]));
        cpu.pc += 3;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ld_d16_u16() {
        let mut cpu = CPU::default();
        cpu.bus.memory[1] = 0x01;
        cpu.bus.memory[2] = 0x02;
        ld_d16_u16(Register16bTarget::BC)(&mut cpu);
        assert_eq!(cpu.registers.get_u16(Register16bTarget::BC), 0x0201);
        assert_eq!(cpu.pc, 3);
    }
}
