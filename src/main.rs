#[macro_use]
extern crate clap;

use clap::{Arg, SubCommand};
use rhai::Engine;

mod active;
mod bool_false_as_none;
mod hook;
mod profile;
mod script;
mod setting;
mod template;

fn main() {
    let engine = Engine::new();
    let matches = app_from_crate!()
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
        .get_matches();

    match matches.subcommand() {
        ("profile", Some(sub_matches)) => {
            match sub_matches.subcommand() {
                ("set", Some(sub_matches)) => {
                    if let Some(profile) = sub_matches.value_of("PROFILE") {
                        active::set_active_profile(profile);
                        profile::parse(profile).apply(&engine);
                    } else {
                        todo!("use quiz crate");
                        // TODO: use quiz
                    }
                }

                ("unset", Some(_sub_matches)) => active::unset_active_profile(),

                _ => {}
            }
        }

        ("setting", Some(sub_matches)) => {
            match sub_matches.subcommand() {
                ("enable", Some(sub_matches)) => {
                    if let Some(mut profile) = active::get_active_profile() {
                        if let Some(settings) = sub_matches.values_of("SETTINGS") {
                            for setting in settings {
                                profile.enable_setting(setting, sub_matches.is_present("noconfirm"));
                            }
                        } else {
                            todo!("use quiz crate");
                            // TODO: use quiz
                        }
                    } else {
                        println!("No active profile");
                        std::process::exit(1);
                    }
                }

                ("disable", Some(sub_matches)) => {
                    if let Some(mut profile) = active::get_active_profile() {
                        if let Some(settings) = sub_matches.values_of("SETTINGS") {
                            for setting in settings {
                                profile.disable_setting(setting);
                            }
                        } else {
                            todo!("use quiz crate");
                            // TODO: use quiz
                        }
                    } else {
                        println!("No active profile");
                        std::process::exit(1);
                    }
                }

                _ => {}
            }
        }

        ("reload", ..) => {
            if let Some(profile) = active::get_active_profile() {
                profile.apply(&engine);
            } else {
                println!("No active profile");
                std::process::exit(1);
            }
        }

        _ => {}
    }
}
