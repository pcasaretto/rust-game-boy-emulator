use super::super::*;

pub fn nop() -> impl Fn(&mut CPU) {
    move |cpu: &mut CPU| {
        cpu.pc += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nop() {
        let mut cpu = CPU::default();
        nop()(&mut cpu);
        assert_eq!(cpu.pc, 1);
    }
}
