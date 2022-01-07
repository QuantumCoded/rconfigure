use std::path::{Path, PathBuf};

pub fn force_absolute<P: AsRef<Path>>(path: P, pre: P) -> PathBuf {
    if path.as_ref().is_absolute() {
        path.as_ref().to_owned()
    } else {
        pre.as_ref().join(path)
    }
}