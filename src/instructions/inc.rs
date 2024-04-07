use crate::CPU;

pub fn inc_sp() -> impl Fn(&mut CPU) {
    move |cpu: &mut CPU| {
        cpu.pc = cpu.pc.wrapping_add(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inc_sp() {
        let mut cpu = CPU {
            pc: 0xF012,
            ..Default::default()
        };
        inc_sp()(&mut cpu);
        assert_eq!(cpu.pc, 0xF013);
    }

    #[test]
    fn test_inc_sp_wrap() {
        let mut cpu = CPU {
            pc: 0xFFFF,
            ..Default::default()
        };
        inc_sp()(&mut cpu);
        assert_eq!(cpu.pc, 0x0000);
    }
}
