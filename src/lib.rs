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

use crate::notation::pitch::NoteName;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

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
pub fn compile(input_file: &'static str) -> bool {
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
/// # Usage:
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
pub fn is_lilypond_file(filename: &'static str) -> bool {
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
    /// ## Usage:
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
    /// ## Usage:
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
