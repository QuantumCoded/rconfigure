use dirs::config_dir as os_config_dir;
use std::fs::create_dir;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("could not find the os level config dir")]
    NoOSConfigDir,

    #[error("failed to create directory {0:?}, {1}")]
    FailedToCreateDir(PathBuf, std::io::Error),

    #[error("the path {0:?} was expected to be a directory, but is not")]
    PathIsNotADir(PathBuf),
}

type DirResult = Result<PathBuf, Error>;

fn concat<P: AsRef<Path>>(path: PathBuf, dir: P) -> DirResult {
    let path = path.join(dir);

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

pub fn config_dir() -> DirResult {
    Ok(concat(
        os_config_dir().ok_or(Error::NoOSConfigDir)?,
        "rconfigure",
    )?)
}

pub fn hooks_dir() -> DirResult {
    Ok(concat(
        config_dir()?,
        "hooks",
    )?)
}

pub fn profiles_dir() -> DirResult {
    Ok(concat(
        config_dir()?,
        "profiles",
    )?)
}

pub fn scripts_dir() -> DirResult {
    Ok(concat(
        config_dir()?,
        "scripts",
    )?)
}

pub fn settings_dir() -> DirResult {
    Ok(concat(
        config_dir()?,
        "settings",
    )?)
}

pub fn templates_dir() -> DirResult {
    Ok(concat(
        config_dir()?,
        "templates",
    )?)
}
