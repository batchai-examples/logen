use logen::app::App;
use serde_json::value::Value;
use tera::Tera;
use std::fs;
use std::collections::HashMap;

fn main() {
    let config_yaml = fs::read_to_string("logen.default.yaml")
        .expect("failed to open config file: logen.default.yaml");
    let app_def = serde_yaml::from_str(config_yaml.as_str())
        .expect(format!("failed to parse config yaml: {}", config_yaml).as_str());

    let mut tera = Tera::default();
    // disable autoescaping completely
    tera.autoescape_on(vec![]);

    tera.register_filter("align_left", Box::new(tera_filter_align_left));
    //handlebars.register_helper("align_right", Box::new(handlebars_helper_align_right));
    //handlebars.register_helper("to_uppercase", Box::new(handlebars_helper_to_uppercase));
    //handlebars.register_helper("to_lowercase", Box::new(handlebars_helper_to_lowercase));

   // tera.register_filter("to_upper", filter);

    let mut app = App::new(&app_def, &mut tera);
    app.generate(&tera);
}
/*
    let mut rendered = String::from(value);
    let mut len = 0;
    for _ in value.chars() {
        len = len + 1;
    }
    while len < width {
        rendered.push(' ');
        len = len + 1;
    }
*/
/*
    let mut rendered = String::from(value);
    let mut len = 0;
    for _ in value.chars() {
        len = len + 1;
    }
    while len < width {
        rendered.insert(0, ' ');
        len = len + 1;
    }
*/

pub fn tera_filter_align_left(value: &Value, args: &HashMap<String, Value>) -> tera::Result<Value> {
    let mut value = tera::try_get_value!("align_left", "value", String, value);

    let width = match args.get("width") {
        Some(width) => tera::try_get_value!("align_left", "width", i32, width),
        None => return Err(tera::Error::msg("Filter `trim_start_matches` expected an arg called `pat`")),
    };

    let mut len = 0;
    for _ in value.chars() {
        len = len + 1;
    }
    while len < width {
        value.push(' ');
        len = len + 1;
    }

    Ok(tera::to_value(value).unwrap())
}
