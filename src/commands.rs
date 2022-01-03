use crate::file::ReactFile;
use crate::messages::Message::{self, *};
use crate::validation::Validation;
use std::{error::Error, process};

type CommandResult = Result<(), Box<dyn Error>>;

#[derive(Debug)]
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

#[derive(Debug)]
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
