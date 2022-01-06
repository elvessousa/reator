use reator::messages::Message::{self, *};
use reator::Arguments;

use std::{env, process};

fn main() {
    let args = Arguments::new(env::args()).unwrap_or_else(|error| {
        Message::print(AboutMsg);
        Message::print(MistakeMsg(error.to_owned()));
        process::exit(1);
    });

    reator::run(args).unwrap();
}
