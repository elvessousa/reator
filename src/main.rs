use reator::Arguments;
use std::{env, process};

fn main() {
    let args = Arguments::new(env::args()).unwrap_or_else(|error| {
        eprintln!("Argument parsing failed: {}", error);
        process::exit(1);
    });

    reator::run(args).unwrap();
}
