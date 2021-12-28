const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub enum Message {
    About,
    Create,
    Error,
    Info,
    Found,
    NotFound,
    Warning,
}

impl Message {
    pub fn print(msg: Message, text: &str) {
        let bg_green = "\x1b[42m";
        let bg_red = "\x1b[41m";
        let bg_yellow = "\x1b[43m";
        let black = "\x1b[30;1m";
        let bold = "\x1b[1m";
        let cyan = "\x1b[36;1m";
        let dim = "\x1b[2m";
        let green = "\x1b[32m";
        let red = "\x1b[31m";
        let reset = "\x1b[0m";

        let failed = format!("{}{} ERR {}", bg_red, black, reset);
        let info = format!("{}{}{}", cyan, text, reset);
        let error = format!("{}{}{}", red, text, reset);
        let ok = format!("{}{} OK! {}", bg_green, black, reset);
        let success = format!("{}{}{}", green, text, reset);
        let warning = format!("{}{} WARN {}", bg_yellow, black, reset);

        let message = match msg {
            Message::About => format!(
                "\n {}::{} {}Reator {}{}\n",
                dim, reset, bold, VERSION, reset
            ),
            Message::Create => format!(" {} File {} created!", ok, success),
            Message::Error => format!(" {} {}\n", failed, text),
            Message::Info => format!(" {}\n", text),
            Message::Found => format!(" A {} file was found.", info),
            Message::NotFound => format!(" {} Couldn't find a {} file.", failed, error),
            Message::Warning => format!(" {} {}", warning, text),
        };

        eprintln!("{}", message);
    }
}

pub fn help() {
    let bold = "\x1b[1m";
    let reset = "\x1b[0m";

    println!(
        r" Simple React CLI for React Projects

{}
    reator <command> <template> <path> [options]

{}
    n | new | create        Creates a new file
    h | help                Shows this screen

{}
    rc | component              React Component
    cc | compound-component     React Compound Component
    rn | native                 React Component
    cn | compound-native        React Native Compound Component
    ct | context                React Context API file
    np | next-page              Next.js Page
    nd | next-doc               Next.js '_document' file
    s  | style                  CSS Module
    sc | styled                 Styled Component

{}
    Filename only, no extension needed. 
    Reator creates folders and files automatically.

{}
    -rns  | --reactnative-style     Creates a component in a folder, with a style.js    
    -css  | --css-module            Creates a component in a folder, with a CSS Module  
    -sass | --sass-module           Creates a component in a folder, with a Sass Module 
    ",
        format!(" {}Usage:{}", bold, reset),
        format!(" {}Commands:{}", bold, reset),
        format!(" {}Templates:{}", bold, reset),
        format!(" {}Path:{}", bold, reset),
        format!(" {}Options:{}", bold, reset),
    )
}
