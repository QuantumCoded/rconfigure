use std::path::{Path, PathBuf};

/// Checks if `path` is absolute, if it's not, prepends the path `pre` and returns. \
/// Note: `pre` must be absolute.
pub fn force_absolute<P: AsRef<Path>>(path: P, pre: P) -> PathBuf {
    assert!(pre.as_ref().is_absolute(), "the pre path must be absolute");

    if path.as_ref().is_absolute() {
        path.as_ref().to_owned()
    } else {
        pre.as_ref().join(path)
    }
}

/// Looks for the path specified, if it doesn't exist, checks the same path with `.toml` extension.
/// If either exist, returns `Some(PathBuf)` for the first found, otherwise returns `None`.
pub fn find_config_file<P: AsRef<Path>>(path: P) -> Option<PathBuf> {
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
