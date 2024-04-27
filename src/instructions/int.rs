use crate::gameboy::Gameboy;

pub fn di(gameboy: &mut Gameboy) -> u8 {
    gameboy.interrupts_enabled = false;
    const TICKS: u8 = 4;
    TICKS
}

pub fn ei(gameboy: &mut Gameboy) -> u8 {
    // TODO: delay enabling interrupts by one cycle
    gameboy.interrupts_enabled = true;
    const TICKS: u8 = 4;
    TICKS
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_di() {
        let mut gameboy = Gameboy::default();
        gameboy.interrupts_enabled = true;

        di(&mut gameboy);

        assert_eq!(gameboy.interrupts_enabled, false);
    }

    #[test]
    fn test_ei() {
        let mut gameboy = Gameboy::default();
        gameboy.interrupts_enabled = false;

        ei(&mut gameboy);

        assert_eq!(gameboy.interrupts_enabled, true);
    }
}
