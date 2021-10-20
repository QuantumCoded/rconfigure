use serde::Deserialize;
use toml::Value;
use std::fs;
use std::path::{Path, PathBuf};

use crate::hook::Hook;
use crate::setting::{self, Setting};

#[derive(Deserialize, Debug)]
struct ProfileDeserialized {
    profile: ProfileTable,
}

#[derive(Deserialize, Debug)]
struct ProfileTable {
    name: Option<String>,
    settings: Option<Vec<String>>,
    hooks: Value,
}

#[derive(Debug)]
pub struct Profile {
    name: String,
    settings: Vec<Setting>,
    hooks: Vec<Hook>,
}

pub fn parse<P: AsRef<Path>>(path: P) -> Profile {
    let path = path.as_ref();
    // FIXME: handle errors for file not found
    let s = fs::read_to_string(path).unwrap();
    let profile: ProfileDeserialized = match toml::from_str(s.as_str()) {
        Ok(value) => value,
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    };

    let mut settings = Vec::new();
    let mut hooks = Vec::new();

    // parse all of the settings and add them to the vector
    if let Some(profile_settings) = profile.profile.settings {
        for setting in profile_settings {
            let path = PathBuf::from(setting);
            
            if path.is_absolute() {
                settings.push(setting::parse(path));
            } else {
                // FIXME: use dirs crate
                let path = PathBuf::from("./config/settings").join(path);
                settings.push(setting::parse(path));
            }
        }
    }

    // TODO: pase all of the hooks and add them to the vector

    Profile {
        name: profile
            .profile.name
            .unwrap_or(path.file_name().unwrap().to_str().unwrap().to_string()),
        settings,
        hooks,
    }
}
