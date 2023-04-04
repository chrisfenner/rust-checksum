use thiserror::Error;

pub type Result<T> = std::result::Result<T, ChecksumError>;

#[derive(Error, Debug, PartialEq)]
pub enum ChecksumError {
    #[error("'{0}' was not a valid message for this checksum")]
    InvalidMessageError(String),
    #[error("checksum value '{got}' was not correct; expected '{wanted}'")]
    IncorrectChecksumError {
        got: String,
        wanted: String,
    },
    #[error("'{0}' did not contain enough data to be a checksummed message")]
    InsufficientMessageError(String),
}
