mod strings;

use crate::{Arguments, Options};
use std::{env, path::Path};

pub struct Content {
    pub name: String,
}

impl Content {
    fn imports(&self, kind: &str) -> String {
        let import = match kind {
            "rc" | "cc" => strings::REACT_IMPORT,
            "rn" | "cn" => strings::REACT_NATIVE_IMPORT,
            "tcc" => strings::REACT_TYPED_IMPORT,
            "tcn" => strings::REACT_NATIVE_TYPED_IMPORT,
            "tns" => strings::NEXT_TYPED_SSG_IMPORT,
            "tnss" => strings::NEXT_TYPED_SSR_IMPORT,
            _ => "",
        };

        format!("{}{}", import, self.style_import())
    }

    fn style_import(&self) -> String {
        let args = Arguments::new(env::args()).unwrap();

        match Options::from(&args.option) {
            Some(Options::ReactNativeStyle) => {
                format!("import {{ styles }} from './styles';\n\n")
            }
            Some(Options::CSSModule) => {
                format!("import styles from './{}.module.css';\n\n", self.name)
            }
            Some(Options::SassModule) => {
                format!("import styles from './{}.module.scss';\n\n", self.name)
            }
            None => "".to_owned(),
        }
    }

    fn typings(&self, kind: &str) -> String {
        let typing = match self.is_typescript() {
            true => strings::REACT_COMPONENT_TYPING,
            false => "",
        };

        match kind {
            "tcc" | "tcn" => typing.to_owned(),
            _ => "".to_owned(),
        }
    }

    fn is_typescript(&self) -> bool {
        Path::new("./tsconfig.json").exists()
    }

    fn body(&self, content: String, kind: &str) -> String {
        let imports = self.imports(kind);
        let typing = self.typings(kind);

        let default = match kind {
            "np" | "tnp" | "ns" | "tns" => "default ",
            _ => "",
        };

        let props = match kind {
            "tcc" | "tcn" => "{ children }: Props",
            "cc" | "cn" => "{ children }",
            _ => "",
        };

        format!(
            "{}{}export {}function {}({}) {{\n  return (\n    {}\n  );\n}}",
            imports, typing, default, self.name, props, content
        )
    }

    pub fn component(&self) -> String {
        format!("{}", self.body(format!("<div>{}</div>", self.name), "rc"))
    }

    pub fn compound(&self) -> String {
        let kind = if self.is_typescript() { "tcc" } else { "cc" };
        format!("{}", self.body(format!("<div>{}</div>", self.name), kind))
    }

    pub fn page(&self) -> String {
        format!("{}", self.body(format!("<div>{}</div>", self.name), "np"))
    }

    pub fn ssg_page(&self) -> String {
        let kind = if self.is_typescript() { "tns" } else { "ns" };
        let props = match self.is_typescript() {
            true => strings::NEXT_STATIC_PROPS_TS,
            false => strings::NEXT_STATIC_PROPS,
        };

        let body = format!("{}", self.body(format!("<div>{}</div>", self.name), kind));
        format!("{}\n{}", body, props)
    }

    pub fn ssr_page(&self) -> String {
        let kind = if self.is_typescript() { "tnss" } else { "nss" };
        let props = match self.is_typescript() {
            true => strings::NEXT_SSR_PROPS_TS,
            false => strings::NEXT_SSR_PROPS,
        };

        let body = format!("{}", self.body(format!("<div>{}</div>", self.name), kind));
        format!("{}\n{}", body, props)
    }

    pub fn native(&self) -> String {
        let contents = format!("<View><Text>{}</Text></View>", self.name);
        format!("{}", self.body(contents, "rn"))
    }

    pub fn native_compound(&self) -> String {
        let kind = if self.is_typescript() { "tcn" } else { "cn" };
        let contents = format!("<View><Text>{}</Text></View>", self.name);
        format!("{}", self.body(contents, kind))
    }

    pub fn context(&self) -> String {
        let typing = match self.is_typescript() {
            true => strings::REACT_CONTEXT_TYPING,
            false => "",
        };

        let content = match self.is_typescript() {
            true => strings::REACT_CONTEXT_TYPED,
            false => strings::REACT_CONTEXT,
        };

        format!(
            "{}{}{}",
            strings::REACT_CONTEXT_IMPORT,
            typing.replace("[name]", &self.name),
            content.replace("[name]", &self.name)
        )
    }

    pub fn document(&self) -> String {
        match self.is_typescript() {
            true => strings::NEXT_DOCUMENT_TS.to_owned(),
            false => strings::NEXT_DOCUMENT.to_owned(),
        }
    }

    pub fn stateless(&self) -> String {
        format!("{}", strings::REACT_STATELESS.replace("[name]", &self.name))
    }

    pub fn style(&self) -> String {
        strings::REACT_STYLE.to_owned()
    }

    pub fn style_module(&self) -> String {
        strings::REACT_CSS_MODULE.to_owned()
    }

    pub fn styled_component(&self) -> String {
        strings::REACT_STYLED.to_owned()
    }
}
