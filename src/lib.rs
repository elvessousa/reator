pub mod commands;
pub mod file;
pub mod messages;
pub mod template;
pub mod validation;

use commands::Commands;
use messages::warnings;
use messages::Message::{self, *};
use std::{env, error::Error, path::Path};

#[derive(Debug)]
pub struct Arguments {
    pub command: String,
    pub template: String,
    pub path: String,
    pub option: String,
}

impl Arguments {
    pub fn new(mut args: env::Args) -> Result<Arguments, &'static str> {
        if args.len() < 1 {
            messages::help();
            return Err("Not enough arguments");
        }

        let command = match args.nth(1) {
            Some(arg) => arg,
            None => return Err("No operation informed."),
        };

        let template = match args.next() {
            Some(arg) => arg,
            None => {
                if command.contains("h") {
                    "".to_owned()
                } else {
                    return Err("No template informed.");
                }
            }
        };

        let path = match args.next() {
            Some(arg) => arg,
            None => {
                if command.contains("h") {
                    "".to_owned()
                } else {
                    return Err("No component name informed.");
                }
            }
        };

        let option = match args.next() {
            Some(arg) => arg,
            None => "".to_owned(),
        };

        Ok(Arguments {
            command,
            template,
            path,
            option,
        })
    }
}

pub fn run(args: Arguments) -> Result<(), Box<dyn Error>> {
    Message::print(AboutMsg);

    let command = Commands::from(&args.command);
    let template = args.template;
    let path = args.path;
    let option = args.option;

    if Path::new("./tsconfig.json").exists() {
        Message::print(FoundMsg("tsconfig.json"));
        Message::print(InfoMsg(warnings::INFO_TS));
    }

    match command {
        Commands::New => {
            command.parse(&template, &path, &option)?;
            Message::print(SuccessMsg("Done!"))
        }
        Commands::Help => messages::help(),
        _ => Message::print(MistakeMsg(format!("Invalid command: '{}'", &args.command))),
    }

    Ok(())
}
