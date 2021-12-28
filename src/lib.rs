mod file;
mod messages;
mod template;

use file::ReactFile;
use messages::Message;
use std::{env, error::Error, path::Path};
use template::Template;

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
            "create" | "new" | "n" => Some(Self::New),
            "help" | "h" => Some(Self::Help),
            _ => None,
        }
    }

    fn parse(template: &str, path: &str, option: &str) {
        let file = ReactFile;
        let option = Options::from(option);

        match Template::from(template) {
            Some(_) => file
                .create(&template, path, option)
                .unwrap_or_else(|error| {
                    let error_msg = format!("{}", error);

                    Message::print(Message::Error, error_msg.as_str());
                    std::process::exit(1);
                }),
            None => println!("Invalid template"),
        }
    }
}

#[derive(Debug)]
pub enum Options {
    ReactNativeStyle,
    CSSModule,
    SassModule,
}

impl Options {
    pub fn from(option: &str) -> Option<Self> {
        match option {
            "--reactnative-style" | "-rns" => Some(Self::ReactNativeStyle),
            "--css" => Some(Self::CSSModule),
            "--sass" => Some(Self::SassModule),
            _ => None,
        }
    }
}

pub fn run(args: Arguments) -> Result<(), Box<dyn Error>> {
    let command = args.command;
    let template = args.template;
    let path = args.path;
    let option = args.option;

    Message::print(Message::About, "");

    validate_project()?;

    if Path::new("./tsconfig.json").exists() {
        Message::print(Message::Found, "tsconfig.json");
        Message::print(Message::Info, "You're using TypeScript.");
    }

    match Commands::from(&command) {
        Some(Commands::New) => Commands::parse(&template, &path, &option),
        Some(Commands::Help) => messages::help(),
        None => println!("No valid command found"),
    }

    println!("\n Done!\n");

    Ok(())
}

pub fn validate_project() -> Result<(), Box<dyn Error>> {
    let project_file = Path::new("./package.json");
    if project_file.exists() {
        Ok(())
    } else {
        Message::print(Message::NotFound, "package.json");
        Message::print(Message::Info, "Make sure you are on a valid react project, and running this tool from its root directory.");
        std::process::exit(1);
    }
}
