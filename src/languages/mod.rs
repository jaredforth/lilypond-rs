//! A module (and submodules) for the various languages of LilyPond note input.
//!
//! The data for each language can be found here:
//! <http://lilypond.org/doc/v2.24/Documentation/notation/writing-pitches#note-names-in-other-languages>,
//! or the corresponding page for whatever LilyPond version is most current.
//!
//! Each language module must define the following public members:
//!
//! - `pub static LANGUAGE_STRING: &str`: The string to be used by LilyPond to
//! select the language, e.g. `"english"` or `"nederlands"`.
//! - `pub static NOTE_REGEX: &str`: A regular expression string which is used
//! to compile
//! `lilypond::lilypond_objects::lilypond_note::LILYPOND_NOTE_REGEX`. The string
//! must define five named capture groups as described in that documentation.
//! - `pub fn from(note: &note) -> String`: A function that generates a string
//! from the data in the note object according to the chosen language.

use crate::{
    lilypond_objects::lilypond_note::LilyPondNote, notation::note::Note, NoteNameLanguage,
};
use lazy_static::lazy_static;

mod common;
mod english;
mod nederlands;

lazy_static! {
    /// The string to be used by LilyPond to select the language,
    /// e.g. `"english"` or `"nederlands"`.
    ///
    /// Its value is dependent on that of
    /// [`NOTE_NAME_LANGUAGE`][struct@crate::NOTE_NAME_LANGUAGE].
    pub static ref LANGUAGE_STR: &'static str = match *crate::NOTE_NAME_LANGUAGE {
        NoteNameLanguage::English => english::LANGUAGE_STR,
        NoteNameLanguage::Nederlands => nederlands::LANGUAGE_STR,
    };
    /// A regular expression string which is used to match and parse LilyPond
    /// notes.
    ///
    /// Its value is dependent on that of
    /// [`NOTE_NAME_LANGUAGE`][struct@crate::NOTE_NAME_LANGUAGE].
    ///
    /// This is compiled to
    /// [`LILYPOND_NOTE_REGEX`][struct@crate::lilypond_objects::lilypond_note::LILYPOND_NOTE_REGEX]. See
    /// its documentation for more details.
    pub static ref NOTE_REGEX_STR: &'static str = match *crate::NOTE_NAME_LANGUAGE {
        NoteNameLanguage::English => english::NOTE_REGEX_STR,
        NoteNameLanguage::Nederlands => nederlands::NOTE_REGEX_STR,
    };
}

/// Convert a note into a LilyPond-formatted String according to the value of
/// [`NOTE_NAME_LANGUAGE`][struct@crate::NOTE_NAME_LANGUAGE].
pub fn lilypond_from_note(note: &Note) -> String {
    match *crate::NOTE_NAME_LANGUAGE {
        NoteNameLanguage::English => english::lilypond_from_note(note),
        NoteNameLanguage::Nederlands => nederlands::lilypond_from_note(note),
    }
}

/// Attempt to convert a LilyPondNote into a Note object according to the value
/// of [`NOTE_NAME_LANGUAGE`][struct@crate::NOTE_NAME_LANGUAGE].
///
/// # Errors
///
/// Returns a `Result` according to whether the conversion was successful. On a
/// success, returns `Ok(Note)`, and on a failure, returns `Err(String)` where
/// the `String` is the associated error message.
pub fn note_from_lilypond(note: &LilyPondNote) -> Result<Note, String> {
    match *crate::NOTE_NAME_LANGUAGE {
        NoteNameLanguage::English => english::note_from_lilypond(note),
        NoteNameLanguage::Nederlands => nederlands::note_from_lilypond(note),
    }
}
