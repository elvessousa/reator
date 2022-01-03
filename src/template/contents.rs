mod strings;

use crate::{commands::Options, template::Template, validation::Validation, Arguments};
use std::{env, path::Path};

pub struct Content<'a> {
    pub name: String,
    pub template: &'a Template,
}

impl<'a> Content<'a> {
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
        let options = Validation.styling(self.template, &args.option, false);

        match options {
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

    fn default(&self, kind: &str) -> String {
        let default = match kind {
            "np" | "tnp" | "ns" | "tns" => "default ",
            _ => "",
        };

        default.to_owned()
    }

    fn props(&self, kind: &str) -> String {
        let props = match kind {
            "tcc" | "tcn" => "{ children }: Props",
            "cc" | "cn" => "{ children }",
            _ => "",
        };

        props.to_owned()
    }

    fn is_typescript(&self) -> bool {
        Path::new("./tsconfig.json").exists()
    }

    fn body(&self, kind: &str) -> String {
        let imports = self.imports(kind);
        let typing = self.typings(kind);
        let default = self.default(kind);
        let props = self.props(kind);

        let contents = match kind {
            "rn" | "trn" | "cn" | "tcn" => format!("<View><Text>{}</Text></View>", self.name),
            _ => format!("<div>{}</div>", self.name),
        };

        format!(
            "{}{}export {}function {}({}) {{\n  return (\n    {}\n  );\n}}",
            imports, typing, default, self.name, props, contents
        )
    }

    pub fn component(&self) -> String {
        self.body("rc")
    }

    pub fn compound(&self) -> String {
        let kind = if self.is_typescript() { "tcc" } else { "cc" };
        self.body(kind)
    }

    pub fn page(&self) -> String {
        self.body("np")
    }

    pub fn ssg_page(&self) -> String {
        let kind = if self.is_typescript() { "tns" } else { "ns" };
        let props = match self.is_typescript() {
            true => strings::NEXT_STATIC_PROPS_TS,
            false => strings::NEXT_STATIC_PROPS,
        };

        format!("{}\n{}", self.body(kind), props)
    }

    pub fn ssr_page(&self) -> String {
        let kind = if self.is_typescript() { "tnss" } else { "nss" };
        let props = match self.is_typescript() {
            true => strings::NEXT_SSR_PROPS_TS,
            false => strings::NEXT_SSR_PROPS,
        };

        format!("{}\n{}", self.body(kind), props)
    }

    pub fn native(&self) -> String {
        self.body("rn")
    }

    pub fn native_compound(&self) -> String {
        let kind = if self.is_typescript() { "tcn" } else { "cn" };
        self.body(kind)
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
