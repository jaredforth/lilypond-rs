//! Abstractions for encoding notes as MIDI integers.

pub struct MidiNote {
    note: u8,
}

impl MidiNote {
    /// Initialize a new `MidiNote` object with given pitch
    pub fn new(note: u8) -> Self {
        MidiNote { note }
    }

    /// Getter for `note` field in `MidiNote` struct.
    pub fn get_note(&self) -> u8 {
        self.note
    }

    /// Setter for `note` field in `MidiNote` struct.
    pub fn set_note(&mut self, note: u8) {
        self.note = note;
    }
}

impl Default for MidiNote {
    /// Set default `MidiNote` to middle C (MIDI value 60)
    fn default() -> Self {
        MidiNote { note: 60 }
    }
}

#[cfg(test)]
mod test {
    use crate::midi::MidiNote;
    #[test]
    fn test_default() {
        let midi_note = MidiNote::default();
        assert_eq!(midi_note.get_note(), 60);
    }
    #[test]
    fn test_new() {
        let midi_note = MidiNote::new(48);
        assert_eq!(midi_note.get_note(), 48);
    }
    #[test]
    fn test_set() {
        let mut midi_note = MidiNote::default();
        midi_note.set_note(84);
        assert_eq!(midi_note.get_note(), 84);
    }
}
