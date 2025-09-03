use crate::savegame::load_save;
use macroquad::audio::*;
use std::collections::HashMap;

pub struct MusicManager {
    current_music: Option<String>,
    tracks: HashMap<String, Sound>,
    music_volume: f32,
}

impl MusicManager {
    pub async fn new() -> Self {
        let save_data = load_save();
        let menu_music = load_sound("assets/music/menu_track.ogg").await.unwrap();
        let gameplay_music = load_sound("assets/music/gameplay_track.ogg").await.unwrap();

        Self {
            current_music: None,
            tracks: HashMap::from([
                ("menu".to_string(), menu_music),
                ("gameplay".to_string(), gameplay_music),
            ]),
            music_volume: save_data.settings.music_volume,
        }
    }

    // Spielt einen Track ab - stoppt automatisch den vorherigen
    pub fn play(&mut self, name: &str) {
        // Aktuellen Track stoppen
        if let Some(current_name) = &self.current_music {
            if let Some(current_sound) = self.tracks.get(current_name) {
                stop_sound(current_sound);
            }
        }

        // Neuen Track abspielen
        if let Some(sound) = self.tracks.get(name) {
            let params = PlaySoundParams {
                looped: true,
                volume: self.music_volume,
            };
            play_sound(sound, params);
            self.current_music = Some(name.to_string());
        }
    }

    // Settings neu laden (z.B. wenn User im Modal die Lautstärke geändert hat)
    pub fn refresh_settings(&mut self) {
        let save_data = load_save();
        let new_volume = save_data.settings.music_volume;

        if (new_volume - self.music_volume).abs() > 0.01 {
            self.music_volume = new_volume;

            // Laufende Musik sofort anpassen
            if let Some(current_name) = &self.current_music {
                if let Some(sound) = self.tracks.get(current_name) {
                    set_sound_volume(sound, new_volume);
                }
            }
        }
    }

    // Aktuellen Track stoppen
    // pub fn stop(&mut self) {
    //     if let Some(current_name) = &self.current_music {
    //         if let Some(sound) = self.tracks.get(current_name) {
    //             stop_sound(sound);
    //         }
    //         self.current_music = None;
    //     }
    // }

    // Name des aktuellen Tracks
    pub fn current_track(&self) -> Option<&String> {
        self.current_music.as_ref()
    }
}
