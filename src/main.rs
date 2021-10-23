#[macro_use]
extern crate clap;

use clap::Arg;
use rhai::Engine;

mod bool_false_as_none;
mod hook;
mod profile;
mod script;
mod setting;
mod template;

fn main() {
    let matches = app_from_crate!().get_matches();
    let engine = Engine::new();

    // TODO: implement clap cli

    profile::parse("/home/jeff/.config/rconfigure/profiles/profile.toml").apply(&engine);
}
