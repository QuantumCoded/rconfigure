#![allow(dead_code)]
#[macro_use]
extern crate clap;

mod bool_false_as_none;
mod cli;
mod dirs;
mod error;
mod hooks;
mod path;
mod profile;
mod setting;
mod template;
mod value;

fn main() -> Result<(), main_error::MainError> {
    Ok(cli::run()?)
}
