#[macro_use]
extern crate clap;

mod bool_false_as_none;
mod cli;
mod dirs;
mod error;
mod path;
mod profile;
mod script;
mod setting;
mod value;

fn main() -> Result<(), main_error::MainError> {
    Ok(cli::run()?)
}
