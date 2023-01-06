//! Common functions for parsing and encoding LilyPond

use std::convert::TryInto;

use crate::{
    lilypond_objects::lilypond_note::LilyPondNote,
    notation::{
        pitch::Octave,
        rhythm::{Dots, DurationType, Length, Rhythm},
    },
};

pub fn duration_type_from_lilypond(note: &LilyPondNote) -> DurationType {
    match note.get_capture("note_name").as_str() {
        "r" => DurationType::Rest,
        _ => DurationType::Note,
    }
}

pub fn octave_from_lilypond(note: &LilyPondNote) -> Result<Octave, String> {
    match duration_type_from_lilypond(note) {
        DurationType::Rest => Ok(Octave::None),
        DurationType::Note => {
            // octave has to be usize to add count() results from it
            let mut octave_int: usize = 3;
            let octave_string = note.get_capture("octave");

            if octave_string.contains(",") && octave_string.contains("'") {
                // Check for both octave transposition characters and return an
                // error
                return Err(format!("Invalid octave indication \"{}\".", octave_string));
            } else if octave_string.contains("'") {
                octave_int += octave_string.matches("'").count();
            } else if octave_string.contains(",") {
                octave_int -= octave_string.matches(",").count();
            }

            match octave_int {
                0 => Ok(Octave::S0),
                1 => Ok(Octave::S1),
                2 => Ok(Octave::S2),
                3 => Ok(Octave::S3),
                4 => Ok(Octave::S4),
                5 => Ok(Octave::S5),
                6 => Ok(Octave::S6),
                7 => Ok(Octave::S7),
                8 => Ok(Octave::S8),
                9 => Ok(Octave::S9),
                _ => Err(format!("Invalid octave indication \"{}\".", octave_string)),
            }
        }
    }
}

pub fn rhythm_from_lilypond(note: &LilyPondNote) -> Result<Rhythm, String> {
    Ok(Rhythm {
        duration_type: duration_type_from_lilypond(note),
        length: length_from_lilypond(note)?,
        dots: dotted_from_lilypond(note)?,
    })
}

fn length_from_lilypond(note: &LilyPondNote) -> Result<Length, String> {
    match note.get_capture("duration").as_str() {
        "1" => Ok(Length::Whole),
        "2" => Ok(Length::Half),
        "4" => Ok(Length::Quarter),
        "8" => Ok(Length::Eighth),
        "16" => Ok(Length::Sixteenth),
        "32" => Ok(Length::ThirtySecond),
        "64" => Ok(Length::SixtyFourth),
        "128" => Ok(Length::OneTwentyEighth),
        "" => Ok(Default::default()),
        e => Err(format!("Invalid duration '{}'.", e)),
    }
}

fn dotted_from_lilypond(note: &LilyPondNote) -> Result<Dots, String> {
    let num_dots = note.get_capture("dot").matches(".").count().try_into();
    match num_dots {
        Ok(n) => Ok(Dots::new(n)),
        Err(e) => Err(format!("Invalid number of dots {}.", e)),
    }
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
        let octave = octave_from_lilypond(&ly_note).unwrap();
        assert_eq!(octave, Octave::None);
        let ly_note = LilyPondNote::new("fs,,,").unwrap();
        let octave = octave_from_lilypond(&ly_note).unwrap();
        assert_eq!(octave, Octave::S0);
        let ly_note = LilyPondNote::new("ef").unwrap();
        let octave = octave_from_lilypond(&ly_note).unwrap();
        assert_eq!(octave, Octave::S3);
        let ly_note = LilyPondNote::new("d''''''").unwrap();
        let octave = octave_from_lilypond(&ly_note).unwrap();
        assert_eq!(octave, Octave::S9);
    }
    #[test]
    fn test_length_from_lilypond() {
        let ly_note = LilyPondNote::new("r8").unwrap();
        let length = length_from_lilypond(&ly_note).unwrap();
        assert_eq!(length, Length::Eighth);
        let ly_note = LilyPondNote::new("as,128").unwrap();
        let length = length_from_lilypond(&ly_note).unwrap();
        assert_eq!(length, Length::OneTwentyEighth);
        let ly_note = LilyPondNote::new("bf''''64").unwrap();
        let length = length_from_lilypond(&ly_note).unwrap();
        assert_eq!(length, Length::SixtyFourth);
    }
    #[test]
    fn get_dot() {
        let ly_note = LilyPondNote::new("r8.").unwrap();
        assert_eq!(dotted_from_lilypond(&ly_note).unwrap().get_num_dots(), 1);
        let ly_note = LilyPondNote::new("r8").unwrap();
        assert_eq!(dotted_from_lilypond(&ly_note).unwrap().get_num_dots(), 0);
        let ly_note = LilyPondNote::new("r8............").unwrap();
        assert_eq!(dotted_from_lilypond(&ly_note).unwrap().get_num_dots(), 12);
    }
}
