use crate::gameboy::Gameboy;

pub fn di() -> impl Fn(&mut Gameboy) {
    move |_cpu: &mut Gameboy| {
        // cpu.interrupts_enabled = false;
    }
}

pub fn ei() -> impl Fn(&mut Gameboy) {
    move |_cpu: &mut Gameboy| {
        // cpu.interrupts_enabled = false;
    }
}
