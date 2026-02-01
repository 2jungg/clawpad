use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone)]
pub struct Settings {
    pub font_size: f32,
    pub font_family: String,
    pub theme_dark: bool,
    pub transparency: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            font_size: 14.0,
            font_family: "monospace".to_string(),
            theme_dark: true,
            transparency: 0.9,
        }
    }
}

impl Settings {
    pub fn load() -> Self {
        let path = Self::path();
        if let Ok(data) = fs::read_to_string(&path) {
            if let Ok(settings) = serde_json::from_str(&data) {
                return settings;
            }
        }
        let default = Self::default();
        let _ = default.save();
        default
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let data = serde_json::to_string_pretty(self).unwrap();
        fs::write(Self::path(), data)
    }

    fn path() -> PathBuf {
        PathBuf::from("settings.json")
    }
}
