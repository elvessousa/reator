mod contents;

#[derive(Debug)]
pub enum Template {
    Component,
    CompoundComponent,
    Context,
    NativeComponent,
    NativeCompoundComponent,
    NextDoc,
    NextPage,
    RNStyle,
    SassModule,
    StatelessComponent,
    StyleModule,
    Styled,
}

impl Template {
    pub fn from(template: &str) -> Option<Self> {
        match template {
            "component" | "rc" => Some(Template::Component),
            "compound-component" | "cc" => Some(Template::CompoundComponent),
            "compound-native" | "cn" => Some(Template::NativeCompoundComponent),
            "context" | "cx" => Some(Template::Context),
            "css" => Some(Template::StyleModule),
            "native" | "rn" => Some(Template::NativeComponent),
            "native-style" | "rns" => Some(Template::RNStyle),
            "next-doc" | "nd" => Some(Template::NextDoc),
            "next-page" | "np" => Some(Template::NextPage),
            "sass" | "scss" => Some(Template::SassModule),
            "stateless" | "st" => Some(Template::StatelessComponent),
            "styled" | "sc" => Some(Template::Styled),
            _ => None,
        }
    }

    pub fn to_string(template: &str, name: &str) -> String {
        let body = contents::Content;

        match Template::from(template) {
            Some(Template::Component) => body.component(name),
            Some(Template::CompoundComponent) => body.compound(name),
            Some(Template::Context) => body.context(name),
            Some(Template::NativeComponent) => body.native(name),
            Some(Template::NativeCompoundComponent) => body.native_compound(name),
            Some(Template::NextDoc) => body.document(),
            Some(Template::NextPage) => body.page(name),
            Some(Template::RNStyle) => body.style(),
            Some(Template::SassModule) => body.style_module(),
            Some(Template::StatelessComponent) => body.stateless(name),
            Some(Template::StyleModule) => body.style_module(),
            Some(Template::Styled) => body.styled_component(),
            None => "".to_string(),
        }
    }

    pub fn to_path(template: &str) -> String {
        match Template::from(template) {
            Some(Template::Component) => "./src/components/".to_owned(),
            Some(Template::CompoundComponent) => "./src/components/".to_owned(),
            Some(Template::Context) => "./src/contexts/".to_owned(),
            Some(Template::NativeComponent) => "./src/components/".to_owned(),
            Some(Template::NativeCompoundComponent) => "./src/components/".to_owned(),
            Some(Template::NextDoc) => "./src/pages/".to_owned(),
            Some(Template::NextPage) => "./src/pages/".to_owned(),
            Some(Template::RNStyle) => "./src/styles/".to_owned(),
            Some(Template::SassModule) => "./src/styles/".to_owned(),
            Some(Template::StatelessComponent) => "./src/components/".to_owned(),
            Some(Template::StyleModule) => "./src/styles/".to_owned(),
            Some(Template::Styled) => "./src/styles/".to_owned(),
            _ => "".to_owned(),
        }
    }
}
