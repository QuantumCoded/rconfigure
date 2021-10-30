use crate::profile::{self, Profile};
use dirs::config_dir;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Deserialize, Serialize)]
struct Active {
    #[serde(default)]
    profile: PathBuf,
    #[serde(default)]
    active: bool,
}

pub fn set_active_profile<P: AsRef<Path>>(path: P) {
    let path = if path.as_ref().is_absolute() {
        path.as_ref().to_owned()
    } else {
        // FIXME: better error handling
        config_dir()
            .expect("config dir borked")
            .join("rconfigure/profiles")
            .join(path)
    };

    let active = Active {
        profile: path,
        active: true,
    };

    // FIXME: better error handling
    fs::write(
        config_dir()
            .expect("config dir borked")
            .join("rconfigure/active.toml"),
        toml::to_string(&active).expect("failed to serialize"),
    )
    .unwrap();
}

pub fn unset_active_profile() {
    // TODO: implement unsetting an active profile
    todo!("unset active profile");
}

pub fn get_active_profile() -> Option<Profile> {
    // FIXME: better error handling
    let path = config_dir()
        .expect("config dir borked")
        .join("rconfigure/active.toml");

    // FIXME: handle errors for file not found
    let s = fs::read_to_string(&path).unwrap();
    let active: Active = match toml::from_str(s.as_str()) {
        Ok(value) => value,
        Err(e) => {
            println!("error when parsing active.toml");
            println!("{}", e);
            std::process::exit(1);
        }
    };

    if active.active {
        Some(profile::parse(active.profile))
    } else {
        None
    }
}
