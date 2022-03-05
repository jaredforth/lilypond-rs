//! Abstractions for specifying the rhythm of notes

/// Possible lengths
#[derive(PartialEq, Debug)]
pub enum Length {
    Whole,
    Half,
    Quarter,
    Eighth,
    Sixteenth,
    ThirtySecond,
    SixtyFourth,
    OneTwentyEighth,
}

impl Default for Length {
    fn default() -> Self {
        Length::Quarter
    }
}

/// Type of duration
#[derive(PartialEq, Debug)]
pub enum DurationType {
    Note,
    Rest,
}

impl Default for DurationType {
    fn default() -> Self {
        DurationType::Note
    }
}

/// A duration for a note
#[derive(PartialEq, Debug)]
pub struct Rhythm {
    /// Length e.g. Quarter, Half, or Whole
    pub length: Length,
    /// Whether or not the note is dotted
    pub dotted: bool,
    /// The duration type (Note or Rest)
    pub duration_type: DurationType,
}

impl Rhythm {
    /// Construct a new rhythm
    ///
    /// This will initialize with default values.
    ///
    /// # Usage:
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
    /// Set length for a rhythm
    ///
    /// # Usage:
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
    /// Set whether or not a rhythm is dotted
    ///
    /// # Usage:
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
    /// Set duration type
    ///
    /// # Usage:
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
