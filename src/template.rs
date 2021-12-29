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

        match Template::from(template).unwrap() {
            Template::Component => body.component(name),
            Template::CompoundComponent => body.compound(name),
            Template::Context => body.context(name),
            Template::NativeComponent => body.native(name),
            Template::NativeCompoundComponent => body.native_compound(name),
            Template::NextDoc => body.document(),
            Template::NextPage => body.page(name),
            Template::RNStyle => body.style(),
            Template::SassModule => body.style_module(),
            Template::StatelessComponent => body.stateless(name),
            Template::StyleModule => body.style_module(),
            Template::Styled => body.styled_component(),
        }
    }

    pub fn to_path(template: &str) -> String {
        let path = match Template::from(template).unwrap() {
            Template::Component => "./src/components/",
            Template::CompoundComponent => "./src/components/",
            Template::Context => "./src/contexts/",
            Template::NativeComponent => "./src/components/",
            Template::NativeCompoundComponent => "./src/components/",
            Template::NextDoc => "./src/pages/",
            Template::NextPage => "./src/pages/",
            Template::RNStyle => "./src/styles/",
            Template::SassModule => "./src/styles/",
            Template::StatelessComponent => "./src/components/",
            Template::StyleModule => "./src/styles/",
            Template::Styled => "./src/styles/",
        };

        path.to_owned()
    }
}
