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
pub fn compile() {
    match Command::new("lilypond")
        .arg("test.ly")
        .output() {
        Ok(o) => {
            if o.status.success() {
                io::stdout().write_all(o.stdout.as_ref()).unwrap();
            } else {
                io::stdout().write_all(o.stderr.as_ref()).unwrap();
            }
        }
        Err(e) => {
            println!("Could not run LilyPond. Error: {}", e);
            println!("Install LilyPond at https://lilypond.org/download.html");
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
