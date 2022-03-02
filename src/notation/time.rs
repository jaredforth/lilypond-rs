//! Abstractions for defining time signatures

#[derive(PartialEq, Debug)]
pub struct TimeSignature {
    /// The number of beats per measure
    pub num_beats: u8,
    /// The fraction of a whole note representing the length of a beat
    pub duration: u8,
}

impl Default for TimeSignature {
    fn default() -> Self {
        TimeSignature {
            num_beats: 4,
            duration: 4,
        }
    }
}

impl TimeSignature {
    /// Construct a new time signature
    ///
    /// This will initialize a new time signature with the default of 4/4.
    ///
    /// # Usage:
    ///
    /// ```
    /// use lilypond::notation::time::TimeSignature;
    ///
    /// let mut time_signature = TimeSignature::new();
    ///
    /// assert_eq!(time_signature.num_beats, 4);
    /// assert_eq!(time_signature.duration, 4);
    /// ```
    pub fn new() -> TimeSignature {
        Default::default()
    }
    /// Set number of beats
    ///
    /// This will set the numerator of a time signature.
    ///
    /// # Usage:
    ///
    /// ```
    /// use lilypond::notation::time::TimeSignature;
    ///
    /// let mut time_signature = TimeSignature::new();
    /// time_signature.num_beats(7);
    ///
    /// assert_eq!(time_signature.num_beats, 7);
    /// assert_eq!(time_signature.duration, 4);
    /// ```
    pub fn num_beats(&mut self, numerator: u8) {
        self.num_beats = numerator;
    }
    /// Set duration of beat
    ///
    /// This will set the denominator of a time signature.
    ///
    /// # Usage:
    ///
    /// ```
    /// use lilypond::notation::time::TimeSignature;
    ///
    /// let mut time_signature = TimeSignature::new();
    /// time_signature.duration(8);
    ///
    /// assert_eq!(time_signature.num_beats, 4);
    /// assert_eq!(time_signature.duration, 8);
    /// ```
    pub fn duration(&mut self, denominator: u8) {
        self.duration = denominator;
    }
}
