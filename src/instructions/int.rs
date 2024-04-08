use crate::CPU;

pub fn di() -> impl Fn(&mut CPU) {
    move |_cpu: &mut CPU| {
        // cpu.interrupts_enabled = false;
    }
}

pub fn ei() -> impl Fn(&mut CPU) {
    move |_cpu: &mut CPU| {
        // cpu.interrupts_enabled = false;
    }
}
