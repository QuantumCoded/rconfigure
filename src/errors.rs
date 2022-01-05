use std::path::PathBuf;

use thiserror::Error;
use crate::setting::Setting;

#[derive(Error, Debug)]
pub enum ProfileError {
    #[error("setting target conflict")]
    SettingConflict {
        setting1: Setting,
        setting2: Setting,
    },

    #[error("there was an error reading the profile {0:?}")]
    ErrorReadingProfile(PathBuf, std::io::Error),

    #[error("there was an error writing the profile {0:?}")]
    ErrorWritingProfile(PathBuf, std::io::Error),

    // #[error("there was an error serializing the profile {0:?}")]
    // ErrorSerializingProfile(PathBuf, toml::ser::Error),
}

#[derive(Error, Debug)]
pub enum ActiveError {}

#[derive(Error, Debug)]
pub enum CrateError {
    #[error("error in profile module")]
    ProfileError(#[from] ProfileError),

    #[error("there was a file system error")]
    FileSystemError(#[from] std::io::Error),

    #[error("there was an error serializing a profile")]
    TomlSerializationError(#[from] toml::ser::Error),

    #[error("there was an error deserializing a profile")]
    TomlDeserializiationError(#[from] toml::de::Error),

    #[error("could not find config directory")]
    NoConfigDir,
}