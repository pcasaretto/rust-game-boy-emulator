use crate::CPU;

pub fn jmp_a16() -> impl Fn(&mut CPU) {
    move |cpu: &mut CPU| {
        let low = cpu.bus.memory[(cpu.pc + 1) as usize];
        let high = cpu.bus.memory[(cpu.pc + 2) as usize];
        cpu.pc = u16::from_le_bytes([low, high]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jmp_a16() {
        let mut cpu = CPU::default();
        cpu.bus.memory[1] = 0x01;
        cpu.bus.memory[2] = 0x02;
        jmp_a16()(&mut cpu);
        assert_eq!(cpu.pc, 0x0201);
    }
}
