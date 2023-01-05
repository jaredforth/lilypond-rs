//! Abstractions for LilyPond-syntax `String` representations of notes.

use crate::{lilypond_from_note, notation::note::Note, NOTE_REGEX_STR};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    /// A regular expression for matching and capturing from LilyPond notes.
    ///
    /// This regex changes depending on the language set by
    /// [`NOTE_NAME_LANGUAGE`][struct@crate::NOTE_NAME_LANGUAGE].
    ///
    /// This regex contains the following named capture groups:
    /// - `note_name`: the name of the note according to the chosen language,
    /// e.g., in English, lowercase letters a-g or r for a rest.
    /// - `accidental`: the accidental of the note, i.e. between zero and two
    /// repetitions of the language's chosen accidentals (e.g. "s" and "f" in
    /// English).
    /// - `octave`: the absolute octave of the note, i.e. between zero and three
    /// commas for each octave below 3 or between zero and six apostrophes for
    /// each octave above 3.
    /// - `duration`: the number (a power of 2) representing the duration of the
    /// note.
    /// - `dot`: a period, present if the note is dotted.
    ///
    /// Only the `note_name` and `accidental` fields change depending on the
    /// specified [`NOTE_NAME_LANGUAGE`][struct@crate::NOTE_NAME_LANGUAGE]; the
    /// others remain consistent.
    ///
    /// These capture groups can be used to extract the relevant subsections of
    /// the note for further analysis as such:
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
    /// let good_note = "fss,,,64.";
    /// let bad_note = "asdf";
    ///
    /// assert!(LILYPOND_NOTE_REGEX.is_match(good_note));
    /// assert!(!LILYPOND_NOTE_REGEX.is_match(bad_note));
    /// ```
    pub static ref LILYPOND_NOTE_REGEX: Regex = Regex::new(
        *NOTE_REGEX_STR
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
    /// [`LILYPOND_NOTE_REGEX`][struct@crate::lilypond_objects::lilypond_note::LILYPOND_NOTE_REGEX].
    /// On a successful match, return `Ok(LilyPondNote)`, and on a failure,
    /// return `Err(String)`, where the `String` is the error message.
    ///
    /// # Examples
    ///
    /// A successful initialization:
    ///
    /// ```rust
    /// use lilypond::lilypond_objects::lilypond_note::LilyPondNote;
    ///
    /// let ly_note = LilyPondNote::new("a4").unwrap();
    /// assert_eq!(ly_note.get_note(), "a4");
    /// ```
    ///
    /// An unsuccessful initialization:
    ///
    /// ```rust
    /// use lilypond::lilypond_objects::lilypond_note::LilyPondNote;
    ///
    /// let ly_note = LilyPondNote::new("asdf");
    /// assert_eq!(ly_note, Err(String::from("Invalid LilyPond note \"asdf\".")));
    /// ```
    pub fn new(note: &str) -> Result<Self, String> {
        if LILYPOND_NOTE_REGEX.is_match(note) {
            Ok(LilyPondNote {
                note: note.to_string(),
            })
        } else {
            Err(format!("Invalid LilyPond note \"{}\".", note))
        }
    }

    /// Return a reference to the string that represents the current
    /// [`LilyPondNote`].
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
    /// let note = LilyPondNote::new("aff'''''8").unwrap();
    /// assert_eq!(note.get_capture("note_name"), "a");
    /// assert_eq!(note.get_capture("accidental"), "ff");
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

impl std::convert::TryFrom<&Note> for LilyPondNote {
    type Error = String;

    /// Attempt to translate a note object into a lilypond note string.
    ///
    /// # Errors
    ///
    /// Returns a `Result` according to whether the conversion was
    /// successful. If an `Err(String)` was returned, the output of
    /// [`lilypond_from_note`] was likely malformed in some way. If so, the
    /// language module corresponding to the value of
    /// [`NOTE_NAME_LANGUAGE`][struct@crate::NOTE_NAME_LANGUAGE] has a bug and
    /// should be reported.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::convert::TryFrom;
    /// use lilypond::lilypond_objects::lilypond_note::LilyPondNote;
    /// use lilypond::notation::note::Note;
    /// use lilypond::notation::pitch::NoteName;
    ///
    /// let note = Note::new(NoteName::A);
    /// let ly_note = LilyPondNote::try_from(&note).unwrap();
    /// assert_eq!(ly_note.get_note(), "a4");
    /// ```
    fn try_from(note: &Note) -> Result<Self, Self::Error> {
        LilyPondNote::new(lilypond_from_note(note).as_str())
    }
}

#[cfg(test)]
mod test {
    use std::convert::{TryFrom, TryInto};

    use crate::lilypond_objects::lilypond_note::LilyPondNote;
    use crate::notation::note::Note;
    use crate::notation::pitch::{Accidental, NoteName, Octave, Pitch};
    use crate::notation::rhythm::{Dots, DurationType, Length, Rhythm};
    fn test_lilypond_note(ly_str: &str) {
        let note = LilyPondNote::new(ly_str).unwrap().note;
        assert_eq!(ly_str, note);
    }
    #[test]
    fn test_new_lilypond_note() {
        // Testing a bunch of possible english lilypond notes
        let ly_notes = [
            "r",
            "a",
            "bf",
            "cs",
            "d,",
            "ef'",
            "fs",
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
            "gs''''''16.",
        ];
        for n in ly_notes {
            test_lilypond_note(n);
        }
    }
    fn test_lilypond_note_error(ly_str: &str) {
        let note = LilyPondNote::new(ly_str);
        assert_eq!(note, Err(format!("Invalid LilyPond note \"{}\".", ly_str)));
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
        let ly_note = LilyPondNote::try_from(&note).unwrap().note;
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
                dots: Dots::new(1),
            },
        };
        assert_eq!(
            <&LilyPondNote as TryInto<Note>>::try_into(&ly_note).unwrap(),
            test_note
        );
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
                dots: Dots::new(0),
            },
        };
        assert_eq!(
            <&LilyPondNote as TryInto<Note>>::try_into(&ly_note).unwrap(),
            test_note
        );
    }
}
