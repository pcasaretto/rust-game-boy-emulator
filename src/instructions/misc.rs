use crate::gameboy::Gameboy;

const TICKS: u8 = 4;
pub fn stop(gameboy: &mut Gameboy) -> u8 {
    //TODO: stop until button pressed
    TICKS
}

#[cfg(test)]
mod tests {}
