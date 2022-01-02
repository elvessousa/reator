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
    NextStatic,
    NextSSR,
    RNStyle,
    SassModule,
    StatelessComponent,
    StyleModule,
    Styled,
}

impl Template {
    pub fn from(template: &str) -> Option<Self> {
        match template {
            "component" | "rc" => Some(Self::Component),
            "compound-component" | "cc" => Some(Self::CompoundComponent),
            "compound-native" | "cn" => Some(Self::NativeCompoundComponent),
            "context" | "cx" => Some(Self::Context),
            "css" => Some(Self::StyleModule),
            "native" | "rn" => Some(Self::NativeComponent),
            "native-style" | "rns" => Some(Self::RNStyle),
            "next-doc" | "nd" => Some(Self::NextDoc),
            "next-page" | "np" => Some(Self::NextPage),
            "next-static" | "ns" => Some(Self::NextStatic),
            "next-ssr" | "nss" => Some(Self::NextSSR),
            "sass" | "scss" => Some(Self::SassModule),
            "stateless" | "st" => Some(Self::StatelessComponent),
            "styled" | "sc" => Some(Self::Styled),
            _ => None,
        }
    }

    pub fn to_string(template: &str, name: &str) -> String {
        let body = contents::Content {
            name: name.to_owned(),
        };

        match Template::from(template).unwrap() {
            Template::Component => body.component(),
            Template::CompoundComponent => body.compound(),
            Template::Context => body.context(),
            Template::NativeComponent => body.native(),
            Template::NativeCompoundComponent => body.native_compound(),
            Template::NextDoc => body.document(),
            Template::NextPage => body.page(),
            Template::NextStatic => body.ssg_page(),
            Template::NextSSR => body.ssr_page(),
            Template::RNStyle => body.style(),
            Template::SassModule => body.style_module(),
            Template::StatelessComponent => body.stateless(),
            Template::StyleModule => body.style_module(),
            Template::Styled => body.styled_component(),
        }
    }

    pub fn to_path(template: &str) -> String {
        let path = match Template::from(template).unwrap() {
            Template::Context => "./src/contexts/",
            Template::NextDoc | Template::NextPage => "./src/pages/",
            Template::NextStatic | Template::NextSSR => "./src/pages/",
            Template::RNStyle | Template::SassModule => "./src/styles/",
            Template::StyleModule | Template::Styled => "./src/styles/",
            _ => "./src/components/",
        };

        path.to_owned()
    }
}
