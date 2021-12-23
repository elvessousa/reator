mod files;
mod utils;

use std::env;
use std::error::Error;

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
            None => return Err("No template informed."),
        };

        let path = match args.next() {
            Some(arg) => arg,
            None => return Err("No path informed."),
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
            "help" | "h" | "?" => Some(Commands::Help),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum Templates {
    Component,
    Context,
    NextPage,
    NextDoc,
    Style,
    Styled,
}

impl Templates {
    fn from(template: &str) -> Option<Self> {
        match template {
            "component" | "rc" => Some(Templates::Component),
            "context" | "ct" => Some(Templates::Context),
            "next-page" | "np" => Some(Templates::NextPage),
            "next-doc" | "nd" => Some(Templates::NextDoc),
            "style" | "s" => Some(Templates::Style),
            "styled" | "sc" => Some(Templates::Component),
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
    match Templates::from(template) {
        Some(_) => files::create(&template, path).unwrap_or_else(|error| {
            eprintln!("Error! {}", error);
            std::process::exit(1);
        }),
        None => println!("Invalid template"),
    }
}
