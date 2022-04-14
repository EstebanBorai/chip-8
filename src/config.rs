use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt, PartialEq, Eq)]
#[structopt(
    name = "chip8",
    author = "Esteban Borai <estebanborai@gmail.com>",
    about = "CHIP-8 Emulator"
)]
pub struct Config {
    /// ROM file to load
    #[structopt(parse(from_os_str))]
    pub rom: PathBuf,
    /// Debug mode
    #[structopt(short = "d", long = "debug")]
    pub debug: bool,
    /// Inspect instructions from ROM
    #[structopt(short = "i", long = "inspect")]
    pub inspect: bool,
}
