use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use super::template::Template;

pub struct ReactFile;

impl ReactFile {
    pub fn create(&self, template: &str, name: &str) -> Result<(), Box<dyn Error>> {
        let fullpath = self.path(template, name);

        let mut file = File::create(&fullpath)?;
        println!("Creating file {} with the template {:?}", name, template);
        write!(file, "{}", Template::to_string(template, &name))?;

        Ok(())
    }

    fn path(&self, template: &str, name: &str) -> String {
        let dirname = Template::to_path(template);

        if let Ok(_) = fs::metadata(&dirname) {
            println!("Creating file in directory {}", &dirname);
        } else {
            println!("Creating directory: {}", &dirname);
            fs::create_dir_all(&dirname).unwrap();
        }

        format!("{}{}{}", dirname, name, self.extension())
    }

    fn extension(&self) -> &str {
        if Path::new("./tsconfig.json").exists() {
            ".tsx"
        } else {
            ".js"
        }
    }
}
