use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use super::messages::Message::{self, *};
use super::template::Template;
use super::Options;

pub struct ReactFile;

type OperationResult = Result<(), Box<dyn Error>>;

impl ReactFile {
    pub fn create(&self, template: &str, name: &str, option: Option<Options>) -> OperationResult {
        let filepath = self.path(template, name, &option);

        let indexpath = format!("{}{}{}", &filepath, "index", self.extension(false));
        let rn_stylepath = format!("{}{}", &filepath, "styles");
        let modulepath = format!("{}{}{}", &filepath, name, ".module");

        let stylepath = match option {
            Some(Options::ReactNativeStyle) => format!("{}{}", rn_stylepath, self.extension(true)),
            Some(Options::CSSModule) => format!("{}{}", modulepath, ".css"),
            Some(Options::SassModule) => format!("{}{}", modulepath, ".scss"),
            None => "".to_owned(),
        };

        match option {
            Some(Options::ReactNativeStyle) => {
                self.write(&indexpath, &template, &name)?;
                self.write(&stylepath, "native-style", &name)?;
            }
            Some(_) => {
                self.write(&indexpath, &template, &name)?;
                self.write(&stylepath, "css", &name)?;
            }
            None => self.write(&filepath, &template, &name)?,
        }

        Ok(())
    }

    fn write(&self, path: &str, template: &str, name: &str) -> OperationResult {
        let mut file = File::create(&path)?;
        write!(file, "{}", Template::to_string(template, &name))?;

        Message::print(SuccessMsg, &path);
        Ok(())
    }

    fn path(&self, template: &str, name: &str, option: &Option<Options>) -> String {
        let dirname = Template::to_path(template);
        let filename = self.filename(template, name);

        let extension = match Template::from(template).unwrap() {
            Template::RNStyle | Template::Styled => self.extension(true),
            Template::StyleModule => ".css",
            _ => self.extension(false),
        };

        let name_as_folder = format!("{}{}/", dirname, filename);
        let name_as_file = format!("{}{}{}", dirname, filename, extension);

        match option {
            Some(_) => {
                self.check_dirs(&name_as_folder);
                name_as_folder
            }
            None => {
                self.check_dirs(&dirname);
                name_as_file
            }
        }
    }

    fn check_dirs(&self, dirname: &str) {
        if let Err(_) = fs::metadata(&dirname) {
            Message::print(
                InfoMsg,
                format!("Creating directory: {}", &dirname).as_str(),
            );

            fs::create_dir_all(&dirname).unwrap()
        }
    }

    fn filename(&self, template: &str, name: &str) -> String {
        match Template::from(template).unwrap() {
            Template::NextDoc => "_document".to_owned(),
            Template::NextPage => name.to_lowercase(),
            _ => name.to_owned(),
        }
    }

    fn extension(&self, module: bool) -> &str {
        let ts_extension = if module { ".ts" } else { ".tsx" };

        match Path::new("./tsconfig.json").exists() {
            true => ts_extension,
            false => ".js",
        }
    }
}
