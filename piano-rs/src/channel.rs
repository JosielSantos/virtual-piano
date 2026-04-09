use std::collections::HashSet;
use std::path::Path;
use crate::notes_manager::NotesManager;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
    Left,
    Center,
    Right,
}

impl Direction {
    pub fn next(self) -> Self {
        match self {
            Direction::Left   => Direction::Center,
            Direction::Center => Direction::Right,
            Direction::Right  => Direction::Left,
        }
    }

    pub fn prev(self) -> Self {
        match self {
            Direction::Right  => Direction::Center,
            Direction::Center => Direction::Left,
            Direction::Left   => Direction::Right,
        }
    }

    pub fn to_midi_pan(self) -> u8 {
        match self {
            Direction::Left   => 0,
            Direction::Center => 64,
            Direction::Right  => 127,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Direction::Left   => "L",
            Direction::Center => "C",
            Direction::Right  => "R",
        }
    }
}

pub struct Channel {
    notes_manager: NotesManager,
    pub notes_on: HashSet<u8>,
    instrument: u8,
    volume: u8,
    direction: Direction,
}

impl Channel {
    pub fn new(keymap_path: &Path) -> anyhow::Result<Self> {
        let mut notes_manager = NotesManager::new();
        notes_manager.load_file(keymap_path)?;
        Ok(Channel {
            notes_manager,
            notes_on: HashSet::new(),
            instrument: 0,
            volume: 127,
            direction: Direction::Center,
        })
    }

    pub fn get_note(&self, key: &str) -> Option<u8> {
        self.notes_manager.get_note(key).map(|n| n.get_number())
    }

    pub fn get_instrument(&self) -> u8 {
        self.instrument
    }

    pub fn set_instrument(&mut self, instrument: u8) {
        self.instrument = instrument;
    }

    pub fn next_instrument(&mut self) -> u8 {
        self.instrument = if self.instrument >= 127 { 0 } else { self.instrument + 1 };
        self.instrument
    }

    pub fn previous_instrument(&mut self) -> u8 {
        self.instrument = if self.instrument == 0 { 127 } else { self.instrument - 1 };
        self.instrument
    }

    pub fn get_volume(&self) -> u8 {
        self.volume
    }

    pub fn volume_up(&mut self, step: u8) -> u8 {
        self.volume = self.volume.saturating_add(step).min(127);
        self.volume
    }

    pub fn volume_down(&mut self, step: u8) -> u8 {
        self.volume = self.volume.saturating_sub(step);
        self.volume
    }

    pub fn get_direction(&self) -> Direction {
        self.direction
    }

    pub fn pan_cycle(&mut self, forward: bool) -> Direction {
        self.direction = if forward {
            self.direction.next()
        } else {
            self.direction.prev()
        };
        self.direction
    }

    pub fn get_start_note(&self) -> u8 {
        self.notes_manager.get_start_note()
    }

    pub fn octave_up(&mut self) -> u8 {
        self.notes_manager.octave_up()
    }

    pub fn octave_down(&mut self) -> u8 {
        self.notes_manager.octave_down()
    }

    pub fn semitone_up(&mut self, total: u8) -> u8 {
        self.notes_manager.semitone_up(total)
    }

    pub fn semitone_down(&mut self, total: u8) -> u8 {
        self.notes_manager.semitone_down(total)
    }
}
