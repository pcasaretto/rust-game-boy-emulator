use crate::cpu::{RegisterTarget, CPU};

pub fn operation_on_r_a(
    target: RegisterTarget,
    operation: fn(left: u8, right: u8) -> u8,
) -> impl Fn(&mut CPU) {
    move |cpu: &mut CPU| {
        let r = cpu.registers.get_u8(target);
        let a = cpu.registers.get_u8(RegisterTarget::A);

        let value = operation(a, r);

        cpu.registers.set_u8(RegisterTarget::A, value);

        cpu.registers.f.zero = value == 0;
        cpu.registers.f.subtract = false;
        cpu.registers.f.half_carry = true;
        cpu.registers.f.carry = false;
    }
}
