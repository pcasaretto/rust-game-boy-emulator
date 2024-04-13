use crate::gameboy::Gameboy;

const TICKS: u8 = 4;
pub fn nop(_: &mut Gameboy) -> u8 {
    TICKS
}

#[cfg(test)]
mod tests {}
