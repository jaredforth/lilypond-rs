//! Abstractions for specifying the pitch of notes

use crate::midi::midi_note::MidiNote;

/// Natural pitch note names.
///
/// These are the 7 pitches in Western music
/// with no accidentals (A through G).
///
/// Each value is representative of a [pitch class](https://en.wikipedia.org/wiki/Pitch_class),
/// and can be designated by both
/// [scientific pitch](https://en.wikipedia.org/wiki/Scientific_pitch_notation)
/// and [helmholtz pitch](https://en.wikipedia.org/wiki/Helmholtz_pitch_notation).
#[derive(PartialEq, Debug)]
pub enum NoteName {
    /// ## A Defaults:
    ///
    /// Scientific: A3
    /// Helmholtz: a
    A,
    /// ## B Defaults:
    ///
    /// Scientific: B3
    /// Helmholtz: b
    B,
    /// ## C Defaults:
    ///
    /// Scientific: C3
    /// Helmholtz: c
    C,
    /// ## D Defaults:
    ///
    /// Scientific: D3
    /// Helmholtz: d
    D,
    /// ## E Defaults:
    ///
    /// Scientific: E3
    /// Helmholtz: e
    E,
    /// ## F Defaults:
    ///
    /// Scientific: F3
    /// Helmholtz: f
    F,
    /// ## G Defaults:
    ///
    /// Scientific: G3
    /// Helmholtz: g
    G,
    /// For rests
    None,
}

/// Octaves a pitch can have
///
/// [Scientific Pitch](https://en.wikipedia.org/wiki/Scientific_pitch_notation)
/// is used to represent possible octave values.
#[derive(PartialEq, Debug)]
pub enum Octave {
    S0,
    S1,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    S8,
    S9,
    None,
}

/// Set pitch below middle C as default
impl Default for Octave {
    fn default() -> Self {
        Octave::S3
    }
}

/// Accidentals a natural note can have
#[derive(PartialEq, Debug)]
pub enum Accidental {
    None,
    Sharp,
    DoubleSharp,
    Flat,
    DoubleFlat,
}

/// Set no accidental as default
impl Default for Accidental {
    fn default() -> Self {
        Accidental::None
    }
}

/// A single pitch
#[derive(PartialEq, Debug)]
pub struct Pitch {
    /// The note letter name
    ///
    /// e.g. C, E, or G
    pub note_name: NoteName,
    /// Octave value in scientific
    /// pitch notation.
    ///
    /// e.g. C4, D5
    pub octave: Octave,
    /// Accidental value
    ///
    /// e.g. None, # or b
    pub accidental: Accidental,
}

impl Pitch {
    /// Construct a new pitch
    ///
    /// This will take a note name, and
    /// initialize with default values.
    ///
    /// # Usage:
    ///
    /// ```
    /// use lilypond::notation::pitch::{Pitch, NoteName, Accidental, Octave};
    ///
    /// let pitch = Pitch::new(NoteName::A);
    ///
    /// assert_eq!(pitch.note_name, NoteName::A);
    /// assert_eq!(pitch.octave, Octave::S3);
    /// assert_eq!(pitch.accidental, Accidental::None);
    /// ```
    pub fn new(note: NoteName) -> Pitch {
        Pitch {
            note_name: note,
            octave: Default::default(),
            accidental: Default::default(),
        }
    }
    /// Set absolute octave value for a pitch
    ///
    /// # Usage:
    ///
    /// ```
    /// use lilypond::notation::pitch::{Pitch, NoteName, Octave};
    ///
    /// let mut  pitch = Pitch::new(NoteName::A);
    /// pitch.octave(Octave::S6);
    ///
    /// assert_eq!(Octave::S6, pitch.octave)
    /// ```
    pub fn octave(&mut self, octave: Octave) {
        self.octave = octave;
    }
    /// Set absolute accidental value for a pitch
    ///
    /// # Usage:
    ///
    /// ```
    /// use lilypond::notation::pitch::{Pitch, NoteName, Octave, Accidental};
    ///
    /// let mut  pitch = Pitch::new(NoteName::A);
    /// pitch.accidental(Accidental::Sharp);
    ///
    /// assert_eq!(Accidental::Sharp, pitch.accidental)
    /// ```
    pub fn accidental(&mut self, accidental: Accidental) {
        self.accidental = accidental;
    }
    /// Sharpen pitch
    ///
    /// # Usage:
    ///
    /// ```
    /// use lilypond::notation::pitch::{Pitch, NoteName, Octave, Accidental};
    ///
    /// let mut  pitch = Pitch::new(NoteName::A);
    /// pitch.sharpen();
    ///
    /// assert_eq!(Accidental::Sharp, pitch.accidental)
    /// ```
    pub fn sharpen(&mut self) {
        self.accidental = Accidental::Sharp
    }
    /// Flatten pitch
    ///
    /// # Usage:
    ///
    /// ```
    /// use lilypond::notation::pitch::{Pitch, NoteName, Octave, Accidental};
    ///
    /// let mut  pitch = Pitch::new(NoteName::A);
    /// pitch.flatten();
    ///
    /// assert_eq!(Accidental::Flat, pitch.accidental)
    /// ```
    pub fn flatten(&mut self) {
        self.accidental = Accidental::Flat
    }
}

