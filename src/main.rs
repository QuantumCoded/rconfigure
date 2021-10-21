#[macro_use]
extern crate clap;

use clap::Arg;

mod hook;
mod profile;
mod script;
mod setting;
mod template;

fn main() {
    let matches = app_from_crate!().get_matches();

    // get the selected profile
    // get the settings for the selected profile
    // run the scripts for each setting
    // apply the settings to the templates
    // overwrite the config files
    // run the hooks

    let profile = profile::parse("/home/jeff/.config/rconfigure/profiles/profile.toml");
    let setting = &profile.settings[0];

    let old_map = setting.compose_map("/home/jeff/.config/rconfigure/templates/madlibs");
    let mut new_map = std::collections::HashMap::new();
    let mut map = std::collections::HashMap::new();

    for (k, v) in old_map {
        new_map.insert(k, v.to_string());
    }

    for (k, v) in &new_map {
        map.insert(k.as_ref(), v.as_ref());
    }

    let out = template::generate_config("/home/jeff/.config/rconfigure/templates/madlibs", &map);

    println!("{:?}", out);
}
