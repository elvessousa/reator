mod contents;

#[derive(Debug)]
pub enum Template {
    Component,
    CompoundComponent,
    Context,
    NativeComponent,
    NativeCompoundComponent,
    NextPage,
    NextDoc,
    StatelessComponent,
    Style,
    StyleModule,
    Styled,
}

impl Template {
    pub fn from(template: &str) -> Option<Self> {
        match template {
            "component" | "rc" => Some(Template::Component),
            "compound-component" | "cc" => Some(Template::CompoundComponent),
            "native" | "rn" => Some(Template::NativeComponent),
            "compound-native" | "cn" => Some(Template::NativeCompoundComponent),
            "context" | "cx" => Some(Template::Context),
            "next-page" | "np" => Some(Template::NextPage),
            "next-doc" | "nd" => Some(Template::NextDoc),
            "stateless" | "st" => Some(Template::StatelessComponent),
            "style" | "s" => Some(Template::Style),
            "css-module" | "css" => Some(Template::StyleModule),
            "sass-module" | "scss" => Some(Template::StyleModule),
            "styled" | "sc" => Some(Template::Styled),
            _ => None,
        }
    }

    pub fn to_string(template: &str, name: &str) -> String {
        let body = contents::Content;

        match Template::from(template) {
            Some(Template::Component) => body.component(name),
            Some(Template::CompoundComponent) => body.compound(name),
            Some(Template::NativeComponent) => body.native(name),
            Some(Template::NativeCompoundComponent) => body.native_compound(name),
            Some(Template::Context) => body.context(name),
            Some(Template::NextPage) => body.page(name),
            Some(Template::NextDoc) => body.document(),
            Some(Template::StatelessComponent) => body.stateless(name),
            Some(Template::Style) => body.style(),
            Some(Template::StyleModule) => body.style_module(),
            Some(Template::Styled) => body.styled_component(),
            None => "".to_string(),
        }
    }

    pub fn to_path(template: &str) -> String {
        match Template::from(template) {
            Some(Template::Component) => "./src/components/".to_owned(),
            Some(Template::CompoundComponent) => "./src/components/".to_owned(),
            Some(Template::NativeComponent) => "./src/components/".to_owned(),
            Some(Template::NativeCompoundComponent) => "./src/components/".to_owned(),
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
