use std::{env, process};

use minigrep::{run, Config};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments:{err}");
        process::exit(1);
    });

    if let Result::Err(v) = run(config) {
        println!("Application Error:{v}");
        process::exit(1)
    }
}
