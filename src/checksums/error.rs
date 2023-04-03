use std::{error, fmt};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidMessageError(String),
    IncorrectChecksumError(String, String),
    InsufficientMessageError(String),
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidMessageError(msg) => write!(f, "'{msg}' was not a valid message for this checksum"),
            Error::IncorrectChecksumError(got, wanted) => write!(f, "checksum value '{got}' was not correct; expected '{wanted}'"),
            Error::InsufficientMessageError(msg) => write!(f, "'{msg}' did not contain enough data to be a checksummed message"),
        }
    }
}