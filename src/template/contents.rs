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
            "rc" | "cc" | "gs" => strings::REACT_IMPORT,
            "rn" | "cn" | "nsc" => strings::REACT_NATIVE_IMPORT,
            "tcc" => strings::REACT_TYPED_IMPORT,
            "tcn" => strings::REACT_NATIVE_TYPED_IMPORT,
            "tgs" => strings::GATSBY_TYPED_IMPORT,
            "tns" => strings::NEXT_TYPED_SSG_IMPORT,
            "tnss" => strings::NEXT_TYPED_SSR_IMPORT,
            _ => "",
        };

        format!("{}{}", import, self.style_import())
    }

    fn style_import(&self) -> String {
        let args = Arguments::new(env::args()).unwrap_or(Arguments {
            command: "new".to_owned(),
            template: "".to_owned(),
            path: "".to_owned(),
            option: "".to_owned(),
        });

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
            "np" | "tnp" | "ns" | "tns" | "nsc" => "default ",
            _ => "",
        };

        default.to_owned()
    }

    fn props(&self, kind: &str) -> String {
        let props = match kind {
            "cc" | "cn" => "{ children }",
            "gs" => "{ serverData }",
            "tcc" | "tcn" => "{ children }: Props",
            "tgs" => "{ serverData }: PageProps",
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
            "rn" | "trn" | "cn" | "tcn" | "nsc" => {
                format!("<View><Text>{}</Text></View>", self.name)
            }
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

    pub fn ssr_gatsby(&self) -> String {
        let kind = if self.is_typescript() { "tgs" } else { "gs" };
        let props = strings::GATSBY_SSR_SERVERDATA;

        format!("{}\n{}", self.body(kind), props)
    }

    pub fn native(&self) -> String {
        self.body("rn")
    }

    pub fn native_screen(&self) -> String {
        self.body("nsc")
    }

    pub fn native_compound(&self) -> String {
        let kind = if self.is_typescript() { "tcn" } else { "cn" };
        self.body(kind)
    }

    pub fn context(&self) -> String {
        let imports = match self.is_typescript() {
            true => strings::REACT_TYPED_CONTEXT_IMPORT,
            false => strings::REACT_CONTEXT_IMPORT,
        };

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
            imports,
            typing.replace("[name]", &self.name),
            content.replace("[name]", &self.name)
        )
    }

    pub fn next_api_route(&self) -> String {
        match self.is_typescript() {
            true => strings::NEXT_API_ROUTE_TS.to_owned(),
            false => strings::NEXT_API_ROUTE.to_owned(),
        }
    }

    pub fn document(&self) -> String {
        match self.is_typescript() {
            true => strings::NEXT_DOCUMENT_TS.to_owned(),
            false => strings::NEXT_DOCUMENT.to_owned(),
        }
    }

    pub fn stateless(&self) -> String {
        strings::REACT_STATELESS
            .replace("[name]", &self.name)
            .to_owned()
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

#[cfg(test)]
mod tests {
    use super::*;

    fn new_content<'a>() -> Content<'a> {
        Content {
            name: "TestContent".to_owned(),
            template: &Template::Component,
        }
    }

    #[test]
    fn does_not_return_empty_strings() {
        let content = new_content();

        let api_route = content.next_api_route();
        let component = content.component();
        let compound = content.compound();
        let context = content.context();
        let native = content.native();
        let ncompound = content.native_compound();
        let page = content.page();
        let ssg = content.ssg_page();
        let ssr = content.ssr_page();
        let style = content.style();
        let style_module = content.style_module();
        let styled = content.styled_component();

        assert!(api_route.contains("handler"));
        assert!(component.contains(&content.name));
        assert!(compound.contains(&content.name));
        assert!(context.contains(&content.name));
        assert!(native.contains(&content.name));
        assert!(ncompound.contains(&content.name));
        assert!(page.contains(&content.name));
        assert!(ssg.contains(&content.name));
        assert!(ssr.contains(&content.name));
        assert_ne!(style, "".to_owned());
        assert_ne!(style_module, "".to_owned());
        assert_ne!(styled, "".to_owned());
    }

    #[test]
    fn compound_components_have_children() {
        let content = new_content();
        let component = content.compound();
        let native_compound = content.native_compound();

        assert!(component.contains("children"));
        assert!(native_compound.contains("children"));
    }

    #[test]
    fn native_components_imports_react() {
        let content = new_content();
        let native = content.native();
        let ncompound = content.native_compound();
        let native_screen = content.native_screen();

        assert!(native.contains("import React"));
        assert!(ncompound.contains("import React"));
        assert!(native_screen.contains("import React"));
    }

    #[test]
    fn native_components_contains_views() {
        let content = new_content();
        let native = content.native();
        let ncompound = content.native_compound();
        let native_screen = content.native_screen();

        assert!(native.contains("<View>"));
        assert!(ncompound.contains("<View>"));
        assert!(native_screen.contains("<View>"));
    }

    #[test]
    fn ssg_and_ssr_differs() {
        let content = new_content();
        let ssg = content.ssg_page();
        let ssr = content.ssr_page();

        assert_ne!(ssg, ssr);
    }
}
