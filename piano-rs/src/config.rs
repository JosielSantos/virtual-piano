use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

pub struct Config {
    sections: HashMap<String, HashMap<String, String>>,
    config_dir: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            sections: HashMap::new(),
            config_dir: PathBuf::from("."),
        }
    }
}

impl Config {
    pub fn load(path: &Path) -> anyhow::Result<Self> {
        let content = fs::read_to_string(path)?;
        let config_dir = path.parent().unwrap_or(Path::new(".")).to_path_buf();
        let sections = parse_ini(&content);
        Ok(Config { sections, config_dir })
    }

    fn get(&self, section: &str, key: &str) -> Option<&str> {
        self.sections
            .get(section)
            .and_then(|s| s.get(key))
            .map(|v| v.as_str())
    }

    pub fn get_keymap_file_path(&self) -> PathBuf {
        let file_path = self.get("general", "keymap_file_path")
            .unwrap_or("pianoeletronico.kmp");

        let path = Path::new(file_path);
        if path.is_absolute() {
            path.to_path_buf()
        } else {
            self.config_dir.join("keymaps").join(file_path)
        }
    }

    pub fn get_midi_output_driver(&self) -> i32 {
        self.get("midi", "output_driver")
            .and_then(|v| v.trim().parse().ok())
            .unwrap_or(-1)
    }
}

fn parse_ini(content: &str) -> HashMap<String, HashMap<String, String>> {
    let mut sections: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut current_section = String::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with(';') || line.starts_with('#') {
            continue;
        }
        if line.starts_with('[') && line.ends_with(']') {
            current_section = line[1..line.len() - 1].trim().to_lowercase();
        } else if let Some(pos) = line.find('=') {
            let key = line[..pos].trim().to_lowercase();
            let value = line[pos + 1..].trim().to_string();
            sections
                .entry(current_section.clone())
                .or_default()
                .insert(key, value);
        }
    }

    sections
}

pub fn find_config_file() -> PathBuf {
    for candidate in &["config.ini", "../config.ini"] {
        let path = PathBuf::from(candidate);
        if path.exists() {
            return path;
        }
    }
    PathBuf::from("config.ini")
}
