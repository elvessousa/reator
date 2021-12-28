mod strings;

use std::path::Path;

pub struct Content;

impl Content {
    fn imports(&self, kind: &str) -> String {
        match kind {
            "rc" | "cc" => strings::REACT_IMPORT.to_owned(),
            "rn" | "cn" => strings::REACT_NATIVE_IMPORT.to_owned(), // done
            "tcc" => strings::REACT_TYPED_IMPORT.to_owned(),
            "tcn" => strings::REACT_NATIVE_TYPED_IMPORT.to_owned(),
            _ => String::from(""),
        }
    }

    fn typings(&self, kind: &str) -> String {
        let typing = if self.is_typescript() {
            strings::REACT_COMPONENT_TYPING
        } else {
            ""
        };

        match kind {
            "tcc" | "tcn" => typing.to_owned(),
            _ => "".to_owned(),
        }
    }

    fn is_typescript(&self) -> bool {
        Path::new("./tsconfig.json").exists()
    }

    fn body(&self, name: &str, content: String, kind: &str) -> String {
        let imports = self.imports(kind);
        let typing = self.typings(kind);
        let default = if kind == "np" { "default " } else { "" };
        let props = match kind {
            "tcc" | "tcn" => "{ children }: Props",
            "cc" | "cn" => "{ children }",
            _ => "",
        };

        format!(
            "{}{}export {}function {}({}) {{\n  return (\n    {}\n  );\n}}",
            imports, typing, default, name, props, content
        )
    }

    pub fn component(&self, name: &str) -> String {
        format!("{}", self.body(name, format!("<div>{}</div>", name), "rc"))
    }

    pub fn compound(&self, name: &str) -> String {
        let kind = if self.is_typescript() { "tcc" } else { "cc" };
        format!("{}", self.body(name, format!("<div>{}</div>", name), kind))
    }

    pub fn page(&self, name: &str) -> String {
        format!("{}", self.body(name, format!("<div>{}</div>", name), "np"))
    }

    pub fn native(&self, name: &str) -> String {
        format!(
            "{}",
            self.body(name, format!("<View><Text>{}</Text></View>", name), "rn")
        )
    }

    pub fn native_compound(&self, name: &str) -> String {
        let kind = if self.is_typescript() { "tcn" } else { "cn" };
        format!(
            "{}",
            self.body(name, format!("<View><Text>{}</Text></View>", name), kind)
        )
    }

    pub fn context(&self, name: &str) -> String {
        let typing = if self.is_typescript() {
            strings::REACT_CONTEXT_TYPING
        } else {
            ""
        };

        let content = if self.is_typescript() {
            strings::REACT_CONTEXT_TYPED
        } else {
            strings::REACT_CONTEXT
        };

        format!(
            "{}{}{}",
            strings::REACT_CONTEXT_IMPORT,
            typing.replace("[name]", name),
            content.replace("[name]", name)
        )
    }

    pub fn document(&self) -> String {
        if self.is_typescript() {
            strings::NEXT_DOCUMENT_TS.to_owned()
        } else {
            strings::NEXT_DOCUMENT.to_owned()
        }
    }

    pub fn stateless(&self, name: &str) -> String {
        format!("{}", strings::REACT_STATELESS.replace("[name]", name))
    }

    pub fn style(&self) -> String {
        strings::REACT_STYLE.to_owned()
    }

    pub fn style_module(&self) -> String {
        strings::REACT_STYLE_MODULE.to_owned()
    }

    pub fn styled_component(&self) -> String {
        strings::REACT_STYLED.to_owned()
    }
}
