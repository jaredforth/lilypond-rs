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
pub enum Accidental {
    None,
    Sharp,
    Flat
}

/// Set no accidental as default
impl Default for Accidental {
    fn default() -> Self { Accidental::None }
}

pub struct Pitch {
    note: Note,
    octave: Octave,
    accidental: Accidental
}

impl Pitch {
    pub fn new(note: Note) -> Pitch {
        Pitch {
            note,
            octave: Default::default(),
            accidental: Default::default()
        }
    }
}