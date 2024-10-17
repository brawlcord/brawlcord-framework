//! Implementations for specialised [`Error`] and [`Result`] types used throughout the library.

use std::error::Error as StdError;
use std::fmt::{self, Display, Formatter};

/// A specialised [`Error`] used throughout the library.
///
/// [`Error`]: StdError
#[derive(Clone, Debug)]
pub enum Error {
    DispatchError(Option<String>),
    ResponseError(Option<String>),
    NoUserFound,
    MiscError(String),
}

impl StdError for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::DispatchError(e) => {
                if let Some(s) = e.as_ref() {
                    write!(f, "{}", s)
                } else {
                    write!(f, "unexpected dispatch error")
                }
            },
            Self::ResponseError(e) => {
                if let Some(s) = e.as_ref() {
                    write!(f, "{}", s)
                } else {
                    write!(f, "unexpected response error")
                }
            },
            Self::NoUserFound => write!(f, "no user found"),
            Self::MiscError(e) => write!(f, "{}", e),
        }
    }
}

/// A specialised [`Result`] type used throughout the library.
///
/// [`Result`]: std::result::Result
pub type Result<T> = std::result::Result<T, Error>;
