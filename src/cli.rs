use clap::ArgMatches;

fn matches<'a>() -> ArgMatches<'a> {
    app_from_crate!().get_matches()
}

pub fn run() -> Result<(), crate::error::Error> {
    let _matches = matches();

    Ok(())
}
