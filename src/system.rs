use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

use crate::config::Config;
use crate::cpu::Cpu;
use crate::display::Display;
use crate::rom::Rom;

pub struct System {
    config: Config,
    cpu: Cpu,
    display: Display,
    event_pump: EventPump,
}

impl System {
    pub fn new(config: Config) -> Self {
        let mut cpu = Cpu::new();
        let sdl = sdl2::init().unwrap();
        let event_pump = sdl.event_pump().unwrap();
        let display = Display::new(&sdl, "Chip8", 12);
        let rom = Rom::from_path(&config.rom);

        cpu.load(rom);

        Self {
            config,
            cpu,
            display,
            event_pump,
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

            let output = self.cpu.cycle();

            if output.display_update {
                self.display.render(&output.display_buffer);
            }

            std::thread::sleep(std::time::Duration::from_millis(2));
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

            let output = self.cpu.cycle();

            if output.display_update {
                self.display.render(&output.display_buffer);
            }

            std::thread::sleep(std::time::Duration::from_millis(2));
        }
    }
}
