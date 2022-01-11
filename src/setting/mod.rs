use crate::dirs::{settings_dir, templates_dir};
use crate::path::{force_absolute, resolve};
use crate::value::Value;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use thiserror::Error;

mod compose_template_map;

/// The error type for interacting with settings.
#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to get settings directory")]
    DirError(#[from] crate::dirs::Error),

    #[error("failed to resolve path setting path")]
    PathError(#[from] crate::path::Error),

    #[error("failed to read setting {path:?}\ncaused by: {err}")]
    IOError { path: PathBuf, err: std::io::Error },

    #[error("failed to deserialize setting {path:?}\ncaused by: {err}")]
    DeserializeError { path: PathBuf, err: toml::de::Error },
}

/// The `[setting]` table in a setting file.
#[derive(Deserialize, Serialize, Debug, Default)]
struct SettingTable {
    name: Option<String>,
    hooks: Option<PathBuf>,
    #[serde(default)]
    hook: Vec<String>,
}

/// The deserialized representation of a setting file.
#[derive(Deserialize, Serialize, Debug)]
struct SettingData {
    #[serde(rename = "setting", default)]
    table: SettingTable,
    #[serde(default)]
    global: HashMap<String, Value>,
    #[serde(flatten)]
    targets: HashMap<PathBuf, HashMap<String, Value>>,
}

/// The container for raw setting file data, provides helpful methods for processing settings.
#[derive(Debug)]
pub struct Setting {
    data: SettingData,
    path: PathBuf,
}

impl Setting {
    /// Creates a `Setting` using data loaded from the setting at `path`.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Setting, Error> {
        let path = resolve(path, settings_dir()?)?;

        Ok(Setting {
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

    /// Gets the setting targets with absolute paths.
    pub fn targets(&self) -> Result<HashMap<PathBuf, HashMap<String, Value>>, Error> {
        Ok(self
            .data
            .targets
            .iter()
            .map(
                |(p, v)| -> Result<(PathBuf, HashMap<String, Value>), Error> {
                    Ok((force_absolute(p, templates_dir()?), v.to_owned()))
                },
            )
            .collect::<Result<_, Error>>()?)
    }
}
