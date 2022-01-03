pub mod warnings;

use std::error::Error;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub enum Message<'a> {
    AboutMsg,
    CreateMsg(&'a str),
    ErrorMsg(Box<dyn Error>),
    MistakeMsg(String),
    FoundMsg(&'a str),
    InfoMsg(&'a str),
    NotFoundMsg(&'a str),
    SuccessMsg(&'a str),
    WarningMsg(&'a str),
}

impl<'a> Message<'a> {
    pub fn print(msg: Message) {
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

        let sign = format!("{}::{} ", dim, reset);
        let fail = format!("{}{} ERR {}", bg_red, black, reset);
        let ok = format!("{}{} OK! {}", bg_green, black, reset);
        let warn = format!("{}{} WARN {}", bg_yellow, black, reset);

        let message = match msg {
            Message::AboutMsg => format!("\n {}{}Reator {}{}\n", sign, bold, VERSION, reset),
            Message::CreateMsg(s) => format!(" {} File {}{}{} created!", ok, green, s, reset),
            Message::ErrorMsg(e) => format!(" {} {}\n", fail, e),
            Message::FoundMsg(s) => format!(" A {}{}{} file was found.", cyan, s, reset),
            Message::InfoMsg(s) => format!(" {}\n", s),
            Message::MistakeMsg(t) => format!(" {} {}\n", fail, t),
            Message::NotFoundMsg(s) => format!(" {} Not found: {}{}{} file.", fail, red, s, reset),
            Message::SuccessMsg(s) => format!("\n {} \n", s),
            Message::WarningMsg(s) => format!(" {} {}", warn, s),
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
    reator <command> <template> <path> [styling options]

{}
    n | new | create            Creates a new file
    h | help                    Shows this screen

{}
    rc  | component             React Component
    cc  | compound-component    React Compound Component
    rn  | native                React Component
    cn  | compound-native       React Native Compound Component
    ct  | context               React Context API file
    np  | next-page             Next.js Page
    ns  | next-ssg              Next.js Static Page
    nss | next-ssr              Next.js SSR Page
    nd  | next-doc              Next.js '_document' file
    s   | style                 CSS Module
    sc  | styled                Styled Component

{}
    Filename only, no extension needed. 
    Reator creates folders and files automatically.

{}
    -rns  | --reactnative-style     Creates a React Native Component in a folder, with a style.js    
    -css  | --css-module            Creates a component in a folder, with a CSS Module  
    -sass | --sass-module           Creates a component in a folder, with a Sass Module 
    ",
        format!(" {}Usage:{}", bold, reset),
        format!(" {}Commands:{}", bold, reset),
        format!(" {}Templates:{}", bold, reset),
        format!(" {}Path:{}", bold, reset),
        format!(" {}Styling Options:{}", bold, reset),
    )
}
