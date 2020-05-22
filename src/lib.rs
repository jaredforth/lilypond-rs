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

use std::process::Command;
use std::io::{self, Write};

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
    match Command::new("lilypond")
        .arg(input_file)
        .output() {
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
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
