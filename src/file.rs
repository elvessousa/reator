use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use super::messages::Message;
use super::template::Template;
use super::Options;

pub struct ReactFile;

type OperationResult = Result<(), Box<dyn Error>>;

impl ReactFile {
    pub fn create(&self, template: &str, name: &str, option: Option<Options>) -> OperationResult {
        let filepath = self.path(template, name, &option);
        let indexpath = format!("{}{}{}", &filepath, "index", self.extension(false));
        let js_stylepath = format!("{}{}", &filepath, "styles");
        let modulepath = format!("{}{}{}", &filepath, name, ".module");

        let stylepath = match option {
            Some(Options::WithStyle) => format!("{}{}", js_stylepath, self.extension(true)),
            Some(Options::CSSModule) => format!("{}{}", modulepath, ".css"),
            Some(Options::SassModule) => format!("{}{}", modulepath, ".scss"),
            Some(_) | None => "".to_owned(),
        };

        match option {
            Some(Options::WithStyle) => {
                self.write(&indexpath, &template, &name)?;
                self.write(&stylepath, "style", &name)?;
            }
            Some(Options::CSSModule) | Some(Options::SassModule) => {
                self.write(&indexpath, &template, &name)?;
                self.write(&stylepath, "css", &name)?;
            }
            Some(_) | None => self.write(&filepath, &template, &name)?,
        }

        Ok(())
    }

    fn write(&self, path: &str, template: &str, name: &str) -> OperationResult {
        let mut file = File::create(&path)?;
        write!(file, "{}", Template::to_string(template, &name))?;

        Message::print(Message::CreateMsg, &path);
        Ok(())
    }

    fn path(&self, template: &str, name: &str, option: &Option<Options>) -> String {
        let dirname = Template::to_path(template);
        let name_as_folder = format!("{}{}/", dirname, name);
        let name_as_file = format!("{}{}{}", dirname, name, self.extension(false));

        match option {
            Some(Options::WithStyle) | Some(Options::CSSModule) | Some(Options::SassModule) => {
                let module_folder = format!("{}{}/", dirname, &name);
                self.check_dirs(&module_folder);
                name_as_folder
            }
            Some(_) | None => {
                self.check_dirs(&dirname);
                name_as_file
            }
        }
    }

    fn check_dirs(&self, dirname: &str) {
        if let Err(_) = fs::metadata(&dirname) {
            Message::print(
                Message::WarningMsg,
                format!("Creating directory: {}", &dirname).as_str(),
            );

            fs::create_dir_all(&dirname).unwrap()
        }
    }

    fn extension(&self, module: bool) -> &str {
        if Path::new("./tsconfig.json").exists() {
            if module {
                ".ts"
            } else {
                ".tsx"
            }
        } else {
            ".js"
        }
    }
}
