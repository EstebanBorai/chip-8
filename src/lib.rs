#[macro_use]
extern crate log;

pub mod cpu;
pub mod memory;
pub mod stack;

use std::env;
use std::fs;

use crate::cpu::Cpu;

/// Executes the CHIP-8 emulator with command line arguments provided.
pub fn run_from_args() {
    #[cfg(debug_assertions)]
    env_logger::init();

    let args = env::args().collect::<Vec<String>>();
    let rom_path = &args[1];
    let mut cpu = Cpu::new();
    let bytes = fs::read(rom_path).expect("Failed to find ROM");

    cpu.load(&bytes);
    cpu.run_cycle();
}
