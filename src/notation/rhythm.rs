//! Abstractions for specifying the rhythm of notes.

/// Possible note values.
#[repr(u16)]
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Length {
    Whole = 1,
    Half = 2,
    Quarter = 4,
    Eighth = 8,
    Sixteenth = 16,
    ThirtySecond = 32,
    SixtyFourth = 64,
    OneTwentyEighth = 128,
}

impl Length {
    /// Return a `u16` representation of the note value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lilypond::notation::rhythm::Length;
    ///
    /// assert_eq!(Length::Whole.as_u16(), 1);
    /// assert_eq!(Length::Half.as_u16(), 2);
    /// assert_eq!(Length::Quarter.as_u16(), 4);
    /// assert_eq!(Length::Eighth.as_u16(), 8);
    /// assert_eq!(Length::Sixteenth.as_u16(), 16);
    /// assert_eq!(Length::ThirtySecond.as_u16(), 32);
    /// assert_eq!(Length::SixtyFourth.as_u16(), 64);
    /// assert_eq!(Length::OneTwentyEighth.as_u16(), 128);
    /// ```
    pub fn as_u16(&self) -> u16 {
        *self as u16
    }
}

impl std::fmt::Display for Length {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.as_u16())
    }
}

impl Default for Length {
    /// Set default length to quarter note.
    fn default() -> Self {
        Length::Quarter
    }
}

/// Type of duration.
#[derive(PartialEq, Debug)]
pub enum DurationType {
    Note,
    Rest,
}

impl Default for DurationType {
    /// Set default duration type to `DurationType::Note`.
    fn default() -> Self {
        DurationType::Note
    }
}

/// A duration for a note.
#[derive(PartialEq, Debug)]
pub struct Rhythm {
    /// Length e.g. Quarter, Half, or Whole.
    pub length: Length,
    /// Whether or not the note is dotted.
    pub dotted: bool,
    /// The duration type (Note or Rest).
    pub duration_type: DurationType,
}

impl Rhythm {
    /// Construct a new rhythm.
    ///
    /// This will initialize with default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use lilypond::notation::rhythm::{Rhythm, Length, DurationType};
    ///
    /// let rhythm = Rhythm::new();
    ///
    /// assert_eq!(rhythm.length, Length::Quarter);
    /// assert_eq!(rhythm.dotted, false);
    /// assert_eq!(rhythm.duration_type, DurationType::Note);
    /// ```
    pub fn new() -> Rhythm {
        Rhythm {
            length: Default::default(),
            dotted: false,
            duration_type: Default::default(),
        }
    }
    /// Set length for a rhythm.
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use lilypond::notation::rhythm::{Rhythm, Length};
    ///
    /// let mut rhythm = Rhythm::new();
    /// rhythm.length(Length::Sixteenth);
    ///
    /// assert_eq!(Length::Sixteenth, rhythm.length);
    /// ```
    pub fn length(&mut self, length: Length) {
        self.length = length;
    }
    /// Set whether or not a rhythm is dotted.
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use lilypond::notation::rhythm::Rhythm;
    ///
    /// let mut rhythm = Rhythm::new();
    /// rhythm.dotted(true);
    ///
    /// assert_eq!(true, rhythm.dotted);
    /// ```
    pub fn dotted(&mut self, is_dotted: bool) {
        self.dotted = is_dotted;
    }
    /// Set duration type.
    ///
    /// # Examples
    ///
    /// ```
    /// use lilypond::notation::rhythm::{Rhythm, DurationType};
    ///
    /// let mut rhythm = Rhythm::new();
    /// rhythm.duration_type(DurationType::Rest);
    ///
    /// assert_eq!(DurationType::Rest, rhythm.duration_type);
    /// ```
    pub fn duration_type(&mut self, duration_type: DurationType) {
        self.duration_type = duration_type;
    }
}

#[cfg(test)]
mod test {
    use crate::notation::rhythm::*;
    #[test]
    fn test_new() {
        let rhythm = Rhythm::new();
        assert_eq!(rhythm.length, Length::Quarter);
        assert_eq!(rhythm.dotted, false);
        assert_eq!(rhythm.duration_type, DurationType::Note);
    }
    #[test]
    fn test_length() {
        let mut rhythm = Rhythm::new();
        rhythm.length(Length::Sixteenth);
        assert_eq!(Length::Sixteenth, rhythm.length);
    }
    #[test]
    fn test_dotted() {
        let mut rhythm = Rhythm::new();
        rhythm.dotted(true);
        assert!(rhythm.dotted);
    }
    #[test]
    fn test_duration_type() {
        let mut rhythm = Rhythm::new();
        rhythm.duration_type(DurationType::Rest);
        assert_eq!(rhythm.duration_type, DurationType::Rest);
    }
}
