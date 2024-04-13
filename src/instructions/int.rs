use crate::gameboy::Gameboy;

pub fn di() -> impl Fn(&mut Gameboy) {
    move |gameboy: &mut Gameboy| {
        gameboy.interrupts_enabled = false;
    }
}

pub fn ei() -> impl Fn(&mut Gameboy) {
    move |gameboy: &mut Gameboy| {
        gameboy.interrupts_enabled = true;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_di() {
        let mut gameboy = Gameboy::default();
        gameboy.interrupts_enabled = true;

        let di_func = di();
        di_func(&mut gameboy);

        assert_eq!(gameboy.interrupts_enabled, false);
    }

    #[test]
    fn test_ei() {
        let mut gameboy = Gameboy::default();
        gameboy.interrupts_enabled = false;

        let ei_func = ei();
        ei_func(&mut gameboy);

        assert_eq!(gameboy.interrupts_enabled, true);
    }
}
