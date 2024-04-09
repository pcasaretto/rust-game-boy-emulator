use crate::CPU;

pub fn call_a16() -> impl Fn(&mut CPU) {
    move |cpu: &mut CPU| {
        let [pc_high, pc_low] = cpu.pc.to_be_bytes();
        cpu.registers.sp = cpu.registers.sp.wrapping_sub(1);
        cpu.bus.write_byte(cpu.registers.sp, pc_high);
        cpu.registers.sp = cpu.registers.sp.wrapping_sub(1);
        cpu.bus.write_byte(cpu.registers.sp, pc_low);

        let low = cpu.read_next_byte();
        let high = cpu.read_next_byte();

        cpu.pc = u16::from_le_bytes([low, high]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_call_a16() {
        let mut cpu = CPU::default();
        cpu.pc = 0x0301;
        cpu.registers.sp = 0xFFFE;
        cpu.bus.memory[cpu.pc as usize] = 0xCD;
        cpu.bus.memory[cpu.pc as usize + 1] = 0xAB;
        call_a16()(&mut cpu);
        assert_eq!(cpu.pc, 0xABCD);
        assert_eq!(cpu.registers.sp, 0xFFFC);
        assert_eq!(cpu.bus.memory[0xFFFD], 0x03);
        assert_eq!(cpu.bus.memory[0xFFFC], 0x01);
    }
}
