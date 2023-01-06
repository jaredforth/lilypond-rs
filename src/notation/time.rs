//! Abstractions for defining time signatures.

#[derive(PartialEq, Debug)]
pub struct TimeSignature {
    /// The number of beats per measure.
    pub numerator: u8,
    /// The fraction of a whole note representing the length of a beat.
    pub denominator: u8,
}

impl Default for TimeSignature {
    /// Initialize default time signature to 4/4.
    fn default() -> Self {
        TimeSignature {
            numerator: 4,
            denominator: 4,
        }
    }
}

impl TimeSignature {
    /// Construct a new time signature.
    ///
    /// This will initialize a new time signature with the default of 4/4.
    ///
    /// # Examples
    ///
    /// ```
    /// use lilypond::notation::time::TimeSignature;
    ///
    /// let mut time_signature = TimeSignature::new();
    ///
    /// assert_eq!(time_signature.numerator, 4);
    /// assert_eq!(time_signature.denominator, 4);
    /// ```
    pub fn new() -> TimeSignature {
        Default::default()
    }
    /// Set number of beats.
    ///
    /// This will set the numerator of a time signature.
    ///
    /// # Examples
    ///
    /// ```
    /// use lilypond::notation::time::TimeSignature;
    ///
    /// let mut time_signature = TimeSignature::new();
    /// time_signature.numerator(7);
    ///
    /// assert_eq!(time_signature.numerator, 7);
    /// assert_eq!(time_signature.denominator, 4);
    /// ```
    pub fn numerator(&mut self, numerator: u8) {
        self.numerator = numerator;
    }
    /// Set length of beat.
    ///
    /// This will set the denominator of a time signature.
    ///
    /// # Examples
    ///
    /// ```
    /// use lilypond::notation::time::TimeSignature;
    ///
    /// let mut time_signature = TimeSignature::new();
    /// time_signature.denominator(8);
    ///
    /// assert_eq!(time_signature.numerator, 4);
    /// assert_eq!(time_signature.denominator, 8);
    /// ```
    pub fn denominator(&mut self, denominator: u8) {
        self.denominator = denominator;
    }
}

#[cfg(test)]
mod test {
    use crate::notation::time::TimeSignature;
    #[test]
    fn test_new() {
        let time_signature = TimeSignature::new();
        assert_eq!(time_signature.numerator, 4);
        assert_eq!(time_signature.denominator, 4);
    }
    #[test]
    fn test_numerator() {
        let mut time_signature = TimeSignature::new();
        time_signature.numerator(7);
        assert_eq!(time_signature.numerator, 7);
    }
    #[test]
    fn test_denominator() {
        let mut time_signature = TimeSignature::new();
        time_signature.denominator(8);
        assert_eq!(time_signature.denominator, 8);
    }
}
