mod file;
mod messages;
mod template;

use file::ReactFile;
use messages::warnings;
use messages::Message::{self, *};
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

    fn parse(&self, template: &str, path: &str, option: &str) {
        let option = self.validate_styling_options(template, &option);

        match Template::from(template) {
            Some(_) => ReactFile
                .create(&template, path, option)
                .unwrap_or_else(|error| {
                    let error_msg = format!("{}", error);

                    Message::print(ErrorMsg, error_msg.as_str());
                    std::process::exit(1);
                }),
            None => println!("Invalid template"),
        }
    }

    fn validate_styling_options(&self, template: &str, option: &str) -> Option<Options> {
        let option = Options::from(option);

        match Template::from(template).unwrap() {
            Template::NativeComponent | Template::NativeCompoundComponent => {
                Message::print(WarningMsg, warnings::WARN_INVALID_RN_STYLE);
                Message::print(InfoMsg, warnings::INFO_NEVER_HAPPEN);
                None
            }
            Template::Context => {
                Message::print(WarningMsg, warnings::WARN_CONTEXT_STYLING);
                Message::print(InfoMsg, warnings::INFO_NEVER_HAPPEN);
                None
            }
            Template::SassModule | Template::RNStyle | Template::Styled => {
                Message::print(WarningMsg, warnings::WARN_INVALID_STYLE);
                Message::print(InfoMsg, warnings::INFO_NEVER_HAPPEN);
                None
            }
            Template::NextDoc => {
                Message::print(WarningMsg, warnings::WARN_NDOC_STYLING);
                Message::print(InfoMsg, warnings::INFO_NEVER_HAPPEN);
                None
            }
            _ => option,
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
            "--css-module" | "-css" => Some(Self::CSSModule),
            "--sass-module" | "-sass" => Some(Self::SassModule),
            _ => None,
        }
    }
}

pub fn run(args: Arguments) -> Result<(), Box<dyn Error>> {
    Message::print(AboutMsg, "");

    validate_project()?;

    let command = Commands::from(&args.command).unwrap();
    let template = args.template;
    let path = args.path;
    let option = args.option;

    if Path::new("./tsconfig.json").exists() {
        Message::print(FoundMsg, "tsconfig.json");
        Message::print(InfoMsg, warnings::INFO_TS);
    }

    match command {
        Commands::New => command.parse(&template, &path, &option),
        Commands::Help => messages::help(),
    }

    println!("\n Done!\n");

    Ok(())
}

pub fn validate_project() -> Result<(), Box<dyn Error>> {
    let project_file = Path::new("./package.json");

    if project_file.exists() {
        Ok(())
    } else {
        Message::print(NotFoundMsg, "package.json");
        Message::print(InfoMsg, warnings::INFO_PROJECT_ROOT);
        std::process::exit(1);
    }
}
