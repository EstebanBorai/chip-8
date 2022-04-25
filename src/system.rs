use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

use crate::config::Config;
use crate::cpu::Cpu;
use crate::display::Display;
use crate::keypad::Keypad;
use crate::rom::Rom;

pub struct System {
    config: Config,
    cpu: Cpu,
    display: Display,
    keypad: Keypad,
}

impl System {
    pub fn new(config: Config) -> Self {
        let mut cpu = Cpu::new();
        let sdl = sdl2::init().unwrap();
        let event_pump = sdl.event_pump().unwrap();
        let display = Display::new(&sdl, "Chip8", 12);
        let keypad = Keypad::new(event_pump);
        let rom = Rom::from_path(&config.rom);

        cpu.load(rom);

        Self {
            config,
            cpu,
            display,
            keypad,
        }
    }

    pub fn start(mut self) {
        while let Ok(pressed_keys) = self.keypad.poll() {
            let cycle_output = self.cpu.cycle(pressed_keys);

            if cycle_output.display_update {
                self.display.render(&cycle_output.display_buffer);
            }

            std::thread::sleep(std::time::Duration::from_millis(2));
        }
    }
}
