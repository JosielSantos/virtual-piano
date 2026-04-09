use std::collections::HashMap;
use std::path::{Path, PathBuf};
use crate::channel::{Channel, Direction};
use crate::midi::MidiDriver;

const CC_PAN: u8 = 10;

pub struct Piano {
    channels: HashMap<u8, Channel>,
    keymap_path: PathBuf,
    midi_driver: Box<dyn MidiDriver>,
}

impl Piano {
    pub fn new(keymap_path: &Path, midi_driver: Box<dyn MidiDriver>) -> anyhow::Result<Self> {
        let mut piano = Piano {
            channels: HashMap::new(),
            keymap_path: keymap_path.to_path_buf(),
            midi_driver,
        };
        piano.ensure_channel(0);
        Ok(piano)
    }

    pub fn ensure_channel(&mut self, n: u8) {
        if !self.channels.contains_key(&n) {
            match Channel::new(&self.keymap_path) {
                Ok(channel) => {
                    self.channels.insert(n, channel);
                    self.midi_driver.set_instrument(0, n);
                }
                Err(e) => eprintln!("Erro ao criar canal {n}: {e}"),
            }
        }
    }

    pub fn note_on(&mut self, key: &str, channel_number: u8) {
        let result = self.channels.get_mut(&channel_number).and_then(|ch| {
            let note = ch.get_note(key)?;
            ch.notes_on.insert(note).then_some((note, ch.get_volume()))
        });
        if let Some((note, velocity)) = result {
            self.midi_driver.note_on(note, velocity, channel_number);
        }
    }

    pub fn note_off(&mut self, key: &str, channel_number: u8) {
        let result = self.channels.get_mut(&channel_number).and_then(|ch| {
            let note = ch.get_note(key)?;
            ch.notes_on.remove(&note).then_some(note)
        });
        if let Some(note) = result {
            self.midi_driver.note_off(note, channel_number);
        }
    }

    pub fn all_notes_off(&mut self) {
        let mut to_stop: Vec<(u8, u8)> = Vec::new();
        for (ch_num, channel) in self.channels.iter_mut() {
            for note in channel.notes_on.drain() {
                to_stop.push((*ch_num, note));
            }
        }
        for (ch_num, note) in to_stop {
            self.midi_driver.note_off(note, ch_num);
        }
    }

    pub fn delete_channel(&mut self, channel_number: u8) {
        if channel_number == 0 {
            return;
        }
        if let Some(channel) = self.channels.get_mut(&channel_number) {
            let notes: Vec<u8> = channel.notes_on.drain().collect();
            for note in notes {
                self.midi_driver.note_off(note, channel_number);
            }
        }
        self.midi_driver.set_instrument(0, channel_number);
        self.channels.remove(&channel_number);
    }

    pub fn get_start_note(&self, channel_number: u8) -> u8 {
        self.channels.get(&channel_number).map(|c| c.get_start_note()).unwrap_or(48)
    }

    pub fn get_instrument(&self, channel_number: u8) -> u8 {
        self.channels.get(&channel_number).map(|c| c.get_instrument()).unwrap_or(0)
    }

    pub fn set_instrument(&mut self, instrument: u8, channel_number: u8) {
        if let Some(channel) = self.channels.get_mut(&channel_number) {
            channel.set_instrument(instrument);
        }
        self.midi_driver.set_instrument(instrument, channel_number);
    }

    pub fn next_instrument(&mut self, channel_number: u8) -> u8 {
        let instrument = match self.channels.get_mut(&channel_number) {
            Some(ch) => ch.next_instrument(),
            None => return 0,
        };
        self.midi_driver.set_instrument(instrument, channel_number);
        instrument
    }

    pub fn previous_instrument(&mut self, channel_number: u8) -> u8 {
        let instrument = match self.channels.get_mut(&channel_number) {
            Some(ch) => ch.previous_instrument(),
            None => return 0,
        };
        self.midi_driver.set_instrument(instrument, channel_number);
        instrument
    }

    pub fn get_volume(&self, channel_number: u8) -> u8 {
        self.channels.get(&channel_number).map(|c| c.get_volume()).unwrap_or(127)
    }

    pub fn volume_up(&mut self, channel_number: u8) -> u8 {
        self.channels.get_mut(&channel_number).map(|c| c.volume_up(10)).unwrap_or(127)
    }

    pub fn volume_down(&mut self, channel_number: u8) -> u8 {
        self.channels.get_mut(&channel_number).map(|c| c.volume_down(10)).unwrap_or(0)
    }

    pub fn get_direction(&self, channel_number: u8) -> Direction {
        self.channels.get(&channel_number).map(|c| c.get_direction()).unwrap_or(Direction::Center)
    }

    pub fn pan(&mut self, forward: bool, channel_number: u8) {
        let dir = match self.channels.get_mut(&channel_number) {
            Some(ch) => ch.pan_cycle(forward),
            None => return,
        };
        self.midi_driver.control_change(CC_PAN, dir.to_midi_pan(), channel_number);
    }

    pub fn octave_up(&mut self, channel_number: u8) -> u8 {
        self.all_notes_off();
        self.channels.get_mut(&channel_number).map(|c| c.octave_up()).unwrap_or(48)
    }

    pub fn octave_down(&mut self, channel_number: u8) -> u8 {
        self.all_notes_off();
        self.channels.get_mut(&channel_number).map(|c| c.octave_down()).unwrap_or(48)
    }

    pub fn semitone_up(&mut self, total: u8, channel_number: u8) -> u8 {
        self.all_notes_off();
        self.channels.get_mut(&channel_number).map(|c| c.semitone_up(total)).unwrap_or(48)
    }

    pub fn semitone_down(&mut self, total: u8, channel_number: u8) -> u8 {
        self.all_notes_off();
        self.channels.get_mut(&channel_number).map(|c| c.semitone_down(total)).unwrap_or(48)
    }
}
