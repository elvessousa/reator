use crate::file::ReactFile;
use crate::messages::Message::{self, *};
use crate::validation::Validation;
use std::{error::Error, process};

type CommandResult = Result<(), Box<dyn Error>>;

#[derive(PartialEq, Debug)]
pub enum Commands {
    New,
    Help,
    Invalid,
}

impl Commands {
    pub fn from(command: &str) -> Self {
        match command {
            "create" | "new" | "n" => Self::New,
            "help" | "h" => Self::Help,
            _ => Self::Invalid,
        }
    }

    pub fn parse(&self, template: &str, path: &str, option: &str) -> CommandResult {
        Validation.project()?;

        let template = Validation.template(template).unwrap();
        let option = Validation.styling(&template, &option, true);

        ReactFile
            .create(&template, path, option)
            .unwrap_or_else(|error| {
                Message::print(ErrorMsg(error));
                process::exit(1);
            });

        Ok(())
    }
}

#[derive(PartialEq, Debug)]
pub enum Options {
    ReactNativeStyle,
    CSSModule,
    SassModule,
}

impl Options {
    pub fn from(option: &str) -> Option<Self> {
        match option {
            "--css-module" | "-css" => Some(Self::CSSModule),
            "--reactnative-style" | "-rns" => Some(Self::ReactNativeStyle),
            "--sass-module" | "-sass" => Some(Self::SassModule),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_only_available_commands() {
        let new1 = Commands::from("new");
        let new2 = Commands::from("create");
        let new3 = Commands::from("n");
        assert_eq!(new1, Commands::New);
        assert_eq!(new1, new2);
        assert_eq!(new2, new3);

        let help1 = Commands::from("help");
        let help2 = Commands::from("h");
        assert_eq!(help1, Commands::Help);
        assert_eq!(help1, help2);

        let invalid = Commands::from("wow");
        assert_eq!(invalid, Commands::Invalid);
    }

    #[test]
    fn returns_only_available_options() {
        let css1 = Options::from("--css-module");
        let css2 = Options::from("-css");
        assert_eq!(css1, Some(Options::CSSModule));
        assert_eq!(css1, css2);

        let sass1 = Options::from("--sass-module");
        let sass2 = Options::from("-sass");
        assert_eq!(sass1, Some(Options::SassModule));
        assert_eq!(sass1, sass2);
    }
}
