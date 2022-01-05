use crate::{
    errors::CrateError,
    profile::{self, Profile},
};
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

pub fn set_active_profile<P: AsRef<Path>>(path: P) -> Result<(), CrateError> {
    let path = if path.as_ref().is_absolute() {
        path.as_ref().to_owned()
    } else {
        config_dir()
            .ok_or_else(|| CrateError::NoConfigDir)?
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
            .ok_or_else(|| CrateError::NoConfigDir)?
            .join("rconfigure/active.toml"),
        toml::to_string(&active)?,
    )
    .unwrap();

    Ok(())
}

pub fn unset_active_profile() {
    // TODO: implement unsetting an active profile
    todo!("unset active profile");
}

pub fn get_active_profile() -> Result<Option<Profile>, CrateError> {
    let path = config_dir()
        .ok_or_else(|| CrateError::NoConfigDir)?
        .join("rconfigure/active.toml");

    let s = match fs::read_to_string(&path) {
        Ok(s) => s,
        Err(err) => {
            use std::io::ErrorKind;

            match err.kind() {
                ErrorKind::NotFound => {
                    fs::write(path, "")?;
                    String::new()
                }

                _ => return Err(err.into()),
            }
        }
    };

    let active: Active = toml::from_str(s.as_str())?;

    Ok(if active.active {
        Some(profile::parse(active.profile))
    } else {
        None
    })
}
