use dirs::config_dir as os_config_dir;
use std::fs::create_dir;
use std::path::{Path, PathBuf};
use thiserror::Error;

/// The error type for interacting with config directories.
#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to find the os level config dir")]
    NoOSConfigDir,

    #[error("failed to create directory {0:?}: {1}")]
    FailedToCreateDir(PathBuf, std::io::Error),

    #[error("{0:?} is not a directory")]
    PathIsNotADir(PathBuf),
}

/// A specialized result type for dir operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Helper function for config directories, used to get or create a directory `dir` in `path`.
fn concat(path: impl AsRef<Path>, dir: impl AsRef<Path>) -> Result<PathBuf> {
    let path = path.as_ref().join(dir);

    if !path.exists() {
        create_dir(&path).map_err(|err| Error::FailedToCreateDir(path.clone(), err))?;
        return Ok(path);
    }

    if path.is_dir() {
        Ok(path)
    } else {
        Err(Error::PathIsNotADir(path))
    }
}

/// The `rconfigure` directory in the OS config directory.
pub fn config_dir() -> Result<PathBuf> {
    Ok(concat(
        os_config_dir().ok_or(Error::NoOSConfigDir)?,
        "rconfigure",
    )?)
}

/// The `hooks` directory in the application config.
pub fn hooks_dir() -> Result<PathBuf> {
    Ok(concat(config_dir()?, "hooks")?)
}

/// The `hooks` directory in the application config.
pub fn profiles_dir() -> Result<PathBuf> {
    Ok(concat(config_dir()?, "profiles")?)
}

/// The `scripts` directory in the application config.
pub fn scripts_dir() -> Result<PathBuf> {
    Ok(concat(config_dir()?, "scripts")?)
}

/// The `settings` directory in the application config.
pub fn settings_dir() -> Result<PathBuf> {
    Ok(concat(config_dir()?, "settings")?)
}

/// The `templates` directory in the application config.
pub fn templates_dir() -> Result<PathBuf> {
    Ok(concat(config_dir()?, "templates")?)
}
