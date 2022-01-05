mod common;
use common::*;
use reator::template::Template;

#[cfg(test)]
#[test]
fn test_components() {
    let components = get_available_components();

    for (template, name) in components.iter() {
        let tpl = Template::from(template).unwrap();
        let contents = Template::to_string(&tpl, &name);

        run_with_args("new", template, name, "");
        assert_eq!(contents, read_file(name, template));
    }

    cleanup();
}

#[test]
fn test_styles() {
    let styles = get_available_styles();

    for (template, name) in styles.iter() {
        let tpl = Template::from(template).unwrap();
        let contents = Template::to_string(&tpl, &name);

        run_with_args("new", template, name, "");
        assert_eq!(contents, read_style(name, template));
    }
}
