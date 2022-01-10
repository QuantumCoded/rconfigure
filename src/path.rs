use std::ffi::OsString;
use std::path::{Path, PathBuf};
use thiserror::Error;

/// The error type for interacting with config directory paths.
#[derive(Error, Debug)]
pub enum Error {
    #[error("there is no file named {name:?} in {path:?}")]
    FileNotFound { name: OsString, path: PathBuf },

    #[error("there is no file named {name:?} or {name_toml:?} in {path:?}")]
    FilesNotFound {
        name: OsString,
        name_toml: OsString,
        path: PathBuf,
    },

    #[error("paths must not be '/' or terminate in '..', found: {0:?}")]
    RootOrPrefix(PathBuf),
}

/// Resolves a user provided `path` to an absolute `PathBuf` using the absolute prefix `pre` if
/// necessary. Changes
///
/// Checks if `path` is absolute, if it's not, prepends the path `pre` \
/// Note: `pre` must be absolute.
///
/// Looks for the path specified, if it doesn't exist, checks the same path with `.toml` extension. \
/// Gives back the path that exists.
///
/// # Errors
///
/// ## FileNotFound
/// Happens when `path` has `.toml` extension and does not exist.
///
/// ## FilesNotFound
/// Happens when `path` does not have `.toml` extension and neither it nor it with `.toml`
/// **appended** to the file name (not with the current extension changed).
pub fn resolve(path: impl AsRef<Path>, pre: impl AsRef<Path>) -> Result<PathBuf, Error> {
    let path = force_absolute(path.as_ref().to_owned(), pre);
    let path = find_config_file(&path).ok_or(match path.extension() {
        Some(s) if s == OsString::from("toml") => Error::FileNotFound {
            name: path
                .file_name()
                .ok_or(Error::RootOrPrefix(path.clone()))?
                .to_owned(),

            path: path
                .parent()
                .ok_or(Error::RootOrPrefix(path.clone()))?
                .to_owned(),
        },

        Some(s) => {
            let mut str = OsString::new();
            str.push(s);
            str.push(".toml");

            Error::FilesNotFound {
                name: path
                    .file_name()
                    .ok_or(Error::RootOrPrefix(path.clone()))?
                    .to_owned(),

                name_toml: path
                    .with_extension(str)
                    .file_name()
                    .ok_or(Error::RootOrPrefix(path.clone()))?
                    .to_owned(),

                path: path
                    .parent()
                    .ok_or(Error::RootOrPrefix(path.clone()))?
                    .to_owned(),
            }
        }

        None => {
            Error::FilesNotFound {
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
            }
        }
    })?;

    Ok(path)
}

/// Checks if `path` is absolute, if it's not, prepends the path `pre` and returns. \
/// Note: `pre` must be absolute.
pub fn force_absolute(path: impl AsRef<Path>, pre: impl AsRef<Path>) -> PathBuf {
    assert!(pre.as_ref().is_absolute(), "the pre path must be absolute");

    if path.as_ref().is_absolute() {
        path.as_ref().to_owned()
    } else {
        pre.as_ref().join(path)
    }
}

/// Looks for the path specified, if it doesn't exist, checks the same path with `.toml` extension
/// appened (not with the current extension changed).
///
/// If either exist, returns `Some(PathBuf)` for the first found, otherwise returns `None`.
fn find_config_file<P: AsRef<Path>>(path: P) -> Option<PathBuf> {
    let path = path.as_ref();

    if path.exists() {
        Some(path.to_owned())
    } else {
        let path = match path.extension() {
            Some(s) => {
                let mut str = OsString::new();
                str.push(s);
                str.push(".toml");

                path.with_extension(str)
            }

            None => path.with_extension("toml"),
        };

        if path.exists() {
            Some(path)
        } else {
            None
        }
    }
}
