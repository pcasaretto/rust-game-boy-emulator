use super::super::*;

impl CPU {
    pub fn nop(&mut self) {
        self.pc += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nop() {
        let mut cpu = CPU::default();
        cpu.execute(Instruction::NOP);
        assert_eq!(cpu.pc, 1);
    }
}
