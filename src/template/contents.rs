mod strings;

pub struct Content;

impl Content {
    fn imports(&self, kind: &str) -> String {
        match kind {
            "rc" => strings::REACT_IMPORT.to_owned(),
            "rnc" => strings::REACT_NATIVE_IMPORT.to_owned(),
            _ => String::from(""),
        }
    }

    fn body(&self, name: &str, content: String, kind: &str) -> String {
        let imports = self.imports(kind);
        let default = if kind == "np" { "default " } else { "" };

        format!(
            "{}export {}function {}() {{\n  return (\n    {}\n  );\n}}",
            imports, default, name, content
        )
    }

    pub fn component(&self, name: &str) -> String {
        format!("{}", self.body(name, format!("<div>{}</div>", name), "rc"))
    }

    pub fn page(&self, name: &str) -> String {
        format!("{}", self.body(name, format!("<div>{}</div>", name), "np"))
    }

    pub fn native(&self, name: &str) -> String {
        format!(
            "{}",
            self.body(name, format!("<View><Text>{}</Text></View>", name), "rnc")
        )
    }

    pub fn context(&self, name: &str) -> String {
        format!(
            "{}{}",
            strings::REACT_CONTEXT_IMPORT,
            strings::REACT_CONTEXT.replace("[name]", name)
        )
    }

    pub fn document(&self) -> String {
        strings::NEXT_DOCUMENT.to_owned()
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
