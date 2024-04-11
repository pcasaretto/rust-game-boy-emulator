use crate::cpu::CPU;

pub fn stop() -> impl Fn(&mut CPU) {
    //TODO: stop until button pressed
    move |_: &mut CPU| {}
}

#[cfg(test)]
mod tests {}
