use crate::bool_false_as_none;
use crate::dirs::hooks_dir;
use crate::path::{find_config_file, force_absolute};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ffi::OsString;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use thiserror::Error;

/// The error type for interacting with hooks.
#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    DirError(#[from] crate::dirs::Error),

    #[error("there is no file named {name:?} or {name:?}.toml in {parent:?}")]
    FileNotFound { name: OsString, parent: PathBuf },

    #[error("profile paths must not be '/' or terminate in '..', found: {0:?}")]
    RootOrPrefix(PathBuf),

    #[error("io error when reading hook: {0}")]
    IOError(#[from] std::io::Error),

    #[error("error deserializing hook: {0}")]
    DeserializeError(#[from] toml::de::Error),
}

/// Implementation of `the bool_false_as_none` serializer and deserializer.
#[derive(Deserialize, Serialize, Clone, Debug)]
struct StringOrFalseAsNone(#[serde(with = "bool_false_as_none")] Option<String>);

impl Deref for StringOrFalseAsNone {
    type Target = Option<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// The deserialized representation of a shell command.
#[derive(Deserialize, Serialize, Debug)]
struct HookCommand {
    cmd: String,
    cwd: Option<PathBuf>,
    #[serde(default)]
    args: Vec<String>,
    #[serde(default)]
    env: HashMap<String, StringOrFalseAsNone>,
}

/// The deserialized representation of one or more shell commands.
#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
enum HookData {
    Single(HookCommand),
    Multiple(Vec<HookCommand>),
}

/// The deserialized representation of a hook used in a config file.
#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum Hook {
    BeforeAfter { before: String, after: String },
    Before { before: String },
    After { after: String },
}

/// The deserialized representation of a hooks file.
#[derive(Deserialize, Serialize, Debug)]
struct HooksData {
    #[serde(flatten)]
    hooks: HashMap<String, HookData>,
}

/// The container for raw hooks file data, provides helpful methods for processing hooks.
pub struct Hooks {
    data: HooksData,
    path: PathBuf,
}

impl Hooks {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Hooks, Error> {
        let path = find_config_file(force_absolute(path.as_ref().to_owned(), hooks_dir()?)).ok_or(
            Error::FileNotFound {
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
            },
        )?;

        Ok(Hooks {
            data: toml::from_str(&std::fs::read_to_string(&path)?)?,
            path,
        })
    }
}
