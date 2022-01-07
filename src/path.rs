use std::path::{Path, PathBuf};

pub fn force_absolute<P: AsRef<Path>>(path: P, pre: P) -> PathBuf {
    if path.as_ref().is_absolute() {
        path.as_ref().to_owned()
    } else {
        pre.as_ref().join(path)
    }
}

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