#[macro_use]
extern crate clap;

mod cli;
mod dirs;
mod error;
mod path;
mod profile;
mod script;

fn main() -> Result<(), main_error::MainError> {
    Ok(cli::run()?)
}
