//! Abstractions for encoding single pitches as MIDI integers.

use crate::notation::pitch::{Accidental, NoteName, Octave, Pitch};

/// A representation of a single pitch as a MIDI integer.
#[derive(Debug, PartialEq)]
pub struct MidiNote {
    note: i16,
}

// TODO: INPUT VALIDATION
impl MidiNote {
    /// Attempt to initialize a new `MidiNote` object with given pitch.
    ///
    /// # Errors
    ///
    /// Returns a `Result` depending on whether the input `note: i16` is
    /// positive (i.e. a valid MIDI integer). On a success, returns
    /// `Ok(MidiNote)` and on a failure, returns `Err(String)`, where the
    /// `String` is the error message.
    ///
    /// # Examples
    ///
    /// A successful initialization:
    ///
    /// ```rust
    /// use lilypond::midi::midi_note::MidiNote;
    ///
    /// let note = MidiNote::new(60).unwrap(); // middle c
    /// ```
    ///
    /// A failed initialization:
    ///
    /// ```rust
    /// use lilypond::midi::midi_note::MidiNote;
    ///
    /// let note = MidiNote::new(-42); // invalid integer
    /// assert_eq!(note, Err(String::from("Invalid MIDI integer -42.")))
    /// ```
    pub fn new(note: i16) -> Result<Self, String> {
        if note < 0 {
            Err(format!("Invalid MIDI integer {}.", note))
        } else {
            Ok(Self { note })
        }
    }

    /// Get the MIDI note integer of the current [`MidiNote`] object.
    pub fn get_note(&self) -> i16 {
        self.note
    }
}

impl Default for MidiNote {
    /// Set default `MidiNote` to middle C (MIDI value 60)
    fn default() -> Self {
        MidiNote { note: 60 }
    }
}

impl std::convert::TryFrom<&Pitch> for MidiNote {
    type Error = String;

    fn try_from(pitch: &Pitch) -> Result<Self, Self::Error> {
        // initialize note to MIDI note for pitch C of a given octave
        let mut note: i16 = match pitch.octave {
            Octave::S0 => 12,
            Octave::S1 => 24,
            Octave::S2 => 36,
            Octave::S3 => 48,
            Octave::S4 => 60,
            Octave::S5 => 72,
            Octave::S6 => 84,
            Octave::S7 => 96,
            Octave::S8 => 108,
            Octave::S9 => 120,
            Octave::None => 0,
        };

        // add to note the number of semitones corresponding to the note name
        note += match pitch.note_name {
            NoteName::C => 0,
            NoteName::D => 2,
            NoteName::E => 4,
            NoteName::F => 5,
            NoteName::G => 7,
            NoteName::A => 9,
            NoteName::B => 11,
            NoteName::None => 0,
        };

        // add to note the number of semitones corresponding to the accidental
        note += match pitch.accidental {
            Accidental::None => 0,
            Accidental::Sharp => 1,
            Accidental::DoubleSharp => 2,
            Accidental::Flat => -1,
            Accidental::DoubleFlat => -2,
        };

        Self::new(note)
    }
}

impl PartialOrd for MidiNote {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.get_note().partial_cmp(&other.get_note())
    }
}

#[cfg(test)]
mod test {
    use std::convert::TryFrom;

    use crate::{
        lilypond_objects::lilypond_note::LilyPondNote, midi::midi_note::MidiNote,
        notation::note::Note,
    };
    #[test]
    fn test_default() {
        let midi_note = MidiNote::default();
        assert_eq!(midi_note.get_note(), 60);
    }
    #[test]
    fn test_new() {
        let midi_note = MidiNote::new(48).unwrap();
        assert_eq!(midi_note.get_note(), 48);
    }
    #[test]
    #[allow(unused_variables)]
    #[should_panic]
    fn test_new_error() {
        let midi_note = MidiNote::new(-1).unwrap();
    }
    #[test]
    fn test_from_pitch_negative() {
        let note = Note::try_from(&LilyPondNote::new("rff").unwrap()).unwrap();
        let midi_note = MidiNote::try_from(&note.pitch).unwrap();
        assert_eq!(midi_note.get_note(), 0);
    }
}
