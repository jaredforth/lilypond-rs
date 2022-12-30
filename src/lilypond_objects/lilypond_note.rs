//! Abstractions for LilyPond-syntax `String` representations of notes.

use crate::notation::note::Note;
use crate::notation::pitch::{Accidental, NoteName, Octave};
use crate::notation::rhythm::{DurationType, Length};
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

    /// Translate a note object into a lilypond note string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lilypond::lilypond_objects::lilypond_note::LilyPondNote;
    /// use lilypond::notation::note::Note;
    /// use lilypond::notation::pitch::NoteName;
    ///
    /// let ly_note = Note::new(NoteName::A).to_lilypond_note();
    /// assert_eq!(ly_note.get_note(), "a4");
    /// ```
    pub fn from_note(note: Note) -> LilyPondNote {
        note.to_lilypond_note()
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

    fn get_duration_type(&self) -> DurationType {
        match self.get_capture("note_name").as_str() {
            "r" => DurationType::Rest,
            _ => DurationType::Note,
        }
    }

    fn get_note_name(&self) -> NoteName {
        match self.get_duration_type() {
            DurationType::Rest => NoteName::None,
            DurationType::Note => match self.get_capture("note_name").as_str() {
                "a" => NoteName::A,
                "b" => NoteName::B,
                "c" => NoteName::C,
                "d" => NoteName::D,
                "e" => NoteName::E,
                "f" => NoteName::F,
                "g" => NoteName::G,
                e => panic!("Invalid note name '{}'.", e),
            },
        }
    }

    fn get_accidental(&self) -> Accidental {
        match self.get_duration_type() {
            DurationType::Rest => Accidental::None,
            DurationType::Note => match self.get_capture("accidental").as_str() {
                "" => Accidental::None,
                "s" => Accidental::Sharp,
                "is" => Accidental::Sharp,
                "ss" => Accidental::DoubleSharp,
                "isis" => Accidental::DoubleSharp,
                "f" => Accidental::Flat,
                "es" => Accidental::Flat,
                "ff" => Accidental::DoubleFlat,
                "eses" => Accidental::DoubleFlat,
                e => panic!("Invalid accidental '{}'.", e),
            },
        }
    }

    fn get_octave(&self) -> Octave {
        match self.get_duration_type() {
            DurationType::Rest => Octave::None,
            DurationType::Note => {
                // octave has to be usize to add count() results from it
                let mut octave_int: usize = 3;
                let octave_string = self.get_capture("octave");

                if octave_string.contains(",") && octave_string.contains("'") {
                    // Check for both octave transposition characters and panic
                    panic!("Mixed octave transpostion symbols , and '.");
                } else if octave_string.contains("'") {
                    octave_int += octave_string.matches("'").count();
                } else if octave_string.contains(",") {
                    octave_int -= octave_string.matches(",").count();
                }

                match octave_int {
                    0 => Octave::S0,
                    1 => Octave::S1,
                    2 => Octave::S2,
                    3 => Octave::S3,
                    4 => Octave::S4,
                    5 => Octave::S5,
                    6 => Octave::S6,
                    7 => Octave::S7,
                    8 => Octave::S8,
                    9 => Octave::S9,
                    _ => panic!("Invalid number of octave transpositions."),
                }
            }
        }
    }

    fn get_length(&self) -> Length {
        match self.get_capture("duration").as_str() {
            "1" => Length::Whole,
            "2" => Length::Half,
            "4" => Length::Quarter,
            "8" => Length::Eighth,
            "16" => Length::Sixteenth,
            "32" => Length::ThirtySecond,
            "64" => Length::SixtyFourth,
            "128" => Length::OneTwentyEighth,
            "" => Default::default(),
            e => panic!("Invalid duration '{}'.", e),
        }
    }

    fn get_dot(&self) -> bool {
        self.get_capture("dot") == "."
    }

    /// Translate a lilypond note string to a note object.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lilypond::lilypond_objects::lilypond_note::LilyPondNote;
    /// use lilypond::notation::note::Note;
    /// use lilypond::notation::pitch::{Pitch, NoteName, Accidental, Octave};
    /// use lilypond::notation::rhythm::{Rhythm, Length, DurationType};
    ///
    /// let note = LilyPondNote::new("af,8.").unwrap().to_note();
    ///
    /// assert_eq!(note.pitch.note_name, NoteName::A);
    /// assert_eq!(note.pitch.octave, Octave::S2);
    /// assert_eq!(note.pitch.accidental, Accidental::Flat);
    /// assert_eq!(note.rhythm.length, Length::Eighth);
    /// assert_eq!(note.rhythm.dotted, true);
    /// assert_eq!(note.rhythm.duration_type, DurationType::Note);
    /// ```
    pub fn to_note(&self) -> Note {
        let note_name = self.get_note_name();
        let note_accidental = self.get_accidental();
        let note_octave = self.get_octave();
        let note_length = self.get_length();
        let note_duration_type = self.get_duration_type();
        let note_dot = self.get_dot();

        let mut note = Note::new(note_name);
        note.pitch.accidental(note_accidental);
        note.pitch.octave(note_octave);
        note.rhythm.duration_type(note_duration_type);
        note.rhythm.length(note_length);
        note.rhythm.dotted(note_dot);

        return note;
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
        let ly_note = LilyPondNote::from_note(note).note;
        assert_eq!("a4", ly_note);
    }
    #[test]
    fn test_get_duration_type() {
        let ly_note = LilyPondNote::new("r8").unwrap();
        let duration_type = ly_note.get_duration_type();
        assert_eq!(duration_type, DurationType::Rest);
        let ly_note = LilyPondNote::new("f8").unwrap();
        let duration_type = ly_note.get_duration_type();
        assert_eq!(duration_type, DurationType::Note);
    }
    #[test]
    fn test_get_note_name() {
        let ly_note = LilyPondNote::new("r8").unwrap();
        let duration_type = ly_note.get_note_name();
        assert_eq!(duration_type, NoteName::None);
        let ly_note = LilyPondNote::new("f8").unwrap();
        let duration_type = ly_note.get_note_name();
        assert_eq!(duration_type, NoteName::F);
    }
    #[test]
    fn test_get_accidental() {
        let ly_note = LilyPondNote::new("r8").unwrap();
        let accidental_type = ly_note.get_accidental();
        assert_eq!(accidental_type, Accidental::None);
        let ly_note = LilyPondNote::new("fs").unwrap();
        let accidental_type = ly_note.get_accidental();
        assert_eq!(accidental_type, Accidental::Sharp);
        let ly_note = LilyPondNote::new("ef").unwrap();
        let accidental_type = ly_note.get_accidental();
        assert_eq!(accidental_type, Accidental::Flat);
    }
    #[test]
    fn test_get_octave() {
        let ly_note = LilyPondNote::new("r8").unwrap();
        let octave = ly_note.get_octave();
        assert_eq!(octave, Octave::None);
        let ly_note = LilyPondNote::new("fs,,,").unwrap();
        let octave = ly_note.get_octave();
        assert_eq!(octave, Octave::S0);
        let ly_note = LilyPondNote::new("ef").unwrap();
        let octave = ly_note.get_octave();
        assert_eq!(octave, Octave::S3);
        let ly_note = LilyPondNote::new("d''''''").unwrap();
        let octave = ly_note.get_octave();
        assert_eq!(octave, Octave::S9);
    }
    #[test]
    fn test_get_length() {
        let ly_note = LilyPondNote::new("r8").unwrap();
        let length = ly_note.get_length();
        assert_eq!(length, Length::Eighth);
        let ly_note = LilyPondNote::new("as,128").unwrap();
        let length = ly_note.get_length();
        assert_eq!(length, Length::OneTwentyEighth);
        let ly_note = LilyPondNote::new("bf''''64").unwrap();
        let length = ly_note.get_length();
        assert_eq!(length, Length::SixtyFourth);
    }
    #[test]
    fn get_dot() {
        let ly_note = LilyPondNote::new("r8.").unwrap();
        assert!(ly_note.get_dot());
        let ly_note = LilyPondNote::new("r8").unwrap();
        assert!(!ly_note.get_dot());
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
        assert_eq!(ly_note.to_note(), test_note);
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
        assert_eq!(ly_note.to_note(), test_note);
    }
}
