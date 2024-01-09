use std::{env, process};

use minigrep::{run, Config};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments:{err}");
        process::exit(1);
    });

    if let Result::Err(v) = run(config) {
        eprintln!("Application Error:{v}");
        process::exit(1)
    }
}
