use thiserror::Error;

/// This is the error type that gets returned to main.
#[derive(Error, Debug)]
pub enum Error {
    #[error("profile error")]
    ProfileError(#[from] crate::profile::Error),
}