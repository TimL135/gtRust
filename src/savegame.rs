use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct SaveData {
    pub highscore: i32,
    // weitere Felder kannst du später leicht ergänzen, z.B.:
    // pub unlocked_ships: Vec<String>,
    // pub sound_volume: f32,
    // pub settings: GameSettings,
}

impl Default for SaveData {
    fn default() -> Self {
        SaveData { highscore: 0 }
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
    if let Ok(json) = serde_json::to_string(data) {
        let _ = fs::write("savegame.json", json);
    }
}
