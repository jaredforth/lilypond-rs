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
pub enum Pitch {
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