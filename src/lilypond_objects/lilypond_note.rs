//! Abstractions for LilyPond-syntax `String` representations of notes.

use crate::notation::{
    note::Note,
    pitch::{Accidental, NoteName, Octave},
    rhythm::{DurationType, Length},
};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    /// A regular expression for matching and capturing from LilyPond notes.
    ///
    /// Contains the following named capture groups:
    /// - `note_name`: the name of the note, i.e. lowercase letters a-g or r for
    /// a rest.
    /// - `accidental`: the accidental of the note, i.e. between zero and two
    /// repetitions of either "is" or "es" (or "s" or "f" in English notation).
    /// - `octave`: the absolute octave of the note, i.e. between zero and three
    /// commas for each octave below 3 or between zero and six apostrophes for
    /// each octave above 3.
    /// - `duration`: the number (a power of 2) representing the duration of the
    /// note.
    /// - `dot`: a period, present if the note is dotted.
    ///
    /// These can be used to extract the relevant subsections of the note for
    /// further analysis as such:
    ///
    /// ```rust
    /// use lilypond::lilypond_objects::lilypond_note::LILYPOND_NOTE_REGEX;
    ///
    /// let note = "fss'''128.";
    /// let note_name = LILYPOND_NOTE_REGEX.replace(note, "$note_name");
    /// let accidental = LILYPOND_NOTE_REGEX.replace(note, "$accidental");
    /// let octave = LILYPOND_NOTE_REGEX.replace(note, "$octave");
    /// let duration = LILYPOND_NOTE_REGEX.replace(note, "$duration");
    /// let dot = LILYPOND_NOTE_REGEX.replace(note, "$dot");
    ///
    /// assert_eq!(note_name, "f");
    /// assert_eq!(accidental, "ss");
    /// assert_eq!(octave, "'''");
    /// assert_eq!(duration, "128");
    /// assert_eq!(dot, ".");
    /// ```
    ///
    /// The same can be done with objects of type `LilyPondNote` by calling the
    /// `get_capture` method. For more information, see
    /// [`LilyPondNote.get_capture()`][crate::lilypond_objects::lilypond_note::LilyPondNote#method.get_capture].
    ///
    /// `LILYPOND_NOTE_REGEX` can also be used to check if a string is a valid
    /// LilyPond note:
    ///
    /// ```rust
    /// use lilypond::lilypond_objects::lilypond_note::LILYPOND_NOTE_REGEX;
    ///
    /// let good_note = "fisis,,,64.";
    /// let bad_note = "asdf";
    ///
    /// assert!(LILYPOND_NOTE_REGEX.is_match(good_note));
    /// assert!(!LILYPOND_NOTE_REGEX.is_match(bad_note));
    /// ```
    pub static ref LILYPOND_NOTE_REGEX: Regex = Regex::new(
        r"(?x-u) # Flags: x = whitespace allowed, -u = no unicode support
        ^(?P<note_name>[a-gr]) # note name or rest
        (?P<accidental>(?:f{0,2}|s{0,2})|(?:(?:is){0,2}|(?:es){0,2})) # accidental
        (?P<octave>(?:(?:,{0,3})|(?:'{0,6}))?) # octave transposition characters
        (?P<duration>(?:1|2|4|8|(?:16)|(?:32)|(?:64)|(?:128))?) # Durations
        (?P<dot>\.?)$ # optional dot and end of line
        ",
    )
    .unwrap();
}

/// A struct to contain the string representation of a LilyPond note.
#[derive(Debug, PartialEq)]
pub struct LilyPondNote {
    note: String,
}

impl LilyPondNote {
    /// Initialize a lilypond note string, checking for proper formatting
    ///
    /// # Errors
    ///
    /// This function returns a Result according to whether `&str note` matches
    /// `LILYPOND_NOTE_REGEX`. On a successful match, return `Ok(LilyPondNote)`,
    /// and on a failure, return `Err(())`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lilypond::lilypond_objects::lilypond_note::LilyPondNote;
    ///
    /// let ly_note = LilyPondNote::new("a4").unwrap();
    /// assert_eq!(ly_note.get_note(), "a4");
    /// ```
    pub fn new(note: &str) -> Result<Self, ()> {
        if LILYPOND_NOTE_REGEX.is_match(note) {
            Ok(LilyPondNote {
                note: note.to_string(),
            })
        } else {
            Err(())
        }
    }

    /// Return a reference to the string that represents the current
    /// `LilyPondNote`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lilypond::lilypond_objects::lilypond_note::LilyPondNote;
    ///
    /// let ly_note = LilyPondNote::new("a4").unwrap();
    /// assert_eq!(ly_note.get_note(), "a4")
    /// ```
    pub fn get_note(&self) -> &String {
        &self.note
    }

    /// Get a capture from current note according to label `&str capture` in `LILYPOND_NOTE_REGEX`.
    ///
    /// Available capture group labels follow. See
    /// [`LILYPOND_NOTE_REGEX`][struct@crate::lilypond_objects::lilypond_note::LILYPOND_NOTE_REGEX]
    /// for more info on their descriptions.
    /// - `note_name`
    /// - `accidental`
    /// - `octave`
    /// - `duration`
    /// - `dot`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lilypond::lilypond_objects::lilypond_note::LilyPondNote;
    ///
    /// let note = LilyPondNote::new("aeses'''''8").unwrap();
    /// assert_eq!(note.get_capture("note_name"), "a");
    /// assert_eq!(note.get_capture("accidental"), "eses");
    /// assert_eq!(note.get_capture("octave"), "'''''");
    /// assert_eq!(note.get_capture("duration"), "8");
    /// assert_eq!(note.get_capture("dot"), "");
    /// ```
    pub fn get_capture(&self, capture: &str) -> String {
        LILYPOND_NOTE_REGEX
            .replace(&self.note, format!("${}", capture))
            .to_string()
    }
}

impl From<&Note> for LilyPondNote {
    /// Translate a note object into a lilypond note string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lilypond::lilypond_objects::lilypond_note::LilyPondNote;
    /// use lilypond::notation::note::Note;
    /// use lilypond::notation::pitch::NoteName;
    ///
    /// let note = Note::new(NoteName::A);
    /// let ly_note = LilyPondNote::from(&note);
    /// assert_eq!(ly_note.get_note(), "a4");
    /// ```
    fn from(note: &Note) -> Self {
        LilyPondNote::new(
            format!(
                "{}{}{}{}{}",
                match note.rhythm.duration_type {
                    DurationType::Rest => "r",
                    DurationType::Note => match note.pitch.note_name {
                        NoteName::A => "a",
                        NoteName::B => "b",
                        NoteName::C => "c",
                        NoteName::D => "d",
                        NoteName::E => "e",
                        NoteName::F => "f",
                        NoteName::G => "g",
                        NoteName::None => "r",
                    },
                },
                match note.pitch.accidental {
                    Accidental::None => "",
                    Accidental::Flat => "f",
                    Accidental::DoubleFlat => "ff",
                    Accidental::Sharp => "s",
                    Accidental::DoubleSharp => "ss",
                },
                match note.pitch.octave {
                    Octave::S0 => ",,,",
                    Octave::S1 => ",,",
                    Octave::S2 => ",",
                    Octave::S3 => "",
                    Octave::S4 => "'",
                    Octave::S5 => "''",
                    Octave::S6 => "'''",
                    Octave::S7 => "''''",
                    Octave::S8 => "'''''",
                    Octave::S9 => "''''''",
                    Octave::None => "",
                },
                match note.rhythm.length {
                    Length::Whole => "1",
                    Length::Half => "2",
                    Length::Quarter => "4",
                    Length::Eighth => "8",
                    Length::Sixteenth => "16",
                    Length::ThirtySecond => "32",
                    Length::SixtyFourth => "64",
                    Length::OneTwentyEighth => "128",
                },
                match note.rhythm.dotted {
                    true => ".",
                    false => "",
                }
            )
            .as_str(),
        )
        .unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::lilypond_objects::lilypond_note::LilyPondNote;
    use crate::notation::note::Note;
    use crate::notation::pitch::{Accidental, NoteName, Octave, Pitch};
    use crate::notation::rhythm::{DurationType, Length, Rhythm};
    fn test_lilypond_note(ly_str: &str) {
        let note = LilyPondNote::new(ly_str).unwrap().note;
        assert_eq!(ly_str, note);
    }
    #[test]
    fn test_new_lilypond_note() {
        // Testing a bunch of possible lilypond notes
        let ly_notes = [
            "r",
            "a",
            "bf",
            "bes",
            "cs",
            "cis",
            "d,",
            "ef'",
            "ees'",
            "fs",
            "fis",
            "g,,",
            "af''1",
            "bs2",
            "c,,,4",
            "df'''8",
            "es16",
            "f32",
            "gf''''64",
            "as128",
            "b,",
            "cf'''''1.",
            "ds,,2.",
            "e4.",
            "ff,,,8.",
            "fff,,,8.",
            "fisis,,,8.",
            "gs''''''16.",
        ];
        for n in ly_notes {
            test_lilypond_note(n);
        }
    }
    fn test_lilypond_note_error(ly_str: &str) {
        let note = LilyPondNote::new(ly_str);
        assert_eq!(note, Err(()));
    }
    #[test]
    fn test_new_lilypond_notes_error() {
        let notes = [
            "asdf",
            "aises",
            "afs",
            "h",
            "c,,,,",
            "asf",
            "aesis",
            "d'''''''",
            "b3",
            "c5",
            "d6",
            "e7",
            "f9",
            "g12",
            "g13",
            "a21",
            "b24",
            "c162",
            "d1281",
            "gs''''''16.1",
        ];
        for note in notes {
            test_lilypond_note_error(note);
        }
    }
    #[test]
    fn test_from_note() {
        let note = Note::new(NoteName::A);
        let ly_note = LilyPondNote::from(&note).note;
        assert_eq!("a4", ly_note);
    }
    #[test]
    fn test_to_note() {
        let ly_note = LilyPondNote::new("r8.").unwrap();
        let test_note = Note {
            pitch: Pitch {
                note_name: NoteName::None,
                octave: Octave::None,
                accidental: Accidental::None,
            },
            rhythm: Rhythm {
                length: Length::Eighth,
                duration_type: DurationType::Rest,
                dotted: true,
            },
        };
        assert_eq!(<&LilyPondNote as Into<Note>>::into(&ly_note), test_note);
        let ly_note = LilyPondNote::new("ef,,,64").unwrap();
        let test_note = Note {
            pitch: Pitch {
                note_name: NoteName::E,
                octave: Octave::S0,
                accidental: Accidental::Flat,
            },
            rhythm: Rhythm {
                length: Length::SixtyFourth,
                duration_type: DurationType::Note,
                dotted: false,
            },
        };
        assert_eq!(<&LilyPondNote as Into<Note>>::into(&ly_note), test_note);
    }
}
