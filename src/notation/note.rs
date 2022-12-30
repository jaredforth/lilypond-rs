//! Abstract types for musical note, describing pitch and rhythm

use crate::lilypond_objects::lilypond_note::LilyPondNote;
use crate::notation::pitch::{Accidental, NoteName, Octave, Pitch};
use crate::notation::rhythm::{DurationType, Length, Rhythm};

/// A note with rhythm and pitch
#[derive(Debug, PartialEq)]
pub struct Note {
    pub pitch: Pitch,
    pub rhythm: Rhythm,
}

/// TODO shorthand for creating a new note with pitch and duration
/// TODO also need to create staff, time signature, key signature, chord
/// before we have a minimum viable product
impl Note {
    /// Construct a new note
    ///
    /// This will take a note name and initialize a note with default octave and
    /// duration.
    ///
    /// # Usage:
    ///
    /// ```
    /// use lilypond::notation::note::Note;
    /// use lilypond::notation::pitch::{Pitch, NoteName, Accidental, Octave};
    /// use lilypond::notation::rhythm::{Rhythm, Length, DurationType};
    ///
    /// let note = Note::new(NoteName::A);
    ///
    /// assert_eq!(note.pitch.note_name, NoteName::A);
    /// assert_eq!(note.pitch.octave, Octave::S3);
    /// assert_eq!(note.pitch.accidental, Accidental::None);
    /// assert_eq!(note.rhythm.length, Length::Quarter);
    /// assert_eq!(note.rhythm.dotted, false);
    /// assert_eq!(note.rhythm.duration_type, DurationType::Note);
    /// ```
    pub fn new(note_name: NoteName) -> Note {
        Note {
            pitch: Pitch::new(note_name),
            rhythm: Rhythm::new(),
        }
    }
    fn get_lilypond_note_name(&self) -> String {
        match self.rhythm.duration_type {
            DurationType::Rest => String::from("r"),
            DurationType::Note => match self.pitch.note_name {
                NoteName::A => String::from("a"),
                NoteName::B => String::from("b"),
                NoteName::C => String::from("c"),
                NoteName::D => String::from("d"),
                NoteName::E => String::from("e"),
                NoteName::F => String::from("f"),
                NoteName::G => String::from("g"),
                NoteName::None => String::from("r"),
            },
        }
    }
    fn get_lilypond_accidental(&self) -> String {
        match self.rhythm.duration_type {
            DurationType::Rest => String::from(""),
            DurationType::Note => match self.pitch.accidental {
                Accidental::Flat => String::from("f"),
                Accidental::DoubleFlat => String::from("ff"),
                Accidental::Sharp => String::from("s"),
                Accidental::DoubleSharp => String::from("ss"),
                Accidental::None => String::from(""),
            },
        }
    }
    fn get_lilypond_octave(&self) -> String {
        match self.rhythm.duration_type {
            DurationType::Rest => String::from(""),
            DurationType::Note => match self.pitch.octave {
                Octave::S0 => String::from(",,,"),
                Octave::S1 => String::from(",,"),
                Octave::S2 => String::from(","),
                Octave::S3 => String::from(""),
                Octave::S4 => String::from("'"),
                Octave::S5 => String::from("''"),
                Octave::S6 => String::from("'''"),
                Octave::S7 => String::from("''''"),
                Octave::S8 => String::from("'''''"),
                Octave::S9 => String::from("''''''"),
                Octave::None => String::from(""),
            },
        }
    }
    fn get_lilypond_duration(&self) -> String {
        match self.rhythm.length {
            Length::Whole => String::from("1"),
            Length::Half => String::from("2"),
            Length::Quarter => String::from("4"),
            Length::Eighth => String::from("8"),
            Length::Sixteenth => String::from("16"),
            Length::ThirtySecond => String::from("32"),
            Length::SixtyFourth => String::from("64"),
            Length::OneTwentyEighth => String::from("128"),
        }
    }
    fn get_lilypond_dot(&self) -> String {
        match self.rhythm.dotted {
            true => String::from("."),
            false => String::from(""),
        }
    }
    /// Construct a lilypond note string from a note object
    ///
    /// # Usage
    ///
    /// ```
    /// use lilypond::notation::note::Note;
    /// use lilypond::notation::pitch::NoteName;
    ///
    /// let note = Note::new(NoteName::A);
    /// let ly_note = note.to_lilypond_note();
    ///
    /// assert_eq!("a4", ly_note.get_note());
    /// ```
    pub fn to_lilypond_note(&self) -> LilyPondNote {
        let note_name = self.get_lilypond_note_name();
        let note_accidental = self.get_lilypond_accidental();
        let note_octave = self.get_lilypond_octave();
        let note_duration = self.get_lilypond_duration();
        let note_dot = self.get_lilypond_dot();

        LilyPondNote::new(
            format!(
                "{}{}{}{}{}",
                note_name, note_accidental, note_octave, note_duration, note_dot
            )
            .as_str(),
        )
        .unwrap()
    }
}

