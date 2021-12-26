mod file;
mod template;
mod utils;

use file::ReactFile;
use std::env;
use std::error::Error;
use template::Template;

#[derive(Debug)]
pub struct Arguments {
    pub command: String,
    pub template: String,
    pub path: String,
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
                    "".to_string()
                } else {
                    return Err("No template informed.");
                }
            }
        };

        let path = match args.next() {
            Some(arg) => arg,
            None => {
                if command.contains("h") {
                    "".to_string()
                } else {
                    return Err("No template informed.");
                }
            }
        };

        Ok(Arguments {
            command,
            template,
            path,
        })
    }
}

#[derive(Debug)]
pub enum Commands {
    New,
    Delete,
    Help,
}

impl Commands {
    fn from(command: &str) -> Option<Self> {
        match command {
            "create" | "new" | "n" => Some(Commands::New),
            "delete" | "del" | "d" => Some(Commands::Delete),
            "help" | "h" => Some(Commands::Help),
            _ => None,
        }
    }
}

pub fn run(args: Arguments) -> Result<(), Box<dyn Error>> {
    let command = args.command;
    let template = args.template;
    let path = args.path;

    match Commands::from(&command) {
        Some(Commands::New) => parse_command(&template, &path),
        Some(Commands::Delete) => println!("File deleted"),
        Some(Commands::Help) => utils::help(),
        None => println!("No valid command found"),
    }

    println!(
        "Command: {} / Template: {} / Path: {}",
        command, template, path
    );

    Ok(())
}

fn parse_command<'a>(template: &'a String, path: &String) {
    let file = ReactFile;

    match Template::from(template) {
        Some(_) => file.create(&template, path).unwrap_or_else(|error| {
            eprintln!("Error! {}", error);
            std::process::exit(1);
        }),
        None => println!("Invalid template"),
    }
}
