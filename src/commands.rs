use crate::file::ReactFile;
use crate::messages::warnings;
use crate::messages::Message::{self, *};
use crate::template::Template;
use std::{error::Error, path::Path, process};

#[derive(Debug)]
pub enum Commands {
    New,
    Help,
    Invalid,
}

type CommandResult<T> = Result<T, Box<dyn Error>>;

impl Commands {
    pub fn from(command: &str) -> Self {
        match command {
            "create" | "new" | "n" => Self::New,
            "help" | "h" => Self::Help,
            _ => Self::Invalid,
        }
    }

    pub fn parse(&self, template: &str, path: &str, option: &str) -> CommandResult<()> {
        self.validate_project()?;
        let template = self.validate_template(template).unwrap();
        let option = self.validate_styling_options(&template, &option);

        ReactFile
            .create(&template, path, option)
            .unwrap_or_else(|error| {
                Message::print(ErrorMsg(error));
                process::exit(1);
            });

        Ok(())
    }

    fn validate_project(&self) -> CommandResult<()> {
        if Path::new("./package.json").exists() {
            Ok(())
        } else {
            Message::print(NotFoundMsg("package.json"));
            Message::print(InfoMsg(warnings::INFO_PROJECT_ROOT));
            process::exit(1);
        }
    }

    fn validate_template(&self, template: &str) -> CommandResult<Template> {
        Ok(Template::from(template).unwrap_or_else(|| {
            Message::print(MistakeMsg(format!("Invalid template: '{}'", template)));
            process::exit(1);
        }))
    }

    fn validate_styling_options(&self, template: &Template, option: &str) -> Option<Options> {
        let option = Options::from(option);

        match option {
            Some(Options::ReactNativeStyle) => match template {
                Template::NativeComponent | Template::NativeCompoundComponent => option,
                _ => {
                    Message::print(WarningMsg(warnings::WARN_INVALID_RN_ONLY_STYLE));
                    None
                }
            },
            Some(_) => match template {
                Template::NativeComponent | Template::NativeCompoundComponent => {
                    Message::print(WarningMsg(warnings::WARN_INVALID_RN_STYLE));
                    None
                }
                Template::Context => {
                    Message::print(WarningMsg(warnings::WARN_CONTEXT_STYLING));
                    None
                }
                Template::SassModule | Template::RNStyle | Template::Styled => {
                    Message::print(WarningMsg(warnings::WARN_INVALID_STYLE));
                    None
                }
                Template::NextDoc => {
                    Message::print(WarningMsg(warnings::WARN_NDOC_STYLING));
                    None
                }
                _ => option,
            },
            None => None,
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
