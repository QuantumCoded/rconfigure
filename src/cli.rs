use clap::{Arg, SubCommand, ArgMatches};

pub fn matches() -> ArgMatches<'static> {
    app_from_crate!()
        .subcommand(
            SubCommand::with_name("profile")
                .alias("p")
                .about("Configure profiles")
                .subcommand(
                    SubCommand::with_name("set")
                        .alias("s")
                        .about("Sets an active profile")
                        .arg(Arg::with_name("PROFILE").index(1)),
                )
                .subcommand(
                    SubCommand::with_name("unset")
                        .alias("u")
                        .about("Unsets the active profile (runs unset hooks)"),
                ),
        )
        .subcommand(
            SubCommand::with_name("setting")
                .alias("s")
                .about("Configure settings")
                .subcommand(
                    SubCommand::with_name("enable")
                        .alias("e")
                        .about("Enables a setting for the active profile")
                        .arg(
                            Arg::with_name("SETTINGS")
                                .index(1)
                                .multiple(true)
                                .takes_value(true),
                        )
                        .arg(
                            Arg::with_name("noconfirm")
                                .long("noconfirm")
                                .short("n")
                                .help("Assumes yes for setting conflicts"),
                        ),
                )
                .subcommand(
                    SubCommand::with_name("disable")
                        .alias("d")
                        .about("Disables a setting for the active profile")
                        .arg(
                            Arg::with_name("SETTINGS")
                                .index(1)
                                .multiple(true)
                                .takes_value(true),
                        ),
                ),
        )
        .subcommand(SubCommand::with_name("reload").about("Reloads the active profile"))
        .get_matches()
}