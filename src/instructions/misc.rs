use crate::gameboy::Gameboy;

pub fn stop() -> impl Fn(&mut Gameboy) {
    //TODO: stop until button pressed
    move |_: &mut Gameboy| {}
}

#[cfg(test)]
mod tests {}
