//! A module for English LilyPond note input.

use super::common;
use crate::{
    lilypond_objects::lilypond_note::LilyPondNote,
    notation::{
        note::Note,
        pitch::{Accidental, NoteName, Pitch},
        rhythm::DurationType,
    },
};

pub static LANGUAGE_STR: &str = "english";

pub static NOTE_REGEX_STR: &str = r"(?x-u)
    # Flags: x = whitespace allowed, -u = no unicode support
    ^(?P<note_name>[a-gr]) # note name or rest
    (?P<accidental>(?:f{0,2}|s{0,2})| # one-letter accidentals
    (?:(?:-sharp)?|(?:-flat)?|(?:-sharpsharp)|(?:-flatflat))) # spelled-out accidentals
    (?P<octave>(?:(?:,{0,3})|(?:'{0,6}))?) # octave transposition characters
    (?P<duration>(?:1|2|4|8|(?:16)|(?:32)|(?:64)|(?:128))?) # Durations
    (?P<dot>\.{0,255})$ # optional dots and end of line
    ";

pub fn lilypond_from_note(note: &Note) -> String {
    format!(
        "{}{}{}{}{}",
        lilypond_from_note_name(note),
        lilypond_from_accidental(note),
        note.pitch.octave,
        note.rhythm.length,
        note.rhythm.dots,
    )
}

fn lilypond_from_note_name(note: &Note) -> &str {
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
    }
}

fn lilypond_from_accidental(note: &Note) -> &str {
    match note.pitch.accidental {
        Accidental::None => "",
        Accidental::Flat => "f",
        Accidental::DoubleFlat => "ff",
        Accidental::Sharp => "s",
        Accidental::DoubleSharp => "ss",
    }
}

pub fn note_from_lilypond(note: &LilyPondNote) -> Result<Note, String> {
    Ok(Note {
        pitch: Pitch {
            note_name: note_name_from_lilypond(note)?,
            accidental: accidental_from_lilypond(note)?,
            octave: common::octave_from_lilypond(note)?,
        },
        rhythm: common::rhythm_from_lilypond(note)?,
    })
}

fn note_name_from_lilypond(note: &LilyPondNote) -> Result<NoteName, String> {
    match common::duration_type_from_lilypond(note) {
        DurationType::Rest => Ok(NoteName::None),
        DurationType::Note => match note.get_capture("note_name").as_str() {
            "a" => Ok(NoteName::A),
            "b" => Ok(NoteName::B),
            "c" => Ok(NoteName::C),
            "d" => Ok(NoteName::D),
            "e" => Ok(NoteName::E),
            "f" => Ok(NoteName::F),
            "g" => Ok(NoteName::G),
            e => Err(format!("Invalid note name '{}'.", e)),
        },
    }
}

fn accidental_from_lilypond(note: &LilyPondNote) -> Result<Accidental, String> {
    match common::duration_type_from_lilypond(note) {
        DurationType::Rest => Ok(Accidental::None),
        DurationType::Note => match note.get_capture("accidental").as_str() {
            "" => Ok(Accidental::None),
            "s" => Ok(Accidental::Sharp),
            "ss" => Ok(Accidental::DoubleSharp),
            "f" => Ok(Accidental::Flat),
            "ff" => Ok(Accidental::DoubleFlat),
            e => Err(format!("Invalid accidental '{}'.", e)),
        },
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use lazy_static::lazy_static;
    use regex::Regex;
    lazy_static! {
        static ref LILYPOND_NOTE_REGEX: Regex = Regex::new(NOTE_REGEX_STR).unwrap();
    }
    #[test]
    fn test_lilypond_from_note_name() {
        let note = Note::new(NoteName::A);
        let note_name_str = lilypond_from_note_name(&note);
        assert_eq!("a", note_name_str);
        let note = Note::new(NoteName::B);
        let note_name_str = lilypond_from_note_name(&note);
        assert_eq!("b", note_name_str);
        let note = Note::new(NoteName::C);
        let note_name_str = lilypond_from_note_name(&note);
        assert_eq!("c", note_name_str);
        let note = Note::new(NoteName::D);
        let note_name_str = lilypond_from_note_name(&note);
        assert_eq!("d", note_name_str);
        let note = Note::new(NoteName::E);
        let note_name_str = lilypond_from_note_name(&note);
        assert_eq!("e", note_name_str);
        let note = Note::new(NoteName::F);
        let note_name_str = lilypond_from_note_name(&note);
        assert_eq!("f", note_name_str);
        let note = Note::new(NoteName::G);
        let note_name_str = lilypond_from_note_name(&note);
        assert_eq!("g", note_name_str);
    }
    #[test]
    fn test_lilypond_from_accidental() {
        let mut note = Note::new(NoteName::A);
        let accidental_str = lilypond_from_accidental(&note);
        assert_eq!("", accidental_str);
        note.pitch.accidental(Accidental::Flat);
        let accidental_str = lilypond_from_accidental(&note);
        assert_eq!("f", accidental_str);
        note.pitch.accidental(Accidental::Sharp);
        let accidental_str = lilypond_from_accidental(&note);
        assert_eq!("s", accidental_str);
        note.pitch.accidental(Accidental::DoubleFlat);
        let accidental_str = lilypond_from_accidental(&note);
        assert_eq!("ff", accidental_str);
        note.pitch.accidental(Accidental::DoubleSharp);
        let accidental_str = lilypond_from_accidental(&note);
        assert_eq!("ss", accidental_str);
    }
    fn test_regex_case(note: &str) {
        assert!(LILYPOND_NOTE_REGEX.is_match(note));
    }
    #[test]
    fn test_regex() {
        let notes = [
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
        for n in notes {
            test_regex_case(n);
        }
    }
    #[test]
    fn test_accidental_from_lilypond() {
        let ly_note = LilyPondNote::new("r8").unwrap();
        let accidental_type = accidental_from_lilypond(&ly_note).unwrap();
        assert_eq!(accidental_type, Accidental::None);
        let ly_note = LilyPondNote::new("fs").unwrap();
        let accidental_type = accidental_from_lilypond(&ly_note).unwrap();
        assert_eq!(accidental_type, Accidental::Sharp);
        let ly_note = LilyPondNote::new("ef").unwrap();
        let accidental_type = accidental_from_lilypond(&ly_note).unwrap();
        assert_eq!(accidental_type, Accidental::Flat);
        let ly_note = LilyPondNote::new("gss").unwrap();
        let accidental_type = accidental_from_lilypond(&ly_note).unwrap();
        assert_eq!(accidental_type, Accidental::DoubleSharp);
        let ly_note = LilyPondNote::new("aff").unwrap();
        let accidental_type = accidental_from_lilypond(&ly_note).unwrap();
        assert_eq!(accidental_type, Accidental::DoubleFlat);
    }
    #[test]
    fn test_note_name_from_lilypond() {
        let ly_note = LilyPondNote::new("r8").unwrap();
        let note_name = note_name_from_lilypond(&ly_note).unwrap();
        assert_eq!(note_name, NoteName::None);
        let ly_note = LilyPondNote::new("fs").unwrap();
        let note_name = note_name_from_lilypond(&ly_note).unwrap();
        assert_eq!(note_name, NoteName::F);
        let ly_note = LilyPondNote::new("ef").unwrap();
        let note_name = note_name_from_lilypond(&ly_note).unwrap();
        assert_eq!(note_name, NoteName::E);
        let ly_note = LilyPondNote::new("gss").unwrap();
        let note_name = note_name_from_lilypond(&ly_note).unwrap();
        assert_eq!(note_name, NoteName::G);
        let ly_note = LilyPondNote::new("aff").unwrap();
        let note_name = note_name_from_lilypond(&ly_note).unwrap();
        assert_eq!(note_name, NoteName::A);
    }
}
