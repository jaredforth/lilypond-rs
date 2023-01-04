//! Types to represent key signatures

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
    pub key: Key,
}

use crate::notation::key;

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
    /// Construct a new key signature.
    ///
    /// This will initialize a C Major key signature.
    ///
    /// # Examples
    ///
    /// ```
    /// use lilypond::notation::key::{Key, KeySignature};
    ///
    /// let key = KeySignature::new();
    ///
    /// assert_eq!(key.key, Key::None);
    /// ```
    pub fn new() -> KeySignature {
        KeySignature {
            key: Default::default(),
        }
    }
    /// Assign key signature.
    ///
    /// # Panics
    ///
    /// This method panics when passed a key signature with a number of sharps
    /// or flats greater than seven.
    ///
    /// # Examples
    ///
    /// ```
    /// use lilypond::notation::key::{Key, KeySignature};
    ///
    /// let mut key = KeySignature::new();
    /// key.set_key(Key::Sharps(2));
    ///
    /// assert_eq!(key.key, Key::Sharps(2));
    /// ```
    pub fn set_key(&mut self, signature: Key) {
        self.key = match signature {
            Key::None => signature,
            Key::Sharps(num) => Key::Sharps(key::check_num_accidentals(num).unwrap()),
            Key::Flats(num) => Key::Flats(key::check_num_accidentals(num).unwrap()),
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::notation::key::*;
    #[test]
    fn test_new() {
        let key = KeySignature::new();
        assert_eq!(key.key, Key::None);
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
        #[allow(unused_variables)]
        let num_accidentals = check_num_accidentals(9);
        assert_eq!(num_accidentals, Err(9));
    }
    #[test]
    fn test_set_key() {
        let mut key = KeySignature::new();
        key.set_key(Key::Sharps(2));
        assert_eq!(key.key, Key::Sharps(2));
    }
    #[test]
    #[should_panic]
    fn test_set_key_panic() {
        // Test if set_key() panics with bad argument
        let mut key = KeySignature::new();
        key.set_key(Key::Sharps(8));
    }
}
