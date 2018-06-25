//! Error handling structures

use core::{convert::From, fmt};

#[derive(Debug)]
pub enum Error {
    Format,
    Unknown,
}

impl From<fmt::Error> for Error {
    fn from(_: fmt::Error) -> Self {
        Error::Format
    }
}

impl From<()> for Error {
    fn from(_: ()) -> Self {
        Error::Unknown
    }
}
