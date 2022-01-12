mod contents;

#[derive(PartialEq, Debug)]
pub enum Template {
    Component,
    CompoundComponent,
    Context,
    NativeComponent,
    NativeCompoundComponent,
    NativeScreen,
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
            "native-screen" | "nsc" => Some(Self::NativeScreen),
            "next-doc" | "nd" => Some(Self::NextDoc),
            "next-page" | "np" => Some(Self::NextPage),
            "next-ssg" | "ns" => Some(Self::NextStatic),
            "next-ssr" | "nss" => Some(Self::NextSSR),
            "sass" | "scss" => Some(Self::SassModule),
            "stateless" | "st" => Some(Self::StatelessComponent),
            "styled" | "sc" => Some(Self::Styled),
            _ => None,
        }
    }

    pub fn to_string(template: &Self, name: &str) -> String {
        let body = contents::Content {
            name: name.to_owned(),
            template,
        };

        match template {
            Template::Component => body.component(),
            Template::CompoundComponent => body.compound(),
            Template::Context => body.context(),
            Template::NativeComponent => body.native(),
            Template::NativeCompoundComponent => body.native_compound(),
            Template::NativeScreen => body.native_screen(),
            Template::NextDoc => body.document(),
            Template::NextPage => body.page(),
            Template::NextSSR => body.ssr_page(),
            Template::NextStatic => body.ssg_page(),
            Template::RNStyle => body.style(),
            Template::SassModule => body.style_module(),
            Template::StatelessComponent => body.stateless(),
            Template::StyleModule => body.style_module(),
            Template::Styled => body.styled_component(),
        }
    }

    pub fn to_path(template: &Self) -> String {
        let path = match template {
            Template::Context => "./src/contexts/",
            Template::NextDoc | Template::NextPage => "./src/pages/",
            Template::NextStatic | Template::NextSSR => "./src/pages/",
            Template::RNStyle | Template::SassModule => "./src/styles/",
            Template::StyleModule | Template::Styled => "./src/styles/",
            Template::NativeScreen => "./src/screens/",
            _ => "./src/components/",
        };

        path.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_only_valid_templates() {
        let valid_template = Template::from("context");
        let invalid_template = Template::from("wow");

        assert_eq!(valid_template, Some(Template::Context));
        assert_eq!(invalid_template, None);
    }

    #[test]
    fn returns_different_template_strings() {
        let cmp_template = Template::to_string(&Template::Component, "Context");
        let ctx_template = Template::to_string(&Template::Context, "Context");

        assert_ne!(cmp_template, "".to_owned());
        assert_ne!(ctx_template, "".to_owned());
        assert_ne!(cmp_template, ctx_template);
    }

    #[test]
    fn returns_only_valid_paths() {
        let comps_path = Template::to_path(&Template::CompoundComponent);
        let pages_path = Template::to_path(&Template::NextStatic);
        let style_path = Template::to_path(&Template::Styled);

        assert_ne!(comps_path, pages_path);
        assert_ne!(comps_path, style_path);
        assert_ne!(pages_path, style_path);
    }
}
