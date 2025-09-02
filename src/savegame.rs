use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SaveData {
    pub highscore: i32,
    pub settings: GameSettings,
    // weitere Felder kannst du später leicht ergänzen
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameSettings {
    pub music_volume: f32,
    pub sound_volume: f32,
}

impl Default for SaveData {
    fn default() -> Self {
        SaveData {
            highscore: 0,
            settings: GameSettings {
                music_volume: 0.7,
                sound_volume: 0.8,
            },
        }
    }
}

pub fn load_save() -> SaveData {
    if let Ok(contents) = fs::read_to_string("savegame.json") {
        serde_json::from_str(&contents).unwrap_or_default()
    } else {
        SaveData::default()
    }
}

pub fn save_game(data: &SaveData) {
    if let Ok(json) = serde_json::to_string_pretty(data) {
        let _ = fs::write("savegame.json", json);
    }
}

// ---- Update Funktionen ----

// Highscore setzen
pub fn update_highscore(new_score: i32) {
    let mut data = load_save();
    data.highscore = new_score;
    save_game(&data);
}

// Settings komplett updaten
pub fn update_settings(new_settings: GameSettings) {
    let mut data = load_save();
    data.settings = new_settings;
    save_game(&data);
}
