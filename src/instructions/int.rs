use crate::CPU;

pub fn di() -> impl Fn(&mut CPU) {
    move |cpu: &mut CPU| {
        // cpu.interrupts_enabled = false;
    }
}
