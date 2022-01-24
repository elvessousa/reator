use reator::template::Template;
use reator::Arguments;
use std::collections::HashMap;
use std::{fs, fs::File, io::prelude::*, path::Path};

pub fn get_available_components<'a>() -> HashMap<&'a str, &'a str> {
    let mut components = HashMap::new();
    components.insert("component", "Component");
    components.insert("compound-component", "CompoundComponent");
    components.insert("compound-native", "NativeCompoundComponent");
    components.insert("context", "New");
    components.insert("native", "NativeComponent");
    components.insert("native-screen", "NativeScreen");
    components.insert("next-api", "NextAPIRoute");
    components.insert("next-doc", "NextDocument");
    components.insert("next-page", "NextPage");
    components.insert("next-ssr", "NextSSRPage");
    components.insert("next-ssg", "NextSSGPage");
    components.insert("stateless", "StatelessComponent");

    components
}

pub fn get_available_styles<'a>() -> HashMap<&'a str, &'a str> {
    let mut styles = HashMap::new();
    styles.insert("css", "CSSStyle");
    styles.insert("sass", "SassStyle");
    styles.insert("styled", "StyledComponent");
    styles.insert("native-style", "ReactNativeStyle");

    styles
}

pub fn get_name(name: &str, template: &Template) -> String {
    match template {
        Template::NextDoc => "_document".to_owned(),
        Template::NextPage | Template::NextStatic | Template::NextSSR | Template::NextAPIRoute => {
            name.to_lowercase()
        }
        Template::Context => format!("{}Context", name),
        _ => name.to_owned(),
    }
}

pub fn read_file(file: &str, template: &str) -> String {
    let template = Template::from(template).unwrap();
    let base_path = Template::to_path(&template);
    let extension = match Path::new("tsconfig.json").exists() {
        true => {
            if base_path.contains("pages/api") {
                ".ts"
            } else {
                ".tsx"
            }
        }
        false => ".js",
    };

    let file_name = format!("{}{}{}", base_path, get_name(file, &template), extension);
    let mut contents = String::new();

    let mut target = File::open(file_name.as_str())
        .expect(format!("Unable to open file {}", file_name).as_str());

    target
        .read_to_string(&mut contents)
        .expect("Unable to read file");

    contents
}

pub fn read_style(style: &str, template: &str) -> String {
    let jsextension = match Path::new("tsconfig.json").exists() {
        true => ".ts",
        false => ".js",
    };

    let template = Template::from(template).unwrap();
    let extension = match template {
        Template::Styled | Template::RNStyle => jsextension,
        Template::SassModule => ".scss",
        Template::StyleModule => ".css",
        _ => "",
    };

    let base_path = Template::to_path(&template);
    let file_name = format!("{}{}{}", base_path, get_name(style, &template), extension);
    let mut contents = String::new();

    let mut target = File::open(file_name.as_str())
        .expect(format!("Unable to open file {}", file_name).as_str());

    target
        .read_to_string(&mut contents)
        .expect("Unable to read file");

    contents
}

pub fn run_with_args(command: &str, template: &str, path: &str, option: &str) {
    let args = Arguments {
        command: command.to_owned(),
        template: template.to_owned(),
        path: path.to_owned(),
        option: option.to_owned(),
    };

    reator::run(args).unwrap();
}

pub fn cleanup() {
    let paths = vec![
        "./src/components/",
        "./src/contexts/",
        "./src/pages/",
        "./src/screens/",
        "./src/styles/",
    ];

    for path in paths.iter() {
        if Path::new(path).is_dir() {
            fs::remove_dir_all(path).unwrap();
        }
    }
}
