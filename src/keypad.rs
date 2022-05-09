use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;
use std::fmt;
use std::ops::{Index, IndexMut};

/// COSMAC VIP Keypad implementation mapped from modern PC's.
///
/// Mapping is achieved as follows:
///
/// COSMAC VIP Keypad Positions
///
/// 1 2 3 C
/// 4 5 6 D
/// 7 8 9 E
/// A 0 B F
///
/// Modern PC's Equivalent
///
/// 1 2 3 4
/// Q W E R
/// A S D F
/// Z X C V
///
/// Keypad implementation uses scancodes instead of string constants to bring
/// support for different keyboard layouts.
pub struct Keypad {
    event_pump: EventPump,
}

/// For each of the 16 keys available, the state (pressed/not-pressed) is kept
/// in a 16-bit array.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct KeypadState([bool; 16]);

impl KeypadState {
    pub fn new() -> Self {
        KeypadState([false; 16])
    }
}

impl fmt::Display for KeypadState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let arr = self.0;

        write!(
            f,
            "1:{} 2:{} 3:{} 4:{} Q:{} W:{} E:{} R:{} A:{} S:{} D:{} F:{} Z:{} X:{} C:{} V:{}",
            arr[0x1] as u8,
            arr[0x2] as u8,
            arr[0x3] as u8,
            arr[0xC] as u8,
            arr[0x4] as u8,
            arr[0x5] as u8,
            arr[0x6] as u8,
            arr[0xD] as u8,
            arr[0x7] as u8,
            arr[0x8] as u8,
            arr[0x9] as u8,
            arr[0xE] as u8,
            arr[0xA] as u8,
            arr[0x0] as u8,
            arr[0xB] as u8,
            arr[0xF] as u8,
        )
    }
}

impl Index<usize> for KeypadState {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for KeypadState {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Keypad {
    /// Creates a new Keypad and polls events from Sdl2's `EventPump`.
    pub fn new(event_pump: EventPump) -> Self {
        Self { event_pump }
    }

    pub fn poll(&mut self) -> Result<KeypadState, ()> {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    panic!("QUIT");
                }
                _ => {}
            }
        }

        Ok(self.pressed_keys())
    }

    /// Retrieve pressed keys from Event Pump which matches any of the
    /// COSMAC VIP keys.
    fn pressed_keys(&self) -> KeypadState {
        let mut keypad_state = KeypadState::new();

        self.event_pump
            .keyboard_state()
            .pressed_scancodes()
            .for_each(|scancode| {
                if let Some(keycode) = Keycode::from_scancode(scancode) {
                    match keycode {
                        Keycode::Num1 => keypad_state[0x1] = true,
                        Keycode::Num2 => keypad_state[0x2] = true,
                        Keycode::Num3 => keypad_state[0x3] = true,
                        Keycode::Num4 => keypad_state[0xC] = true,
                        Keycode::Q => keypad_state[0x4] = true,
                        Keycode::W => keypad_state[0x5] = true,
                        Keycode::E => keypad_state[0x6] = true,
                        Keycode::R => keypad_state[0xD] = true,
                        Keycode::A => keypad_state[0x7] = true,
                        Keycode::S => keypad_state[0x8] = true,
                        Keycode::D => keypad_state[0x9] = true,
                        Keycode::F => keypad_state[0xE] = true,
                        Keycode::Z => keypad_state[0xA] = true,
                        Keycode::X => keypad_state[0x0] = true,
                        Keycode::C => keypad_state[0xB] = true,
                        Keycode::V => keypad_state[0xF] = true,
                        _ => {}
                    }
                }
            });

        keypad_state
    }
}
