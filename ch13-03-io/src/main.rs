use std::env;
use std::process;

use ch13_03_io;

fn main() {
    let config = ch13_03_io::Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = ch13_03_io::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    };
}
