use crate::hook::Hook;
use crate::setting::{self, Setting};
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Debug)]
struct ProfileDeserialized {
    profile: ProfileTable,
}

#[derive(Deserialize, Debug)]
struct ProfileTable {
    name: Option<String>,
    settings: Option<Vec<String>>,
    hooks: Option<Vec<String>>,
}

#[derive(Debug)]
pub struct Profile {
    name: String,
    pub settings: Vec<Setting>,
    hooks: Vec<Hook>,
}

impl Profile {
    // TODO: implement a method to check for setting target collisions
    // TODO: implement an apply method that overwrites all the config files
}

pub fn parse<P: AsRef<Path>>(path: P) -> Profile {
    // FIXME: handle errors for file not found
    let s = fs::read_to_string(path.as_ref()).unwrap();
    let profile: ProfileDeserialized = match toml::from_str(s.as_str()) {
        Ok(value) => value,
        Err(e) => {
            println!("error when parsing setting {:?}", path.as_ref());
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
                let path = PathBuf::from("/home/jeff/.config/rconfigure/settings").join(path);
                settings.push(setting::parse(path));
            }
        }
    }

    // TODO: pase all of the hooks and add them to the vector

    Profile {
        name: profile.profile.name.unwrap_or(
            path.as_ref()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
        ),
        settings,
        hooks,
    }
}