impl From<&MidiNote> for Pitch {
    fn from(note: &MidiNote) -> Self {
        let note_int = note.get_note();
        let octave_int: i16 = note_int / 12;
        let pc_int: i16 = note_int % 12;

        Pitch {
            octave: match octave_int - 1 {
                -1 => Octave::None,
                0 => Octave::S0,
                1 => Octave::S1,
                2 => Octave::S2,
                3 => Octave::S3,
                4 => Octave::S4,
                5 => Octave::S5,
                6 => Octave::S6,
                7 => Octave::S7,
                8 => Octave::S8,
                9 => Octave::S9,
                e => panic!("Invalid octave integer {}.", e),
            },
            note_name: match octave_int - 1 {
                -1 => NoteName::None,
                _ => match pc_int {
                    0 | 1 => NoteName::C,
                    2 => NoteName::D,
                    3 | 4 => NoteName::E,
                    5 | 6 => NoteName::F,
                    7 => NoteName::G,
                    8 | 9 => NoteName::A,
                    10 | 11 => NoteName::B,
                    e => panic!("Invalid pitch-class integer {}.", e),
                },
            },
            accidental: match octave_int - 1 {
                -1 => Accidental::None,
                _ => match pc_int {
                    0 | 2 | 4 | 5 | 7 | 9 | 11 => Accidental::None,
                    1 | 6 => Accidental::Sharp,
                    3 | 8 | 10 => Accidental::Flat,
                    e => panic!("Invalid pitch-class integer {}.", e),
                },
            },
        }
    }
}

#[cfg(test)]
mod test {
    use crate::notation::pitch::*;
    #[test]
    fn test_new() {
        let pitch = Pitch::new(NoteName::A);
        assert_eq!(pitch.note_name, NoteName::A);
        assert_eq!(pitch.octave, Octave::S3);
        assert_eq!(pitch.accidental, Accidental::None);
    }
    #[test]
    fn test_octave() {
        let mut pitch = Pitch::new(NoteName::A);
        pitch.octave(Octave::S9);
        assert_eq!(pitch.octave, Octave::S9);
    }
    #[test]
    fn test_accidental() {
        let mut pitch = Pitch::new(NoteName::A);
        pitch.accidental(Accidental::Flat);
        assert_eq!(pitch.accidental, Accidental::Flat);
    }
    #[test]
    fn test_flatten() {
        let mut pitch = Pitch::new(NoteName::A);
        pitch.flatten();
        assert_eq!(pitch.accidental, Accidental::Flat);
    }
    #[test]
    fn test_sharpen() {
        let mut pitch = Pitch::new(NoteName::A);
        pitch.sharpen();
        assert_eq!(pitch.accidental, Accidental::Sharp);
    }
    #[test]
    fn test_from_midi_note() {
        let middle_c_midi = MidiNote::new(60);
        let middle_c_pitch = Pitch::from(&middle_c_midi);
        assert_eq!(middle_c_pitch.octave, Octave::S4);
        assert_eq!(middle_c_pitch.note_name, NoteName::C);
        assert_eq!(middle_c_pitch.accidental, Accidental::None);
        let very_low_midi = MidiNote::new(12);
        let very_low_pitch = Pitch::from(&very_low_midi);
        assert_eq!(very_low_pitch.octave, Octave::S0);
        assert_eq!(very_low_pitch.note_name, NoteName::C);
        assert_eq!(very_low_pitch.accidental, Accidental::None);
        let very_high_midi = MidiNote::new(126);
        let very_high_pitch = Pitch::from(&very_high_midi);
        assert_eq!(very_high_pitch.octave, Octave::S9);
        assert_eq!(very_high_pitch.note_name, NoteName::F);
        assert_eq!(very_high_pitch.accidental, Accidental::Sharp);
    }
    #[test]
    fn test_from_midi_note_rest() {
        let midi_note = MidiNote::new(0);
        let pitch = Pitch::from(&midi_note);
        assert_eq!(pitch.octave, Octave::None);
        assert_eq!(pitch.note_name, NoteName::None);
        assert_eq!(pitch.accidental, Accidental::None);
    }
}
