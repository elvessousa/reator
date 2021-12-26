mod contents;

#[derive(Debug)]
pub enum Template {
    Component,
    Context,
    NativeComponent,
    NextPage,
    NextDoc,
    StatelessComponent,
    Style,
    Styled,
}

impl Template {
    pub fn from(template: &str) -> Option<Self> {
        match template {
            "component" | "rc" => Some(Template::Component),
            "native" | "rnc" => Some(Template::NativeComponent),
            "context" | "ctx" => Some(Template::Context),
            "next-page" | "np" => Some(Template::NextPage),
            "next-doc" | "nd" => Some(Template::NextDoc),
            "stateless" | "st" => Some(Template::StatelessComponent),
            "style" | "s" => Some(Template::Style),
            "styled" | "sc" => Some(Template::Styled),
            _ => None,
        }
    }

    pub fn to_string(template: &str, name: &str) -> String {
        let body = contents::Content;

        match Template::from(template) {
            Some(Template::Component) => body.component(name),
            Some(Template::NativeComponent) => body.native(name),
            Some(Template::Context) => body.context(name),
            Some(Template::NextPage) => body.page(name),
            Some(Template::NextDoc) => body.document(),
            Some(Template::StatelessComponent) => body.stateless(name),
            Some(Template::Style) => body.style(),
            Some(Template::Styled) => body.styled_component(),
            None => "".to_string(),
        }
    }

    pub fn to_path(template: &str) -> String {
        match Template::from(template) {
            Some(Template::Component) => "./src/components/".to_owned(),
            Some(Template::NativeComponent) => "./src/components/".to_owned(),
            Some(Template::Context) => "./src/contexts/".to_owned(),
            Some(Template::NextPage) => "./src/pages/".to_owned(),
            Some(Template::NextDoc) => "./src/pages/".to_owned(),
            Some(Template::StatelessComponent) => "./src/components/".to_owned(),
            Some(Template::Style) => "./src/components/".to_owned(),
            Some(Template::Styled) => "./src/components/".to_owned(),
            _ => "".to_owned(),
        }
    }
}
