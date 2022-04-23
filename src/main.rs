use chip8::config::Config;
use chip8::cpu::Cpu;
use chip8::system::System;
use structopt::StructOpt;

fn main() {
    let config = Config::from_args();
    let mut system = System::new(config);

    system.start();
}
