use crate::{dirs::templates_dir, path::force_absolute};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use thiserror::Error;

/// The error type for interacting with templates.
#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to get templates directory")]
    DirError(#[from] crate::dirs::Error),

    #[error("failed to resolve template path")]
    PathError(#[from] crate::path::Error),

    #[error("failed to read template {path:?}\ncaused by: {err}")]
    IOError { path: PathBuf, err: std::io::Error },

    #[error("first line of template file must be a output path, found {line:?} in {path:?}")]
    HeaderError { line: String, path: PathBuf },

    #[error("template file empty {0:?}")]
    EmptyTemplateError(PathBuf),
}

#[derive(Debug)]
/// The container for raw template file data, provides helpful methods for processing templates.
pub struct Template {
    output: PathBuf,
    content: String,
    path: PathBuf,
}

impl Template {
    /// Creates a `Template` using data loaded from the template at `path`.
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Template, Error> {
        let path = force_absolute(path, templates_dir()?);

        if !path.exists() {
            use crate::path::Error;

            Err(Error::FileNotFound {
                name: path
                    .file_name()
                    .ok_or(Error::RootOrPrefix(path.clone()))?
                    .to_owned(),

                path: path
                    .parent()
                    .ok_or(Error::RootOrPrefix(path.clone()))?
                    .to_owned(),
            })?;
        }

        let data = std::fs::read_to_string(&path).map_err(|err| Error::IOError {
            path: path.clone(),
            err,
        })?;

        let mut lines = data.lines().peekable();

        let header = Path::new(
            lines
                .next()
                .ok_or(Error::EmptyTemplateError(path.clone()))?,
        );

        loop {
            if let Some(line) = lines.peek() {
                if line.trim() == "" {
                    lines.next();
                } else {
                    break;
                }
            }
        }

        let content = lines.collect::<Vec<_>>().join("\n");

        Ok(Template {
            output: header.to_owned(),
            content,
            path,
        })
    }

    pub fn generate(&self, _map: HashMap<String, String>) -> Result<String, Error> {
        todo!()
    }
}
