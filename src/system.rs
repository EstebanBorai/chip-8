use anyhow::Error;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::{EventPump, Sdl};
use std::fs;

use crate::config::Config;
use crate::cpu::Cpu;
use crate::display::Display;

pub struct System {
    config: Config,
    cpu: Cpu,
    display: Display,
    event_pump: EventPump,
    sdl: Sdl,
}

impl System {
    pub fn new(config: Config) -> Self {
        let mut cpu = Cpu::new();
        let sdl = sdl2::init().unwrap();
        let event_pump = sdl.event_pump().unwrap();
        let display = Display::new(&sdl, "Chip8", 2);
        let bytes = fs::read(&config.rom)
            .map_err(|err| Error::msg(err.to_string()))
            .expect("Failed to read ROM.");

        cpu.load(&bytes);

        Self {
            config,
            cpu,
            display,
            event_pump,
            sdl,
        }
    }

    pub fn start(self) {
        if self.config.debug {
            self.start_debugging();
        } else {
            self.start_not_debugging();
        }

        println!("Chip8 Exiting");
    }

    fn start_not_debugging(mut self) {
        'chip8: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        break 'chip8;
                    }
                    _ => {}
                }
            }

            self.cpu.cycle();
            self.display.render(&self.cpu.display_buffer);
        }
    }

    fn start_debugging(mut self) {
        'chip8: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        break 'chip8;
                    }
                    Event::KeyDown {
                        keycode: Some(Keycode::Space),
                        ..
                    } => {
                        self.cpu.cycle();
                        self.display.render(&self.cpu.display_buffer);
                    }
                    _ => {}
                }
            }
        }
    }
}
