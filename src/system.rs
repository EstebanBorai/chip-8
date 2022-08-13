use std::io::{stdin, stdout, Read, Write};

use crate::audio::Audio;
use crate::config::Config;
use crate::cpu::Cpu;
use crate::display::Display;
use crate::keypad::Keypad;
use crate::memory::MEMORY_SIZE;
use crate::rom::Rom;

pub struct System {
    audio: Audio,
    #[allow(dead_code)]
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
        let audio = Audio::new(&sdl);
        let display = Display::new(&sdl, "Chip8", 12);
        let keypad = Keypad::new(event_pump);
        let rom = Rom::from_path(&config.rom);

        cpu.load(rom);

        Self {
            audio,
            config,
            cpu,
            display,
            keypad,
        }
    }

    pub fn start(mut self) {
        while let Ok(pressed_keys) = self.keypad.poll() {
            if self.cpu.pc as usize >= MEMORY_SIZE {
                panic!("EOF");
            }

            let cycle_output = self.cpu.cycle(pressed_keys);

            if cycle_output.display_update {
                self.display.render(&cycle_output.display_buffer);
            }

            if cycle_output.beep {
                self.audio.play();
            } else {
                self.audio.stop();
            }

            if self.config.debug {
                let mut stdout = stdout();

                stdout
                    .write(b"Debugging Mode. Press ENTER to run next cycle.")
                    .expect("Failed to write to stdout.");
                stdout.flush().expect("Failed to flush stdout.");
                stdin().read(&mut [0]).expect("Failed to read from stdin.");
            } else {
                std::thread::sleep(std::time::Duration::from_millis(2));
            }
        }
    }
}
