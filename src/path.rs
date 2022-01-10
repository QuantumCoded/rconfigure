use std::ffi::OsString;
use std::path::{Path, PathBuf};
use thiserror::Error;

/// The error type for interacting with config directory paths.
#[derive(Error, Debug)]
pub enum Error {
    #[error("there is no file named {name:?} or {name_toml:?} in {path:?}")]
    FileNotFound {
        name: OsString,
        name_toml: OsString,
        path: PathBuf,
    },

    #[error("paths must not be '/' or terminate in '..', found: {0:?}")]
    RootOrPrefix(PathBuf),
}

/// Resolves a user provided `path` to an absolute `PathBuf` using the absolute prefix `pre` if
/// necessary. \
/// Note: `pre` must be absolute.
pub fn resolve(path: impl AsRef<Path>, pre: impl AsRef<Path>) -> Result<PathBuf, Error> {
    let path = force_absolute(path.as_ref().to_owned(), pre);
    let path = find_config_file(&path).ok_or(Error::FileNotFound {
        name: path
            .file_name()
            .ok_or(Error::RootOrPrefix(path.clone()))?
            .to_owned(),

        name_toml: path
            .with_extension("toml")
            .file_name()
            .ok_or(Error::RootOrPrefix(path.clone()))?
            .to_owned(),

        path: path
            .parent()
            .ok_or(Error::RootOrPrefix(path.clone()))?
            .to_owned(),
    })?;

    Ok(path)
}

/// Checks if `path` is absolute, if it's not, prepends the path `pre` and returns. \
/// Note: `pre` must be absolute.
fn force_absolute(path: impl AsRef<Path>, pre: impl AsRef<Path>) -> PathBuf {
    assert!(pre.as_ref().is_absolute(), "the pre path must be absolute");

    if path.as_ref().is_absolute() {
        path.as_ref().to_owned()
    } else {
        pre.as_ref().join(path)
    }
}

/// Looks for the path specified, if it doesn't exist, checks the same path with `.toml` extension.
/// If either exist, returns `Some(PathBuf)` for the first found, otherwise returns `None`.
fn find_config_file<P: AsRef<Path>>(path: P) -> Option<PathBuf> {
    let path = path.as_ref();

    if path.exists() {
        Some(path.to_owned())
    } else {
        let path = path.with_extension("toml");

        if path.exists() {
            Some(path)
        } else {
            None
        }
    }
}
