mod file;
mod messages;
mod template;

use file::ReactFile;
use messages::Message;
use std::{env, error::Error};
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

                    Message::print(Message::ErrorMsg, error_msg.as_str());
                    std::process::exit(1);
                }),
            None => println!("Invalid template"),
        }
    }
}

#[derive(Debug)]
pub enum Options {
    WithStyle,
    CSSModule,
    SassModule,
    InterfaceProps,
    TypeProps,
}

impl Options {
    pub fn from(option: &str) -> Option<Self> {
        match option {
            "--with-style" | "-ws" => Some(Self::WithStyle),
            "--css" => Some(Self::CSSModule),
            "--sass" => Some(Self::SassModule),
            "--iprops" | "-i" => Some(Self::InterfaceProps),
            "--tprops" | "-t" => Some(Self::TypeProps),
            _ => None,
        }
    }
}

pub fn run(args: Arguments) -> Result<(), Box<dyn Error>> {
    let command = args.command;
    let template = args.template;
    let path = args.path;
    let option = args.option;

    Message::print(Message::AboutMsg, "");

    match Commands::from(&command) {
        Some(Commands::New) => Commands::parse(&template, &path, &option),
        Some(Commands::Help) => messages::help(),
        None => println!("No valid command found"),
    }

    println!("\n Done!\n");

    Ok(())
}
