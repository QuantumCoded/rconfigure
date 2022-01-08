use crate::dirs::profiles_dir;
use crate::path::{find_config_file, force_absolute};
use crate::setting::Setting;
use serde::{Deserialize, Serialize};
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    DirError(#[from] crate::dirs::Error),

    #[error("there is no file named {name:?} or {name:?}.toml in {parent:?}")]
    FileNotFound { name: OsString, parent: PathBuf },

    #[error("profile paths must not be '/' or terminate in '..', found: {0:?}")]
    RootOrPrefix(PathBuf),

    #[error("io error when reading profile: {0}")]
    IOError(#[from] std::io::Error),

    #[error("error deserializing profile: {0}")]
    DeserializeError(#[from] toml::de::Error),

    #[error("error creating setting: {0}")]
    SettingError(#[from] crate::setting::Error),
}

#[derive(Deserialize, Serialize, Debug, Default)]
struct ProfileTable {
    name: Option<String>,
    #[serde(default)]
    settings: Vec<String>,
    #[serde(default)]
    hooks: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
struct ProfileData {
    #[serde(rename = "profile", default)]
    table: ProfileTable,
}

#[derive(Debug)]
pub struct Profile {
    data: ProfileData,
    path: PathBuf,
}

impl Profile {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Profile, Error> {
        let path = find_config_file(force_absolute(path.as_ref().to_owned(), profiles_dir()?))
            .ok_or(Error::FileNotFound {
                name: path
                    .as_ref()
                    .file_name()
                    .ok_or(Error::RootOrPrefix(path.as_ref().to_owned()))?
                    .to_owned(),

                parent: path
                    .as_ref()
                    .parent()
                    .ok_or(Error::RootOrPrefix(path.as_ref().to_owned()))?
                    .to_owned(),
            })?;

        Ok(Profile {
            data: toml::from_str(&std::fs::read_to_string(&path)?)?,
            path,
        })
    }

    fn name(&self) {}

    fn settings(&self) -> Result<Vec<Setting>, Error> {
        Ok(self
            .data
            .table
            .settings
            .iter()
            .map(|s| Setting::new(s))
            .collect::<Result<Vec<_>, crate::setting::Error>>()?)
    }

    fn hooks(&self) {}
}
