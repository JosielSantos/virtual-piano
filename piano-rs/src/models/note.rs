#[derive(Debug, Clone, Copy)]
pub struct Note {
    number: u8,
}

impl Note {
    pub fn new(number: u8) -> Self {
        assert!(number <= 127, "Note number must be 0-127");
        Note { number }
    }

    pub fn get_number(&self) -> u8 {
        self.number
    }

    pub fn semitone_up(&mut self, total: u8) -> bool {
        let new_number = self.number as i16 + total as i16;
        if new_number > 120 {
            return false;
        }
        self.number = new_number as u8;
        true
    }

    pub fn semitone_down(&mut self, total: u8) -> bool {
        let new_number = self.number as i16 - total as i16;
        if new_number < 0 {
            return false;
        }
        self.number = new_number as u8;
        true
    }
}
