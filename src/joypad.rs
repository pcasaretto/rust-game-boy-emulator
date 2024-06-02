pub enum JoypadButton {
    Right,
    Left,
    Up,
    Down,
    A,
    B,
    Select,
    Start,
}

pub struct Joypad {
    pub directional_keys: u8,
    pub standard_buttons: u8,
}

impl Joypad {
    pub fn new() -> Self {
        Joypad {
            directional_keys: 0x0F,
            standard_buttons: 0x0F,
        }
    }

    pub fn get_button_state(&self, button: &JoypadButton) -> bool {
        match button {
            JoypadButton::Right => self.directional_keys & 0x01 == 0,
            JoypadButton::Left => self.directional_keys & 0x02 == 0,
            JoypadButton::Up => self.directional_keys & 0x04 == 0,
            JoypadButton::Down => self.directional_keys & 0x08 == 0,
            JoypadButton::A => self.standard_buttons & 0x01 == 0,
            JoypadButton::B => self.standard_buttons & 0x02 == 0,
            JoypadButton::Select => self.standard_buttons & 0x04 == 0,
            JoypadButton::Start => self.standard_buttons & 0x08 == 0,
        }
    }

    pub fn set_button_state(&mut self, button: &JoypadButton, state: bool) -> bool {
        let current_state = self.get_button_state(button);
        if state {
            self.press_button(button)
        } else {
            self.release_button(button)
        };
        current_state != state
    }

    fn press_button(&mut self, button: &JoypadButton) {
        match button {
            JoypadButton::Right => self.directional_keys &= 0x0E,
            JoypadButton::Left => self.directional_keys &= 0x0D,
            JoypadButton::Up => self.directional_keys &= 0x0B,
            JoypadButton::Down => self.directional_keys &= 0x07,
            JoypadButton::A => self.standard_buttons &= 0x0E,
            JoypadButton::B => self.standard_buttons &= 0x0D,
            JoypadButton::Select => self.standard_buttons &= 0x0B,
            JoypadButton::Start => self.standard_buttons &= 0x07,
        }
    }

    fn release_button(&mut self, button: &JoypadButton) {
        match button {
            JoypadButton::Right => self.directional_keys |= 0x01,
            JoypadButton::Left => self.directional_keys |= 0x02,
            JoypadButton::Up => self.directional_keys |= 0x04,
            JoypadButton::Down => self.directional_keys |= 0x08,
            JoypadButton::A => self.standard_buttons |= 0x01,
            JoypadButton::B => self.standard_buttons |= 0x02,
            JoypadButton::Select => self.standard_buttons |= 0x04,
            JoypadButton::Start => self.standard_buttons |= 0x08,
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_directional_buttons() {
        macro_rules! test_directional_button {
            ($button:expr, $directional_key:expr) => {
                let mut joypad = Joypad::new();
                assert_eq!(joypad.get_button_state(&$button), false);

                joypad.set_button_state($button, true);
                assert_eq!(joypad.get_button_state(&$button), true);
                assert_eq!(joypad.directional_keys, $directional_key);

                joypad.set_button_state($button, false);
                assert_eq!(joypad.get_button_state(&$button), false);
                assert_eq!(joypad.directional_keys, 0x0F);
            };
        }

        test_directional_button!(&JoypadButton::Right, 0b00001110);
        test_directional_button!(&JoypadButton::Left, 0b00001101);
        test_directional_button!(&JoypadButton::Up, 0b00001011);
        test_directional_button!(&JoypadButton::Down, 0b00000111);
    }

    #[test]
    fn test_standard_buttons() {
        macro_rules! test_standard_button {
            ($button:expr, $standard_key:expr) => {
                let mut joypad = Joypad::new();
                assert_eq!(joypad.get_button_state(&$button), false);

                joypad.set_button_state($button, true);
                assert_eq!(joypad.get_button_state(&$button), true);
                assert_eq!(joypad.standard_buttons, $standard_key);

                joypad.set_button_state($button, false);
                assert_eq!(joypad.get_button_state(&$button), false);
                assert_eq!(joypad.standard_buttons, 0x0F);
            };
        }

        test_standard_button!(&JoypadButton::A, 0b00001110);
        test_standard_button!(&JoypadButton::B, 0b00001101);
        test_standard_button!(&JoypadButton::Select, 0b00001011);
        test_standard_button!(&JoypadButton::Start, 0b00000111);
    }
}
