use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use super::template::Template;
use super::utils::Message;

pub struct ReactFile;

impl ReactFile {
    pub fn create(&self, template: &str, name: &str) -> Result<(), Box<dyn Error>> {
        let fullpath = self.path(template, name);
        let mut file = File::create(&fullpath)?;
        write!(file, "{}", Template::to_string(template, &name))?;

        Message::print(Message::CreateMsg, &fullpath);
        Ok(())
    }

    fn path(&self, template: &str, name: &str) -> String {
        let dirname = Template::to_path(template);

        if let Err(_) = fs::metadata(&dirname) {
            Message::print(
                Message::WarningMsg,
                format!("Creating directory: {}", &dirname).as_str(),
            );
            fs::create_dir_all(&dirname).unwrap();
        }

        format!("{}{}{}", dirname, name, self.extension())
    }

    fn extension(&self) -> &str {
        if Path::new("./tsconfig.json").exists() {
            Message::print(Message::FoundMsg, "tsconfig.json");
            ".tsx"
        } else {
            ".js"
        }
    }
}
