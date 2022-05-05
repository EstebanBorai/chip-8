use ch8::config::Config;
use ch8::system::System;
use structopt::StructOpt;

fn main() {
    let config = Config::from_args();
    let system = System::new(config);

    system.start();
}
