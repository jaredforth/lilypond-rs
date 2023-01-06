[![Build Status](https://travis-ci.com/jaredforth/lilypond-rs.svg?token=mH2pScYxqRkBEzpBQAu6&branch=master)](https://travis-ci.com/jaredforth/lilypond)
[![Build status](https://ci.appveyor.com/api/projects/status/w75cp0q4qr0hngf8?svg=true)](https://ci.appveyor.com/project/jaredforth/lilypond)
[![Crate](https://img.shields.io/crates/v/lilypond.svg)](https://crates.io/crates/lilypond)
[![API](https://docs.rs/lilypond/badge.svg)](https://docs.rs/lilypond)

# lilypond-rs

A Rust wrapper and types for [GNU LilyPond](https://lilypond.org/).

**lilypond-rs** provides an API to ergonomically wrap LilyPond, and provide Rust types that resolve to LilyPond output. This is still a work in progress and is not stable as of yet.

Documentation:
-   [API Reference](https://docs.rs/lilypond)


## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
lilypond = "0.1"
```

## Installing LilyPond

Because this library wraps LilyPond, the [GNU LilyPond](https://lilypond.org/) program must be installed before library usage.

After you install LilyPond, check to see if LilyPond is installed:

```shell script
lilypond --version
GNU LilyPond 2.20.0

Copyright (c) 1996--2015 by
  Han-Wen Nienhuys <hanwen@xs4all.nl>
  Jan Nieuwenhuizen <janneke@gnu.org>
  and others.

This program is free software.  It is covered by the GNU General Public
License and you are welcome to change it and/or distribute copies of it
under certain conditions.  Invoke as `lilypond --warranty' for more
information.
```

## Inspiration

One of the major inspirations for this library is [Abjad](https://abjad.github.io/), and the goal is that eventually a similar compositional workflow can be achieved in Rust.

## Contribution 

This project is too large for one individual to complete, so contributions are greatly appreciated. All contributors on this project are expected to abide by the [Contributor Covenant Code of Conduct](/code_of_conduct.md).
## License

**lilypond-rs** is distributed under the terms of the GNU General Public License. 

This is to respect the philosophy of the GNU LilyPond project. You can learn more about this [here](https://lilypond.org/freedom.html).
