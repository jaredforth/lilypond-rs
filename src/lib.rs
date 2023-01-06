// Copyright 2020 Jared Forth.
//
// Licensed under the GNU General Public License v3.0 <LICENSE or
// https://www.gnu.org/licenses/gpl-3.0.en.html>.
// This file may not be copied, modified, or distributed
// except according to those terms.

//! Utilities for common filesystem operations.
//!
//! **lilypond** provides an API to ergonomically wrap LilyPond,
//! and provide Rust types that resolve to LilyPond output.

use lazy_static::lazy_static;

pub use crate::languages::{lilypond_from_note, note_from_lilypond, LANGUAGE_STR, NOTE_REGEX_STR};
use crate::notation::pitch::NoteName;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

mod languages;
pub mod lilypond_objects;
pub mod midi;
pub mod notation;
pub mod parser;

/// Compiles a `.ly` source file
///
/// # Usage
///
/// ```
/// use lilypond::compile;
///
/// let input = "test.ly";
/// let output = "test.pdf";
/// // Create test input file
/// fsutils::write_file(input, "{ c e g }");
///
/// if compile(input) {
///    // LilyPond is probably installed
///    assert_eq!(fsutils::path_exists(output), true);
/// } else {
///     // LilyPond is not installed and we will
///     // not get an output
///     assert_eq!(fsutils::path_exists(output), false);
/// }
///
/// // Cleanup
/// fsutils::rm(input);
/// fsutils::rm(output);
///
/// ```
pub fn compile(input_file: &str) -> bool {
    if is_lilypond_file(input_file) {
        match Command::new("lilypond").arg(input_file).output() {
            Ok(o) => {
                if o.status.success() {
                    println!("Compiled {}", input_file);
                    io::stdout().write_all(o.stdout.as_ref()).unwrap();
                    true
                } else {
                    io::stdout().write_all(o.stderr.as_ref()).unwrap();
                    false
                }
            }
            Err(e) => {
                println!("Could not run LilyPond. Error: {}", e);
                println!("Install LilyPond at https://lilypond.org/download.html");
                false
            }
        }
    } else {
        println!("File {} does not exist or is invalid.", input_file);
        println!("Make sure your file has the .ly extension");
        false
    }
}

/// Checks if file has `.ly` extension
///
/// # Examples
///
/// ```
/// use lilypond::is_lilypond_file;
///
/// let valid = "test.ly";
/// let invalid = "test.png";
/// // Create valid file
/// fsutils::create_file(valid);
/// // Create invalid file
/// fsutils::create_file(invalid);
///
/// assert_eq!(is_lilypond_file(valid), true);
/// assert_eq!(is_lilypond_file(invalid), false);
/// // Will also return false if file does not exist
/// assert_eq!(is_lilypond_file("does_not_exit.txt"), false);
///
/// // Clean up
/// fsutils::rm(valid);
/// fsutils::rm(invalid);
/// ```
pub fn is_lilypond_file(filename: &str) -> bool {
    match Path::new(filename).extension() {
        Some(ex) => {
            if ex == "ly" {
                true
            } else {
                false
            }
        }
        None => false,
    }
}

/// A Rust representation of LilyPond data.
#[derive(PartialEq, Debug)]
pub struct LilyPond {
    pub notes: Vec<NoteName>,
}

impl LilyPond {
    /// Creates new instance of `LilyPond` struct
    ///
    /// # Examples
    ///
    /// ```
    /// use lilypond::LilyPond;
    ///
    /// let ly = LilyPond::new();
    ///
    /// assert_eq!(ly, LilyPond {notes: vec![]})
    /// ```
    pub fn new() -> LilyPond {
        LilyPond { notes: vec![] }
    }
    /// Parses LilyPond input as a string into the data structure.
    ///
    /// # Examples
    ///
    /// ```
    /// use lilypond::LilyPond;
    ///
    /// let mut ly = LilyPond::new();
    /// let parsed = ly.parse("{c e f}");
    /// ```
    pub fn parse(&mut self, raw: &'static str) {
        println!("{}", raw);
    }
}

/// The possible languages for note parsing.
///
/// See
/// <https://lilypond.org/doc/v2.24/Documentation/notation/writing-pitches#note-names-in-other-languages>
/// or the corresponding page for whatever LilyPond version is most current
/// for more information. Currently only supports English and Dutch (LilyPond's
/// default).
///
/// TODO: add support for more languages.
pub enum NoteNameLanguage {
    /// English note names and accidentals.
    ///
    /// - Note names: `c`, `d`, `e`, `f`, `g`, `a`, `b`
    ///
    /// - Accidentals: `s`, `f`, `ss`, `ff`, `-sharp`, `-flat`, `-sharpsharp`,
    /// `-flatflat`
    ///
    English,
    /// Dutch note names and accidentals.
    ///
    /// - Note names: `c`, `d`, `e`, `f`, `g`, `a`, `b`
    /// - Accidentals: `is`, `es`, `isis`, `eses`
    Nederlands,
}

impl Default for NoteNameLanguage {
    /// Initialize the default `NoteNameLanguage` to English.
    fn default() -> Self {
        Self::English
    }
}

lazy_static! {
    /// The [`NoteNameLanguage`][crate::NoteNameLanguage] for parsing from and
    /// encoding to
    /// [`LilyPondNote`][crate::lilypond_objects::lilypond_note::LilyPondNote]
    /// objects.
    ///
    /// The default NoteNameLanguage for the library is English.
    ///
    /// Note that this is created by the `lazy_static!` macro, and as such may
    /// need to be dereferenced with the `*` operator (e.g. in `match`
    /// expressions).
    ///
    /// ```rust
    /// match *lilypond::NOTE_NAME_LANGUAGE {
    ///     _ => println!("Don't forget to deref!"),
    /// }
    /// ```
    ///
    /// TODO: make this read a config file via the `config` crate.
    pub static ref NOTE_NAME_LANGUAGE: NoteNameLanguage = Default::default();
}
