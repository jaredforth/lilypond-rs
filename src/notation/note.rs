//! Abstract types for musical note, describing pitch and rhythm

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
    /// ```
    /// use lilypond::notation::note::Note;
    /// use lilypond::notation::pitch::{Pitch, NoteName, Accidental, Octave};
    /// use lilypond::notation::rhythm::{Rhythm, Length, DurationType};
    ///
    /// let note = Note::new(NoteName::A);
    ///
    /// assert_eq!(note.pitch.note_name, NoteName::A);
    /// assert_eq!(note.pitch.octave, Octave::S3);
    /// assert_eq!(note.pitch.accidental, Accidental::None);
    /// assert_eq!(note.rhythm.length, Length::Quarter);
    /// assert_eq!(note.rhythm.dotted, false);
    /// assert_eq!(note.rhythm.duration_type, DurationType::Note);
    /// ```
    pub fn new(note_name: NoteName) -> Note {
        Note {
            pitch: Pitch::new(note_name),
            rhythm: Rhythm::new(),
        }
    }
}

impl From<&LilyPondNote> for Note {
    /// Translate a lilypond note string to a note object.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lilypond::lilypond_objects::lilypond_note::LilyPondNote;
    /// use lilypond::notation::note::Note;
    /// use lilypond::notation::pitch::{Pitch, NoteName, Accidental, Octave};
    /// use lilypond::notation::rhythm::{Rhythm, Length, DurationType};
    ///
    /// let ly_note = LilyPondNote::new("af,8.").unwrap();
    /// let note = Note::from(&ly_note);
    ///
    /// assert_eq!(note.pitch.note_name, NoteName::A);
    /// assert_eq!(note.pitch.octave, Octave::S2);
    /// assert_eq!(note.pitch.accidental, Accidental::Flat);
    /// assert_eq!(note.rhythm.length, Length::Eighth);
    /// assert_eq!(note.rhythm.dotted, true);
    /// assert_eq!(note.rhythm.duration_type, DurationType::Note);
    /// ```
    fn from(note: &LilyPondNote) -> Note {
        note_from_lilypond(note)
    }
}

#[cfg(test)]
mod tests {
    use crate::lilypond_objects::lilypond_note::LilyPondNote;
    use crate::notation::note::Note;
    use crate::notation::pitch::{Accidental, NoteName, Octave};
    use crate::notation::rhythm::{DurationType, Length};
    #[test]
    fn test_from_lilypond_note() {
        let ly_note = LilyPondNote::new("a4").unwrap();
        let note = Note::from(&ly_note);
        assert_eq!(note.pitch.note_name, NoteName::A);
        assert_eq!(note.pitch.octave, Octave::S3);
        assert_eq!(note.pitch.accidental, Accidental::None);
        assert_eq!(note.rhythm.length, Length::Quarter);
        assert_eq!(note.rhythm.dotted, false);
        assert_eq!(note.rhythm.duration_type, DurationType::Note);
    }
}
