//! Types to represent key signatures.

use std::cmp::Ordering;

#[derive(PartialEq, Debug)]
pub enum Key {
    /// A key signature may either have up to 7 sharps,
    Sharps(u8),
    /// up to 7 flats,
    Flats(u8),
    /// or neither (C Major or atonal)
    None,
}

impl Default for Key {
    /// Default to C major
    fn default() -> Self {
        Key::None
    }
}

#[derive(PartialEq, Debug)]
pub struct KeySignature {
    key: Key,
}

/// Check that NUM is valid number of accidentals in key signature.
///
/// # Errors
///
/// This method returns a `Result` according to whether NUM is a valid number of
/// accidentals. `Ok(num)` is returned when the number is less than or equal to
/// 7, and `Err(num)` when greater than 7.
fn check_num_accidentals(num: u8) -> Result<u8, u8> {
    match num.cmp(&7) {
        Ordering::Greater => Err(num),
        _ => Ok(num),
    }
}

impl KeySignature {
    /// Attempt to construct a new `KeySignature` with the input [`Key`].
    ///
    /// # Errors
    ///
    /// Returns a `Result` according to whether the input [`Key`] object is
    /// valid (i.e. `Sharp` and `Flat` signatures may have between 0 and 7
    /// accidentals). On a success, returns `Ok(KeySignature)`, and on a
    /// failure, returns `Err(String)`, where the `String` is the error message.
    ///
    /// # Examples
    ///
    /// A successful initialization:
    ///
    /// ```rust
    /// use lilypond::notation::key::{Key, KeySignature};
    ///
    /// let key = KeySignature::new(Key::None).unwrap();
    /// assert_eq!(key.get_key(), &Key::None);
    /// ```
    ///
    /// An unsuccessful initialization:
    ///
    /// ```rust
    /// use lilypond::notation::key::{Key, KeySignature};
    ///
    /// let key = KeySignature::new(Key::Flats(10));
    /// assert_eq!(key, Err(String::from("Invalid number of accidentals 10.")));
    /// ```
    pub fn new(key: Key) -> Result<KeySignature, String> {
        match key {
            Key::None => Ok(KeySignature { key }),
            Key::Flats(n) | Key::Sharps(n) => match check_num_accidentals(n) {
                Ok(_) => Ok(KeySignature { key }),
                Err(e) => Err(format!("Invalid number of accidentals {}.", e)),
            },
        }
    }
    /// Return a reference to the [`Key`] object of the current `KeySignature`.
    pub fn get_key(&self) -> &Key {
        &self.key
    }
}

#[cfg(test)]
mod tests {
    use crate::notation::key::*;
    #[test]
    fn test_new() {
        let key = KeySignature::new(Key::None).unwrap();
        assert_eq!(key.get_key(), &Key::None);
    }
    #[test]
    fn test_check_num_accidentals() {
        // Test if check_num_accidentals() returns proper value
        let num_accidentals = check_num_accidentals(2);
        assert_eq!(num_accidentals, Ok(2));
    }
    #[test]
    fn test_check_num_accidentals_error() {
        // Test if check_num_accidentals() returns an error with improper input
        let num_accidentals = check_num_accidentals(9);
        assert_eq!(num_accidentals, Err(9));
    }
    #[test]
    #[should_panic]
    #[allow(unused_variables)]
    fn test_new_error() {
        let key = KeySignature::new(Key::Sharps(8)).unwrap();
    }
}
