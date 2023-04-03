use crate::checksums::error::Result;

pub trait Checksum {
    // Give the name of this checksum algorithm.
    fn name(&self) -> &'static str;

    // Given a payload, generate the checksummed version of the payload.
    fn generate(&self, payload: &str) -> Result<String>;

    // Given a checksummed message, check the checksum and return the inner message if OK.
    fn validate(&self, msg: &str) -> Result<String>;
}