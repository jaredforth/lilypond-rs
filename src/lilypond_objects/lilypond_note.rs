//! Abstractions for LilyPond-syntax `String` representations of notes.

use crate::notation::note::Note;
use crate::notation::pitch::{Accidental, NoteName, Octave};
use crate::notation::rhythm::{DurationType, Length};
use regex::Regex;

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
    fn get_duration_type(&self) -> DurationType {
        match &self.name[0..1] {
            "r" => DurationType::Rest,
            _ => DurationType::Note,
        }
    }
    fn get_note_name(&self) -> NoteName {
        match self.get_duration_type() {
            DurationType::Rest => NoteName::None,
            DurationType::Note => match &self.name[0..1] {
                "a" => NoteName::A,
                "b" => NoteName::B,
                "c" => NoteName::C,
                "d" => NoteName::D,
                "e" => NoteName::E,
                "f" => NoteName::F,
                "g" => NoteName::G,
                _ => panic!("Invalid note name."),
            },
        }
    }
    fn get_accidental(&self) -> Accidental {
        match self.get_duration_type() {
            DurationType::Rest => Accidental::None,
            DurationType::Note => {
                // Must exclude first character, note names can have f in them
                let note_without_name = &self.name[1..];
                if note_without_name.contains("s") {
                    Accidental::Sharp
                } else if note_without_name.contains("f") {
                    Accidental::Flat
                } else {
                    Accidental::None
                }
            }
        }
    }
    fn get_octave(&self) -> Octave {
        let ly_note = &self.name;
        match self.get_duration_type() {
            DurationType::Rest => Octave::None,
            DurationType::Note => {
                // octave has to be usize to add count() results from it
                let mut octave: usize = 3;
                if ly_note.contains(",") && ly_note.contains("'") {
                    // Check for both octave transposition characters and panic
                    panic!("Mixed octave transpostion symbols , and '.");
                } else if ly_note.contains("'") {
                    octave += ly_note.matches("'").count();
                } else if ly_note.contains(",") {
                    octave -= ly_note.matches(",").count();
                }
                match octave {
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
        let re = Regex::new(
            r"(?x-u) # Flags: x = whitespace allowed, -u = no unicode support
            ^[a-gr] # note name or rest
            (?:f|s)? # accidental
            (?:(?:,{0,3})|(?:'{0,6}))? # octave transposition characters
            ((?:1|2|4|8|(?:16)|(?:32)|(?:64)|(?:128))?) # Durations, captured
            \.?$ # optional dot and end of line
            ",
        )
        .unwrap();
        let duration = re.captures(&self.name).unwrap();
        let duration_str: &str = &duration.get(1).map_or("", |m| m.as_str());
        match duration_str {
            "1" => Length::Whole,
            "2" => Length::Half,
            "4" => Length::Quarter,
            "8" => Length::Eighth,
            "16" => Length::Sixteenth,
            "32" => Length::ThirtySecond,
            "64" => Length::SixtyFourth,
            "128" => Length::OneTwentyEighth,
            "" => Default::default(),
            _ => panic!("Invalid duration."),
        }
    }
    fn get_dot(&self) -> bool {
        return self.name.contains(".");
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
            "gs''''''16.",
        ];
        for n in ly_notes {
            test_lilypond_note(n);
        }
    }
    #[test]
    #[should_panic]
    #[allow(unused_variables)]
    fn test_new_lilypond_note_panic() {
        let note = LilypondNote::new("asdf");
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
