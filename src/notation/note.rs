//! A single note type

use crate::notation::pitch::Pitch;
use crate::notation::rhythm::NoteDuration;
use crate::notation::pitch;

/// A note with rhythm and pitch
pub struct Note {
    pub pitch: Pitch,
    pub rhythm: NoteDuration
}

/// TODO shorthand for creating a new note with pitch and duration
/// TODO also need to create staff, time signature, key signature, chord
/// before we have a minimum viable product
impl Note {
    pub fn new() -> Note {
        Note {
            pitch: Pitch {
                note: pitch::Note::A,
                octave: Default::default(),
                accidental: Default::default()
            },
            rhythm: NoteDuration {
                length: Default::default(),
                dotted: false,
                duration_type: Default::default()
            }
        }
    }
}