//! Abstract types for musical note, describing pitch and rhythm.

use crate::lilypond_objects::lilypond_note::LilyPondNote;
use crate::notation::pitch::{NoteName, Pitch};
use crate::notation::rhythm::Rhythm;
use crate::note_from_lilypond;

/// A note with rhythm and pitch
#[derive(Debug, PartialEq)]
pub struct Note {
    pub pitch: Pitch,
    pub rhythm: Rhythm,
}

/// - TODO shorthand for creating a new note with pitch and duration
/// - TODO also need to create staff, time signature, key signature, chord
/// before we have a minimum viable product
impl Note {
    /// Construct a new note
    ///
    /// This will take a note name and initialize a note with default octave and
    /// duration.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lilypond::notation::note::Note;
    /// use lilypond::notation::pitch::{Pitch, NoteName, Accidental, Octave};
    /// use lilypond::notation::rhythm::{Rhythm, Length, DurationType, Dots};
    ///
    /// let note = Note::new(NoteName::A);
    ///
    /// assert_eq!(note.pitch.note_name, NoteName::A);
    /// assert_eq!(note.pitch.octave, Octave::S3);
    /// assert_eq!(note.pitch.accidental, Accidental::None);
    /// assert_eq!(note.rhythm.length, Length::Quarter);
    /// assert_eq!(note.rhythm.dots, Dots::new(0));
    /// assert_eq!(note.rhythm.duration_type, DurationType::Note);
    /// ```
    pub fn new(note_name: NoteName) -> Note {
        Note {
            pitch: Pitch::new(note_name),
            rhythm: Rhythm::new(),
        }
    }
}

impl std::convert::TryFrom<&LilyPondNote> for Note {
    type Error = String;

    /// Translate a lilypond note string to a note object.
    ///
    /// # Errors
    ///
    /// Returns a `Result` according to whether or not the conversion was
    /// successful. On a success, returns `Ok(Note)`, and on a failure, returns
    /// `Err(String)` where the `String` is the error message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::convert::TryFrom;
    /// use lilypond::lilypond_objects::lilypond_note::LilyPondNote;
    /// use lilypond::notation::note::Note;
    /// use lilypond::notation::pitch::{Pitch, NoteName, Accidental, Octave};
    /// use lilypond::notation::rhythm::{Rhythm, Length, DurationType, Dots};
    ///
    /// let ly_note = LilyPondNote::new("af,8.").unwrap();
    /// let note = Note::try_from(&ly_note).unwrap();
    ///
    /// assert_eq!(note.pitch.note_name, NoteName::A);
    /// assert_eq!(note.pitch.octave, Octave::S2);
    /// assert_eq!(note.pitch.accidental, Accidental::Flat);
    /// assert_eq!(note.rhythm.length, Length::Eighth);
    /// assert_eq!(note.rhythm.dots, Dots::new(1));
    /// assert_eq!(note.rhythm.duration_type, DurationType::Note);
    /// ```
    fn try_from(note: &LilyPondNote) -> Result<Self, Self::Error> {
        note_from_lilypond(note)
    }
}

#[cfg(test)]
mod tests {
    use crate::lilypond_objects::lilypond_note::LilyPondNote;
    use crate::notation::note::Note;
    use crate::notation::pitch::{Accidental, NoteName, Octave};
    use crate::notation::rhythm::{Dots, DurationType, Length};
    use std::convert::TryFrom;
    #[test]
    fn test_from_lilypond_note() {
        let ly_note = LilyPondNote::new("a4").unwrap();
        let note = Note::try_from(&ly_note).unwrap();
        assert_eq!(note.pitch.note_name, NoteName::A);
        assert_eq!(note.pitch.octave, Octave::S3);
        assert_eq!(note.pitch.accidental, Accidental::None);
        assert_eq!(note.rhythm.length, Length::Quarter);
        assert_eq!(note.rhythm.dots, Dots::new(0));
        assert_eq!(note.rhythm.duration_type, DurationType::Note);
    }
}
