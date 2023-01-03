use crate::{
    lilypond_objects::lilypond_note::LilyPondNote,
    notation::{
        note::Note,
        pitch::Octave,
        rhythm::{DurationType, Length, Rhythm},
    },
};

pub fn octave(note: &Note) -> &str {
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
    }
}

pub fn length(note: &Note) -> &str {
    match note.rhythm.length {
        Length::Whole => "1",
        Length::Half => "2",
        Length::Quarter => "4",
        Length::Eighth => "8",
        Length::Sixteenth => "16",
        Length::ThirtySecond => "32",
        Length::SixtyFourth => "64",
        Length::OneTwentyEighth => "128",
    }
}

pub fn dotted(note: &Note) -> &str {
    match note.rhythm.dotted {
        true => ".",
        false => "",
    }
}

pub fn duration_type_from_lilypond(note: &LilyPondNote) -> DurationType {
    match note.get_capture("note_name").as_str() {
        "r" => DurationType::Rest,
        _ => DurationType::Note,
    }
}

pub fn octave_from_lilypond(note: &LilyPondNote) -> Octave {
    match duration_type_from_lilypond(note) {
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

pub fn rhythm_from_lilypond(note: &LilyPondNote) -> Rhythm {
    Rhythm {
        duration_type: duration_type_from_lilypond(note),
        length: length_from_lilypond(note),
        dotted: dotted_from_lilypond(note),
    }
}

fn length_from_lilypond(note: &LilyPondNote) -> Length {
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

fn dotted_from_lilypond(note: &LilyPondNote) -> bool {
    note.get_capture("dot") == "."
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lilypond_objects::lilypond_note::LilyPondNote;
    #[test]
    fn test_get_duration_type() {
        let note = LilyPondNote::new("r8").unwrap();
        let duration_type = duration_type_from_lilypond(&note);
        assert_eq!(duration_type, DurationType::Rest);
        let note = LilyPondNote::new("f8").unwrap();
        let duration_type = duration_type_from_lilypond(&note);
        assert_eq!(duration_type, DurationType::Note);
    }
    #[test]
    fn test_octave_from_lilypond() {
        let ly_note = LilyPondNote::new("r8").unwrap();
        let octave = octave_from_lilypond(&ly_note);
        assert_eq!(octave, Octave::None);
        let ly_note = LilyPondNote::new("fs,,,").unwrap();
        let octave = octave_from_lilypond(&ly_note);
        assert_eq!(octave, Octave::S0);
        let ly_note = LilyPondNote::new("ef").unwrap();
        let octave = octave_from_lilypond(&ly_note);
        assert_eq!(octave, Octave::S3);
        let ly_note = LilyPondNote::new("d''''''").unwrap();
        let octave = octave_from_lilypond(&ly_note);
        assert_eq!(octave, Octave::S9);
    }
    #[test]
    fn test_length_from_lilypond() {
        let ly_note = LilyPondNote::new("r8").unwrap();
        let length = length_from_lilypond(&ly_note);
        assert_eq!(length, Length::Eighth);
        let ly_note = LilyPondNote::new("as,128").unwrap();
        let length = length_from_lilypond(&ly_note);
        assert_eq!(length, Length::OneTwentyEighth);
        let ly_note = LilyPondNote::new("bf''''64").unwrap();
        let length = length_from_lilypond(&ly_note);
        assert_eq!(length, Length::SixtyFourth);
    }
    #[test]
    fn get_dot() {
        let ly_note = LilyPondNote::new("r8.").unwrap();
        assert!(dotted_from_lilypond(&ly_note));
        let ly_note = LilyPondNote::new("r8").unwrap();
        assert!(!dotted_from_lilypond(&ly_note));
    }
}
