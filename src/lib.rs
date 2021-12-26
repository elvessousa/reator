mod file;
mod template;
mod utils;

use file::ReactFile;
use std::env;
use std::error::Error;
use template::Template;
use utils::Message;

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
            utils::help();
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
                    return Err("No template informed.");
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

#[derive(Debug)]
pub enum Commands {
    New,
    Help,
}

impl Commands {
    fn from(command: &str) -> Option<Self> {
        match command {
            "create" | "new" | "n" => Some(Commands::New),
            "help" | "h" => Some(Commands::Help),
            _ => None,
        }
    }

    fn parse(template: &str, path: &str) {
        let file = ReactFile;

        match Template::from(template) {
            Some(_) => file.create(&template, path).unwrap_or_else(|error| {
                Message::print(Message::ErrorMsg, format!("{}", error).as_str());
                std::process::exit(1);
            }),
            None => println!("Invalid template"),
        }
    }
}

pub fn run(args: Arguments) -> Result<(), Box<dyn Error>> {
    let command = args.command;
    let template = args.template;
    let path = args.path;
    // let option = args.option;

    match Commands::from(&command) {
        Some(Commands::New) => Commands::parse(&template, &path),
        Some(Commands::Help) => utils::help(),
        None => println!("No valid command found"),
    }

    /* println!(
        "Command: {} / Template: {} / Path: {} / Option: {}",
        command, template, path, option
    ); */

    Ok(())
}
