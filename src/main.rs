#[macro_use]
extern crate clap;

use rhai::Engine;
use clap::Arg;

mod bool_false_as_none;
mod hook;
mod profile;
mod script;
mod setting;
mod template;

fn main() {
    let matches = app_from_crate!().get_matches();
    let engine = Engine::new();

    profile::parse("/home/jeff/.config/rconfigure/profiles/profile.toml").apply(&engine);
}
