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
    OneTwentyEighth
}

impl Default for Length {
    fn default() -> Self { Length::Quarter }
}

/// Type of duration
#[derive(PartialEq, Debug)]
pub enum NoteDurationType {
    Note,
    Rest
}

impl Default for NoteDurationType {
    fn default() -> Self { NoteDurationType::Note }
}

/// A duration for a note
#[derive(PartialEq, Debug)]
pub struct NoteDuration {
    /// Length e.g. Quarter, Half, or Whole
    pub length: Length,
    /// Whether or not the note is dotted
    pub dotted: bool,
    /// The duration type (Note or Rest)
    pub duration_type: NoteDurationType
}

impl NoteDuration {
    /// Construct a new duration
    ///
    /// This will initialize with default values.
    ///
    /// # Usage:
    ///
    /// ```
    /// use lilypond::notation::rhythm::{NoteDuration, Length, NoteDurationType};
    ///
    /// let duration = NoteDuration::new();
    ///
    /// assert_eq!(duration.length, Length::Quarter);
    /// assert_eq!(duration.dotted, false);
    /// assert_eq!(duration.duration_type, NoteDurationType::Note);
    /// ```
    pub fn new() -> NoteDuration {
        NoteDuration {
            length: Default::default(),
            dotted: false,
            duration_type: Default::default()
        }
    }
}
