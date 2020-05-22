//! Abstractions for specifying the pitch of notes

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
pub enum Note {
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
    /// ## B Defaults:
    ///
    /// Scientific: B3
    /// Helmholtz: b
    D,
    /// ## D Defaults:
    ///
    /// Scientific: D3
    /// Helmholtz: d
    E,
    /// ## E Defaults:
    ///
    /// Scientific: E3
    /// Helmholtz: e
    F,
    /// ## F Defaults:
    ///
    /// Scientific: F3
    /// Helmholtz: f
    G
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
}

/// Set pitch below middle C as default
impl Default for Octave {
    fn default() -> Self { Octave::S3 }
}

/// Accidentals a natural note can have
#[derive(PartialEq, Debug)]
pub enum Accidental {
    None,
    Sharp,
    Flat
}

/// Set no accidental as default
impl Default for Accidental {
    fn default() -> Self { Accidental::None }
}

/// A single pitch
#[derive(PartialEq, Debug)]
pub struct Pitch {
    /// The note letter name
    ///
    /// e.g. C, E, or G
    pub note: Note,
    /// Octave value in scientific
    /// pitch notation.
    ///
    /// e.g. C4, D5
    pub octave: Octave,
    /// Accidental value
    ///
    /// e.g. None, # or b
    pub accidental: Accidental
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
    /// use lilypond::notation::pitch::{Pitch, Note, Accidental, Octave};
    ///
    /// let pitch = Pitch::new(Note::A);
    ///
    /// assert_eq!(pitch.note, Note::A);
    /// assert_eq!(pitch.octave, Octave::S3);
    /// assert_eq!(pitch.accidental, Accidental::None);
    /// ```
    pub fn new(note: Note) -> Pitch {
        Pitch {
            note,
            octave: Default::default(),
            accidental: Default::default()
        }
    }
    /// Set absolute octave value for a pitch
    ///
    /// # Usage:
    ///
    /// ```
    /// use lilypond::notation::pitch::{Pitch, Note, Octave};
    ///
    /// let mut  pitch = Pitch::new(Note::A);
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
    /// use lilypond::notation::pitch::{Pitch, Note, Octave, Accidental};
    ///
    /// let mut  pitch = Pitch::new(Note::A);
    /// pitch.accidental(Accidental::Sharp);
    ///
    /// assert_eq!(Accidental::Sharp, pitch.accidental)
    /// ```
    pub fn accidental(&mut self, accidental: Accidental) {
        self.accidental = accidental;
    }
}