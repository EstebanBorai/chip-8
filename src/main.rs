use chip8::config::Config;
use chip8::cpu::Cpu;
use structopt::StructOpt;

fn main() {
    let config = Config::from_args();
    let mut cpu = Cpu::try_from(config).unwrap();

    cpu.cycle();
}
