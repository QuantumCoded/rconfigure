mod apply;
mod disable_setting;
mod enable_setting;
mod setting_conflict;

use crate::hook::Hook;
use crate::setting::{self, Setting};
use dirs::config_dir;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Serialize)]
struct ProfileDeserialized {
    #[serde(rename = "profile")]
    profile_table: Option<ProfileTable>,
}

#[derive(Deserialize, Serialize)]
struct ProfileTable {
    name: Option<String>,
    settings: Option<Vec<String>>,
    #[serde(default)]
    hooks: Vec<Hook>,
}

pub struct Profile {
    path: PathBuf,
    name: String,
    settings: Vec<Setting>,
    hooks: Vec<Hook>,
}

pub fn parse<P: AsRef<Path>>(path: P) -> Profile {
    let path = if path.as_ref().is_absolute() {
        path.as_ref().to_owned()
    } else {
        // FIXME: better error handling
        config_dir()
            .expect("config dir borked")
            .join("rconfigure/profiles")
            .join(path)
    };

    // FIXME: handle errors for file not found
    let s = fs::read_to_string(&path).unwrap();
    let profile: ProfileDeserialized = match toml::from_str(s.as_str()) {
        Ok(value) => value,
        Err(e) => {
            println!("error when parsing setting {:?}", path);
            println!("{}", e);
            std::process::exit(1);
        }
    };

    let mut settings_buf = Vec::new();

    // parse all of the settings and add them to the vector
    if let Some(ProfileTable {
        settings: Some(settings),
        ..
    }) = &profile.profile_table
    {
        for setting in settings {
            let path = PathBuf::from(setting);

            settings_buf.push(setting::parse(path));
        }
    }

    Profile {
        name: profile
            .profile_table
            .as_ref()
            .and_then(|t| t.name.clone())
            .unwrap_or_else(|| path.file_name().unwrap().to_str().unwrap().to_string()),
        settings: settings_buf,
        hooks: profile
            .profile_table
            .and_then(|t| Some(t.hooks))
            .unwrap_or_default(),
        path,
    }
}
