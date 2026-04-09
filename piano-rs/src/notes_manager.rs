use std::path::Path;
use std::fs;
use indexmap::IndexMap;
use crate::models::note::Note;

pub struct NotesManager {
    map: IndexMap<String, Note>,
    key_list: Vec<String>,
    start_note: u8,
}

impl NotesManager {
    pub fn new() -> Self {
        NotesManager {
            map: IndexMap::new(),
            key_list: Vec::new(),
            start_note: 48,
        }
    }

    pub fn load_file(&mut self, path: &Path) -> anyhow::Result<()> {
        self.start_note = 48;
        let content = fs::read_to_string(path)?;
        self.key_list = content
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty() && !l.starts_with('#'))
            .collect();
        self.organize_notes();
        Ok(())
    }

    fn organize_notes(&mut self) {
        self.map.clear();
        for (i, key) in self.key_list.iter().enumerate() {
            let normalized = key.to_uppercase();
            self.map.insert(normalized, Note::new(self.start_note + i as u8));
        }
    }

    pub fn get_note(&self, key: &str) -> Option<&Note> {
        self.map.get(key)
    }

    pub fn get_start_note(&self) -> u8 {
        self.start_note
    }

    pub fn octave_up(&mut self) -> u8 {
        self.semitone_up(12)
    }

    pub fn octave_down(&mut self) -> u8 {
        self.semitone_down(12)
    }

    pub fn semitone_up(&mut self, total: u8) -> u8 {
        let mut changed_count = 0usize;
        let mut success = true;

        for note in self.map.values_mut() {
            if note.semitone_up(total) {
                changed_count += 1;
            } else {
                success = false;
                break;
            }
        }

        if !success {
            for note in self.map.values_mut().take(changed_count) {
                note.semitone_down(total);
            }
        } else {
            self.start_note = self.start_note.saturating_add(total);
        }

        self.start_note
    }

    pub fn semitone_down(&mut self, total: u8) -> u8 {
        let mut changed_count = 0usize;
        let mut success = true;

        for note in self.map.values_mut() {
            if note.semitone_down(total) {
                changed_count += 1;
            } else {
                success = false;
                break;
            }
        }

        if !success {
            for note in self.map.values_mut().take(changed_count) {
                note.semitone_up(total);
            }
        } else {
            self.start_note = self.start_note.saturating_sub(total);
        }

        self.start_note
    }
}
