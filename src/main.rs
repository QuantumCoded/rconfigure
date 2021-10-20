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

    let profile = profile::parse("./config/profiles/profile.toml");

    println!("{:#?}", profile);
}
