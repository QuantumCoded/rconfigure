#[macro_use]
extern crate clap;

use clap::Arg;

mod hook;
mod profile;
mod script;
mod setting;
mod template;
mod engine;

fn main() {
    let matches = app_from_crate!().get_matches();
    let mut engine = engine::engine();

    profile::parse("/home/jeff/.config/rconfigure/profiles/profile.toml").apply(&mut engine);
}
