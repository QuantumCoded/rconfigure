#[macro_use]
extern crate clap;

mod cli;
mod dirs;
mod error;
mod path;

fn main() -> Result<(), main_error::MainError> {
    Ok(cli::run()?)
}
