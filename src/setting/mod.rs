use crate::dirs::settings_dir;
use crate::path::{find_config_file, force_absolute};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    DirError(#[from] crate::dirs::Error),

    #[error("there is no file named {name:?} or {name:?}.toml in {parent:?}")]
    FileNotFound { name: OsString, parent: PathBuf },

    #[error("setting paths must not be '/' or terminate in '..', found: {0:?}")]
    RootOrPrefix(PathBuf),

    #[error("io error when reading setting: {0}")]
    IOError(#[from] std::io::Error),

    #[error("error deserializing setting: {0}")]
    DeserializeError(#[from] toml::de::Error),
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum Value {
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Array(Vec<Value>),
    Script { script: String, value: Box<Value> },
}

#[derive(Deserialize, Serialize, Debug, Default)]
struct SettingTable {
    name: Option<String>,
    #[serde(default)]
    hooks: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
struct SettingData {
    #[serde(rename = "setting", default)]
    table: SettingTable,
    #[serde(default)]
    global: HashMap<String, Value>,
    #[serde(flatten)]
    targets: HashMap<PathBuf, HashMap<String, Value>>,
}

#[derive(Debug)]
pub struct Setting {
    data: SettingData,
    path: PathBuf,
}

impl Setting {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Setting, Error> {
        let path = find_config_file(force_absolute(path.as_ref().to_owned(), settings_dir()?))
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

        Ok(Setting {
            data: toml::from_str(&std::fs::read_to_string(&path)?)?,
            path,
        })
    }
}
