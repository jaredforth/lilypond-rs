//! A single note type

use crate::notation::pitch::{NoteName, Pitch};
use crate::notation::rhythm::NoteDuration;

/// A note with rhythm and pitch
pub struct Note {
    pub pitch: Pitch,
    pub rhythm: NoteDuration,
}

/// TODO shorthand for creating a new note with pitch and duration
/// TODO also need to create staff, time signature, key signature, chord
/// before we have a minimum viable product
impl Note {
    /// Construct a new note
    ///
    /// This will take a note name and initialize a note with default octave and
    /// duration.
    ///
    /// # Usage:
    ///
    /// ```
    /// use lilypond::notation::note::Note;
    /// use lilypond::notation::pitch::{Pitch, NoteName, Accidental, Octave};
    /// use lilypond::notation::rhythm::{NoteDuration, Length, NoteDurationType};
    ///
    /// let note = Note::new(NoteName::A);
    ///
    /// assert_eq!(note.pitch.note, NoteName::A);
    /// assert_eq!(note.pitch.octave, Octave::S3);
    /// assert_eq!(note.pitch.accidental, Accidental::None);
    /// assert_eq!(note.rhythm.length, Length::Quarter);
    /// assert_eq!(note.rhythm.dotted, false);
    /// assert_eq!(note.rhythm.duration_type, NoteDurationType::Note);
    /// ```
    pub fn new(note: NoteName) -> Note {
        Note {
            pitch: Pitch::new(note),
            rhythm: NoteDuration::new(),
        }
    }
}
