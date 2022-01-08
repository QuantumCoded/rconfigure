#[macro_use]
extern crate clap;

mod cli;
mod dirs;
mod error;
mod path;
mod profile;
mod script;
mod setting;

fn main() -> Result<(), main_error::MainError> {
    Ok(cli::run()?)
}
