use chip8::config::Config;
use chip8::cpu::Cpu;
use chip8::display::Display;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use structopt::StructOpt;

fn main() {
    let config = Config::from_args();
    let mut cpu = Cpu::try_from(config).unwrap();
    let sdl_context = sdl2::init().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut display = Display::new(&sdl_context, "Chip8", 2);

    'chip8: loop {
        for event in event_pump.poll_iter() {
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

        cpu.cycle();
        display.render(&cpu.display_buffer());
    }

    println!("Chip8 Interpreter Closed");
}
