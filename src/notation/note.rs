//! Abstract types for musical note, describing pitch and rhythm

use crate::lilypond_objects::lilypond_note_string::LilypondNoteString;
use crate::notation::pitch::{Accidental, NoteName, Octave, Pitch};
use crate::notation::rhythm::{Length, NoteDuration, NoteDurationType};

/// A note with rhythm and pitch
pub struct Note {
    pub pitch: Pitch,
    pub rhythm: NoteDuration,
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
    /// use lilypond::notation::rhythm::{NoteDuration, Length, NoteDurationType};
    ///
    /// let note = Note::new(NoteName::A);
    ///
    /// assert_eq!(note.pitch.note, NoteName::A);
    /// assert_eq!(note.pitch.octave, Octave::S3);
    /// assert_eq!(note.pitch.accidental, Accidental::None);
    /// assert_eq!(note.rhythm.length, Length::Quarter);
    /// assert_eq!(note.rhythm.dotted, false);
    /// assert_eq!(note.rhythm.duration_type, NoteDurationType::Note);
    /// ```
    pub fn new(note: NoteName) -> Note {
        Note {
            pitch: Pitch::new(note),
            rhythm: NoteDuration::new(),
        }
    }
    fn get_lilypond_note_name(&self) -> String {
        match self.rhythm.duration_type {
            NoteDurationType::Rest => String::from("r"),
            NoteDurationType::Note => match self.pitch.note {
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
            NoteDurationType::Rest => String::from(""),
            NoteDurationType::Note => match self.pitch.accidental {
                Accidental::Flat => String::from("f"),
                Accidental::Sharp => String::from("s"),
                Accidental::None => String::from(""),
            },
        }
    }
    fn get_lilypond_octave(&self) -> String {
        match self.rhythm.duration_type {
            NoteDurationType::Rest => String::from(""),
            NoteDurationType::Note => match self.pitch.octave {
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
    /// let ly_note = note.to_lilypond_note_string();
    ///
    /// assert_eq!("a4", ly_note.name);
    /// ```
    pub fn to_lilypond_note_string(&self) -> LilypondNoteString {
        let note_name = self.get_lilypond_note_name();
        let note_accidental = self.get_lilypond_accidental();
        let note_octave = self.get_lilypond_octave();
        let note_duration = self.get_lilypond_duration();
        let note_dot = self.get_lilypond_dot();

        LilypondNoteString {
            name: format!(
                "{}{}{}{}{}",
                note_name, note_accidental, note_octave, note_duration, note_dot
            ),
        }
    }
    /// Translate a lilypond note string to a note object
    ///
    /// # Usage
    ///
    /// ```
    /// use lilypond::notation::note::Note;
    /// use lilypond::lilypond_objects::lilypond_note_string::LilypondNoteString;
    /// use lilypond::notation::pitch::{Pitch, NoteName, Accidental, Octave};
    /// use lilypond::notation::rhythm::{NoteDuration, Length, NoteDurationType};
    ///
    /// let ly_note = LilypondNoteString::new("af'4.");
    /// let note = Note::from_lilypond_note_string(ly_note);
    ///
    /// assert_eq!(note.pitch.note, NoteName::A);
    /// assert_eq!(note.pitch.octave, Octave::S4);
    /// assert_eq!(note.pitch.accidental, Accidental::Flat);
    /// assert_eq!(note.rhythm.length, Length::Quarter);
    /// assert_eq!(note.rhythm.dotted, true);
    /// assert_eq!(note.rhythm.duration_type, NoteDurationType::Note);
    /// ```
    pub fn from_lilypond_note_string(ly_note: LilypondNoteString) -> Note {
        ly_note.to_note()
    }
}
}
