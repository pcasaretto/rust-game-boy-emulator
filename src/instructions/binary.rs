use crate::cpu::RegisterTarget;
use crate::gameboy::Gameboy;

pub fn operation_on_r_a(
    target: RegisterTarget,
    operation: fn(left: u8, right: u8) -> u8,
) -> impl Fn(&mut Gameboy) {
    move |gameboy: &mut Gameboy| {
        let r = gameboy.cpu.registers.get_u8(target);
        let a = gameboy.cpu.registers.get_u8(RegisterTarget::A);

        let value = operation(a, r);

        gameboy.cpu.registers.set_u8(RegisterTarget::A, value);

        gameboy.cpu.registers.f.zero = value == 0;
        gameboy.cpu.registers.f.subtract = false;
        gameboy.cpu.registers.f.half_carry = true;
        gameboy.cpu.registers.f.carry = false;
    }
}
