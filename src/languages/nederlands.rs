//! A module for Dutch LilyPond note input.

use super::common;
use crate::{
    lilypond_objects::lilypond_note::LilyPondNote,
    notation::{
        note::Note,
        pitch::{Accidental, NoteName, Pitch},
        rhythm::DurationType,
    },
};

pub static LANGUAGE_STR: &str = "nederlands";

pub static NOTE_REGEX_STR: &str = r"(?x-u)
    # Flags: x = whitespace allowed, -u = no unicode support
    ^(?P<note_name>[a-gr]) # note name or rest
    (?P<accidental>(?:is){0,2}|(?:es){0,2}) # accidental
    (?P<octave>(?:(?:,{0,3})|(?:'{0,6}))?) # octave transposition characters
    (?P<duration>(?:1|2|4|8|(?:16)|(?:32)|(?:64)|(?:128))?) # Durations
    (?P<dot>\.{0,255})$ # optional dot and end of line
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
        Accidental::Flat => "es",
        Accidental::DoubleFlat => "eses",
        Accidental::Sharp => "is",
        Accidental::DoubleSharp => "isis",
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
            "is" => Ok(Accidental::Sharp),
            "isis" => Ok(Accidental::DoubleSharp),
            "es" => Ok(Accidental::Flat),
            "eses" => Ok(Accidental::DoubleFlat),
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
    fn test_note_name() {
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
    fn test_accidental() {
        let mut note = Note::new(NoteName::A);
        let accidental_str = lilypond_from_accidental(&note);
        assert_eq!("", accidental_str);
        note.pitch.accidental(Accidental::Flat);
        let accidental_str = lilypond_from_accidental(&note);
        assert_eq!("es", accidental_str);
        note.pitch.accidental(Accidental::Sharp);
        let accidental_str = lilypond_from_accidental(&note);
        assert_eq!("is", accidental_str);
        note.pitch.accidental(Accidental::DoubleFlat);
        let accidental_str = lilypond_from_accidental(&note);
        assert_eq!("eses", accidental_str);
        note.pitch.accidental(Accidental::DoubleSharp);
        let accidental_str = lilypond_from_accidental(&note);
        assert_eq!("isis", accidental_str);
    }
    fn test_regex_case(note: &str) {
        assert!(LILYPOND_NOTE_REGEX.is_match(note));
    }
    #[test]
    fn test_regex() {
        let notes = [
            "r",
            "a",
            "bes",
            "cis",
            "d,",
            "ees'",
            "fis",
            "g,,",
            "aes''1",
            "bis2",
            "c,,,4",
            "des'''8",
            "eis16",
            "f32",
            "ges''''64",
            "ais128",
            "b,",
            "ces'''''1.",
            "dis,,2.",
            "e4.",
            "fes,,,8.",
            "feses,,,8.",
            "gis''''''16.",
        ];
        for note in notes {
            test_regex_case(note);
            println!("{}", note);
        }
    }
    // not currently testing these two because I can't change the value of
    // NOTE_NAME_LANGUAGE to change the LILYPOND_NOTE_REGEX.
    #[allow(dead_code)]
    fn test_accidental_from_lilypond() {
        let ly_note = LilyPondNote::new("r8").unwrap();
        let accidental_type = accidental_from_lilypond(&ly_note).unwrap();
        assert_eq!(accidental_type, Accidental::None);
        let ly_note = LilyPondNote::new("fis").unwrap();
        let accidental_type = accidental_from_lilypond(&ly_note).unwrap();
        assert_eq!(accidental_type, Accidental::Sharp);
        let ly_note = LilyPondNote::new("ees").unwrap();
        let accidental_type = accidental_from_lilypond(&ly_note).unwrap();
        assert_eq!(accidental_type, Accidental::Flat);
        let ly_note = LilyPondNote::new("gisis").unwrap();
        let accidental_type = accidental_from_lilypond(&ly_note).unwrap();
        assert_eq!(accidental_type, Accidental::DoubleSharp);
        let ly_note = LilyPondNote::new("aeses").unwrap();
        let accidental_type = accidental_from_lilypond(&ly_note).unwrap();
        assert_eq!(accidental_type, Accidental::DoubleFlat);
    }
    #[allow(dead_code)]
    fn test_note_name_from_lilypond() {
        let ly_note = LilyPondNote::new("r8").unwrap();
        let note_name = note_name_from_lilypond(&ly_note).unwrap();
        assert_eq!(note_name, NoteName::None);
        let ly_note = LilyPondNote::new("fis").unwrap();
        let note_name = note_name_from_lilypond(&ly_note).unwrap();
        assert_eq!(note_name, NoteName::F);
        let ly_note = LilyPondNote::new("ees").unwrap();
        let note_name = note_name_from_lilypond(&ly_note).unwrap();
        assert_eq!(note_name, NoteName::E);
        let ly_note = LilyPondNote::new("gisis").unwrap();
        let note_name = note_name_from_lilypond(&ly_note).unwrap();
        assert_eq!(note_name, NoteName::G);
        let ly_note = LilyPondNote::new("aeses").unwrap();
        let note_name = note_name_from_lilypond(&ly_note).unwrap();
        assert_eq!(note_name, NoteName::A);
    }
}
