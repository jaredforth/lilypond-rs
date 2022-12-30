//! Abstractions for encoding single pitches as MIDI integers.

use crate::notation::pitch::{Accidental, NoteName, Octave, Pitch};

#[derive(Debug, PartialEq)]
pub struct MidiNote {
    note: i16,
}

impl MidiNote {
    /// Initialize a new `MidiNote` object with given pitch
    pub fn new(note: i16) -> Self {
        MidiNote { note }
    }

    /// Getter for `note` field in `MidiNote` struct.
    pub fn get_note(&self) -> i16 {
        self.note
    }

    /// Setter for `note` field in `MidiNote` struct.
    pub fn set_note(&mut self, note: i16) {
        self.note = note;
    }
}

impl Default for MidiNote {
    /// Set default `MidiNote` to middle C (MIDI value 60)
    fn default() -> Self {
        MidiNote { note: 60 }
    }
}

impl From<&Pitch> for MidiNote {
    fn from(pitch: &Pitch) -> Self {
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

        if note < 0 {
            panic!("Cannot have negative MIDI value.");
        }

        MidiNote { note }
    }
}

impl PartialOrd for MidiNote {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.get_note().partial_cmp(&other.get_note())
    }
}

#[cfg(test)]
mod test {
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
        let midi_note = MidiNote::new(48);
        assert_eq!(midi_note.get_note(), 48);
    }
    #[test]
    fn test_set() {
        let mut midi_note = MidiNote::default();
        midi_note.set_note(84);
        assert_eq!(midi_note.get_note(), 84);
    }
    #[test]
    fn test_from_pitch_negative() {
        let note = Note::from(&LilyPondNote::new("rff").unwrap());
        let midi_note = MidiNote::from(&note.pitch);
        assert_eq!(midi_note.get_note(), 0);
    }
}
