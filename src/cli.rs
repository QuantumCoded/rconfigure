use clap::ArgMatches;

/// Uses clap to parse the command line and get matches to the described syntax.
fn matches() -> ArgMatches {
    app_from_crate!().get_matches()
}

/// Parses the command line and initiates the main control flow for the application.
pub fn run() -> Result<(), crate::error::Error> {
    let _matches = matches();

    Ok(())
}
