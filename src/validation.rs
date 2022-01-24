use crate::messages::warnings::*;
use crate::messages::Message::{self, *};
use crate::{commands::Options, template::Template};
use std::{error::Error, path::Path, process};

type ValidationResult<T> = Result<T, Box<dyn Error>>;

pub struct Validation;

impl Validation {
    pub fn project(&self) -> ValidationResult<()> {
        if Path::new("./package.json").exists() {
            Ok(())
        } else {
            Message::print(NotFoundMsg("package.json"));
            Message::print(InfoMsg(INFO_PROJECT_ROOT));
            process::exit(1);
        }
    }

    pub fn template(&self, template: &str) -> ValidationResult<Template> {
        Ok(Template::from(template).unwrap_or_else(|| {
            Message::print(MistakeMsg(format!("Invalid template: '{}'", template)));
            process::exit(1);
        }))
    }

    pub fn styling(&self, template: &Template, option: &str, warnings: bool) -> Option<Options> {
        let option = Options::from(option);

        self.warning(template, &option, warnings);

        match option {
            Some(Options::ReactNativeStyle) => match template {
                Template::NativeComponent | Template::NativeCompoundComponent => option,
                _ => None,
            },
            Some(_) => match template {
                Template::Context => None,
                Template::NativeComponent | Template::NativeCompoundComponent => None,
                Template::NextDoc | Template::NextAPIRoute => None,
                Template::SassModule | Template::RNStyle | Template::Styled => None,
                _ => option,
            },
            None => None,
        }
    }

    fn warning(&self, template: &Template, option: &Option<Options>, visible: bool) {
        if !visible {
            return;
        }

        let message = match option {
            Some(Options::ReactNativeStyle) => match template {
                Template::NativeComponent | Template::NativeCompoundComponent => "",
                _ => WARN_INVALID_RN_ONLY_STYLE,
            },
            Some(_) => match template {
                Template::Context => WARN_CONTEXT_STYLING,
                Template::NativeComponent | Template::NativeCompoundComponent => {
                    WARN_INVALID_RN_STYLE
                }
                Template::NextAPIRoute => WARN_API_STYLING,
                Template::NextDoc => WARN_NDOC_STYLING,
                Template::SassModule | Template::RNStyle | Template::Styled => WARN_INVALID_STYLE,
                _ => "",
            },
            None => "",
        };

        if message != "" {
            Message::print(WarningMsg(message))
        }
    }
}
