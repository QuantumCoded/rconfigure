use crate::bool_false_as_none;
use crate::dirs::hooks_dir;
use crate::path::resolve;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use thiserror::Error;

/// The error type for interacting with hooks.
#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to get hooks directory")]
    DirError(#[from] crate::dirs::Error),

    #[error("failed to resolve path hook path")]
    PathError(#[from] crate::path::Error),

    #[error("failed to read hook {path:?}\ncause by: {err}")]
    IOError { path: PathBuf, err: std::io::Error },

    #[error("failed to deserialize hook {path:?}\ncaused by: {err}")]
    DeserializeError { path: PathBuf, err: toml::de::Error },
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
        let path = resolve(path, hooks_dir()?)?;

        Ok(Hooks {
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
}
