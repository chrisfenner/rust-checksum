use crate::checksums::{Checksum, Result, Error::{IncorrectChecksumError, InsufficientMessageError, InvalidMessageError}};

pub struct Luhn {}

impl Checksum for Luhn {
    fn name(&self) -> &'static str {"Luhn"}
    
    fn generate(&self, payload: &str) -> Result<String> {
        let checksum: char = calculate(payload)?;
        let mut result = String::with_capacity(payload.len() + 1);
        result.push_str(payload);
        result.push(checksum);

        Ok(result)
    }

    fn validate(&self, msg: &str) -> Result<String> {
        if msg.len() < 1 {
            return Err(InsufficientMessageError(msg.to_owned()));
        }
        let mut payload: String = msg.to_owned();
        let actual_checksum: char = payload.pop().unwrap();

        // Make sure the checksum format is valid (calculate will check the payload).
        if !is_valid_digit(actual_checksum) {
            return Err(InvalidMessageError(msg.to_owned()));
        }

        let expected_checksum: char;

        match calculate(payload.as_str()) {
            // Special case: calculate complained about the payload; complain about the whole input instead.
            Err(InvalidMessageError(_)) => return Err(InvalidMessageError(msg.to_owned())),
            // Pass any other error through as-is.
            Err(x) => return Err(x),
            Ok(checksum) => expected_checksum = checksum,
        };

        if actual_checksum != expected_checksum {
            return Err(IncorrectChecksumError(actual_checksum.to_string(), expected_checksum.to_string()));
        }

        Ok(payload.to_owned())
    }
}

// Returns the expected checksum value as a one-character String.
fn calculate(msg: &str) -> Result<char> {
    let mut sum: u32 = 0;
    // digits are counted odd/even from the right, but we'll iterate from the left.
    let mut odd_digit = msg.len() % 2 == 1;
    for c in msg.chars() {
        if !is_valid_digit(c) {
            return Err(InvalidMessageError(msg.to_owned()));
        }
        if odd_digit {
            // every odd digit's value is doubled and the digits of that added together.
            let doubled_digits = match 2 * (c.to_digit(10).unwrap()) {
                0 => 0,
                // shortcut for adding up digits: use mod-9 (except returning 9 instead of 0).
                x => (x - 1) % 9 + 1,
            };
            sum = (sum + doubled_digits) % 10;
        } else {
            sum = (sum + (c.to_digit(10).unwrap())) % 10;
        }
        odd_digit = !odd_digit;
    }
    let value: u32 = (10 - sum) % 10;
    
    Ok(char::from_digit(value, 10).unwrap())
}

// Returns whether the given character is valid for this checksum algorithm (i.e., is a decimal digit).
fn is_valid_digit(c: char) -> bool {
    return c >= '0' && c <= '9';
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let checksum = Luhn{};
        assert_eq!(checksum.name(), "Luhn");
    }

    #[test]
    fn test_generate() {
        let checksum = Luhn{};
        assert_eq!(checksum.generate("12345"), Ok("123455".to_owned()));
        assert_eq!(checksum.generate("1000"), Ok("10009".to_owned()));
        assert_eq!(checksum.generate("7992739871"), Ok("79927398713".to_owned()));
        assert_eq!(checksum.generate(""), Ok("0".to_owned()));
        assert_eq!(checksum.generate("foo"), Err(InvalidMessageError("foo".to_owned())));
        assert_eq!(checksum.generate("👍"), Err(InvalidMessageError("👍".to_owned())));
    }

    #[test]
    fn test_validate() {
        let checksum = Luhn{};
        assert_eq!(checksum.validate("123455"), Ok("12345".to_owned()));
        assert_eq!(checksum.validate("123456"), Err(IncorrectChecksumError("6".to_owned(), "5".to_owned())));
        assert_eq!(checksum.validate("10009"), Ok("1000".to_owned()));
        assert_eq!(checksum.validate("10000"), Err(IncorrectChecksumError("0".to_owned(), "9".to_owned())));
        assert_eq!(checksum.validate("79927398713"), Ok("7992739871".to_owned()));
        assert_eq!(checksum.validate("79927398715"), Err(IncorrectChecksumError("5".to_owned(), "3".to_owned())));
        assert_eq!(checksum.validate(""), Err(InsufficientMessageError("".to_owned())));
        assert_eq!(checksum.validate("foo"), Err(InvalidMessageError("foo".to_owned())));
        assert_eq!(checksum.validate("👍"), Err(InvalidMessageError("👍".to_owned())));
        assert_eq!(checksum.validate("👍🐹"), Err(InvalidMessageError("👍🐹".to_owned())));
    }
}