// private functions for implementing the From<LilyPondNote> trait
impl Note {
    fn get_duration_type(note: &LilyPondNote) -> DurationType {
        match note.get_capture("note_name").as_str() {
            "r" => DurationType::Rest,
            _ => DurationType::Note,
        }
    }

    fn get_note_name(note: &LilyPondNote) -> NoteName {
        match Note::get_duration_type(note) {
            DurationType::Rest => NoteName::None,
            DurationType::Note => match note.get_capture("note_name").as_str() {
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

    fn get_accidental(note: &LilyPondNote) -> Accidental {
        match Note::get_duration_type(note) {
            DurationType::Rest => Accidental::None,
            DurationType::Note => match note.get_capture("accidental").as_str() {
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

    fn get_octave(note: &LilyPondNote) -> Octave {
        match Note::get_duration_type(note) {
            DurationType::Rest => Octave::None,
            DurationType::Note => {
                // octave has to be usize to add count() results from it
                let mut octave_int: usize = 3;
                let octave_string = note.get_capture("octave");

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

    fn get_length(note: &LilyPondNote) -> Length {
        match note.get_capture("duration").as_str() {
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

    fn get_dot(note: &LilyPondNote) -> bool {
        note.get_capture("dot") == "."
    }
}

impl From<&LilyPondNote> for Note {
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
    /// let ly_note = LilyPondNote::new("af,8.").unwrap();
    /// let note = Note::from(&ly_note);
    ///
    /// assert_eq!(note.pitch.note_name, NoteName::A);
    /// assert_eq!(note.pitch.octave, Octave::S2);
    /// assert_eq!(note.pitch.accidental, Accidental::Flat);
    /// assert_eq!(note.rhythm.length, Length::Eighth);
    /// assert_eq!(note.rhythm.dotted, true);
    /// assert_eq!(note.rhythm.duration_type, DurationType::Note);
    /// ```
    fn from(note: &LilyPondNote) -> Note {
        let note_name = Note::get_note_name(note);
        let note_accidental = Note::get_accidental(note);
        let note_octave = Note::get_octave(note);
        let note_length = Note::get_length(note);
        let note_duration_type = Note::get_duration_type(note);
        let note_dot = Note::get_dot(note);

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
mod tests {
    use crate::lilypond_objects::lilypond_note::LilyPondNote;
    use crate::notation::note::Note;
    use crate::notation::pitch::{Accidental, NoteName, Octave};
    use crate::notation::rhythm::{DurationType, Length};
    #[test]
    fn test_get_lilypond_note_name() {
        let note = Note::new(NoteName::A);
        let lilypond_note_name = note.get_lilypond_note_name();
        assert_eq!("a", lilypond_note_name);
    }
    #[test]
    fn test_get_lilypond_accidental() {
        let note = Note::new(NoteName::A);
        let lilypond_accidental = note.get_lilypond_accidental();
        assert_eq!("", lilypond_accidental);
    }
    #[test]
    fn test_get_lilypond_octave() {
        let note = Note::new(NoteName::A);
        let lilypond_octave = note.get_lilypond_octave();
        assert_eq!("", lilypond_octave);
    }
    #[test]
    fn test_get_lilypond_duration() {
        let note = Note::new(NoteName::A);
        let lilypond_duration = note.get_lilypond_duration();
        assert_eq!("4", lilypond_duration);
    }
    #[test]
    fn test_get_lilypond_dot() {
        let note = Note::new(NoteName::A);
        let lilypond_dot = note.get_lilypond_dot();
        assert_eq!("", lilypond_dot);
    }
    #[test]
    fn test_to_lilypond_note() {
        let note = Note::new(NoteName::A);
        let ly_note = note.to_lilypond_note();
        assert_eq!(ly_note.get_note(), "a4");
    }
    #[test]
    fn test_from_lilypond_note() {
        let ly_note = LilyPondNote::new("a4").unwrap();
        let note = Note::from(&ly_note);
        assert_eq!(note.pitch.note_name, NoteName::A);
        assert_eq!(note.pitch.octave, Octave::S3);
        assert_eq!(note.pitch.accidental, Accidental::None);
        assert_eq!(note.rhythm.length, Length::Quarter);
        assert_eq!(note.rhythm.dotted, false);
        assert_eq!(note.rhythm.duration_type, DurationType::Note);
    }
    #[test]
    fn test_get_duration_type() {
        let ly_note = LilyPondNote::new("r8").unwrap();
        let duration_type = Note::get_duration_type(&ly_note);
        assert_eq!(duration_type, DurationType::Rest);
        let ly_note = LilyPondNote::new("f8").unwrap();
        let duration_type = Note::get_duration_type(&ly_note);
        assert_eq!(duration_type, DurationType::Note);
    }
    #[test]
    fn test_get_note_name() {
        let ly_note = LilyPondNote::new("r8").unwrap();
        let duration_type = Note::get_note_name(&ly_note);
        assert_eq!(duration_type, NoteName::None);
        let ly_note = LilyPondNote::new("f8").unwrap();
        let duration_type = Note::get_note_name(&ly_note);
        assert_eq!(duration_type, NoteName::F);
    }
    #[test]
    fn test_get_accidental() {
        let ly_note = LilyPondNote::new("r8").unwrap();
        let accidental_type = Note::get_accidental(&ly_note);
        assert_eq!(accidental_type, Accidental::None);
        let ly_note = LilyPondNote::new("fs").unwrap();
        let accidental_type = Note::get_accidental(&ly_note);
        assert_eq!(accidental_type, Accidental::Sharp);
        let ly_note = LilyPondNote::new("ef").unwrap();
        let accidental_type = Note::get_accidental(&ly_note);
        assert_eq!(accidental_type, Accidental::Flat);
    }
    #[test]
    fn test_get_octave() {
        let ly_note = LilyPondNote::new("r8").unwrap();
        let octave = Note::get_octave(&ly_note);
        assert_eq!(octave, Octave::None);
        let ly_note = LilyPondNote::new("fs,,,").unwrap();
        let octave = Note::get_octave(&ly_note);
        assert_eq!(octave, Octave::S0);
        let ly_note = LilyPondNote::new("ef").unwrap();
        let octave = Note::get_octave(&ly_note);
        assert_eq!(octave, Octave::S3);
        let ly_note = LilyPondNote::new("d''''''").unwrap();
        let octave = Note::get_octave(&ly_note);
        assert_eq!(octave, Octave::S9);
    }
    #[test]
    fn test_get_length() {
        let ly_note = LilyPondNote::new("r8").unwrap();
        let length = Note::get_length(&ly_note);
        assert_eq!(length, Length::Eighth);
        let ly_note = LilyPondNote::new("as,128").unwrap();
        let length = Note::get_length(&ly_note);
        assert_eq!(length, Length::OneTwentyEighth);
        let ly_note = LilyPondNote::new("bf''''64").unwrap();
        let length = Note::get_length(&ly_note);
        assert_eq!(length, Length::SixtyFourth);
    }
    #[test]
    fn get_dot() {
        let ly_note = LilyPondNote::new("r8.").unwrap();
        assert!(Note::get_dot(&ly_note));
        let ly_note = LilyPondNote::new("r8").unwrap();
        assert!(!Note::get_dot(&ly_note));
    }
}
