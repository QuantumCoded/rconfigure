use crate::path::resolve;
use crate::{dirs::profiles_dir, hooks::Hook, setting::Setting, value::Value};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use thiserror::Error;

/// The error type for interacting with profiles.
#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to get profiles directory")]
    DirError(#[from] crate::dirs::Error),

    #[error("failed to resolve profile path")]
    PathError(#[from] crate::path::Error),

    #[error("failed to create setting")]
    SettingError(#[from] crate::setting::Error),

    #[error("failed to read profile {path:?}\ncaused by: {err}")]
    IOError { path: PathBuf, err: std::io::Error },

    #[error("failed to deserialize profile {path:?}\ncaused by: {err}")]
    DeserializeError { path: PathBuf, err: toml::de::Error },
}

/// The `[profile]` table in a profile file.
#[derive(Deserialize, Serialize, Debug, Default)]
struct ProfileTable {
    name: Option<String>,
    #[serde(default)]
    settings: Vec<String>,
    hooks: Option<PathBuf>,
    #[serde(default)]
    hook: Vec<Hook>,
}

/// The `[values]` table in a profile file.
#[derive(Deserialize, Serialize, Debug, Default)]
struct ProfileValues {
    #[serde(flatten, rename = "values")]
    map: HashMap<String, Value>,
}

/// The deserialized representation of a profile file.
#[derive(Deserialize, Serialize, Debug)]
struct ProfileData {
    #[serde(rename = "profile", default)]
    table: ProfileTable,
    #[serde(default)]
    values: ProfileValues,
}

/// The container for raw profile file data, provides helpful methods for processing profiles.
#[derive(Debug)]
pub struct Profile {
    data: ProfileData,
    path: PathBuf,
}

impl Profile {
    /// Creates a `Profile` using data loaded from the profile at `path`.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Profile, Error> {
        let path = resolve(path, profiles_dir()?)?;

        Ok(Profile {
            data: toml::from_str(&std::fs::read_to_string(&path).map_err(|err| {
                Error::IOError {
                    path: path.clone(),
                    err,
                }
            })?)
            .map_err(|err| Error::DeserializeError {
                path: path.clone(),
                err,
            })?,
            path,
        })
    }

    /// The name of the profile.
    fn name(&self) {}

    /// The settings in the profile.
    pub fn settings(&self) -> Result<Vec<Setting>, Error> {
        Ok(self
            .data
            .table
            .settings
            .iter()
            .map(|s| Setting::new(s))
            .collect::<Result<Vec<_>, crate::setting::Error>>()?)
    }

    /// The hooks file in the profile.
    fn hooks(&self) {}

    /// The hooked events from the hooks file.
    fn hooked(&self) {}
}
