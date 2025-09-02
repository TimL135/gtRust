use crate::savegame::load_save;
use macroquad::audio::*;
use std::collections::HashMap;

pub struct MusicManager {
    current_music: Option<String>,
    tracks: HashMap<String, Sound>,
    music_volume: f32,
}

impl MusicManager {
    pub fn new() -> Self {
        let save_data = load_save();
        Self {
            current_music: None,
            tracks: HashMap::new(),
            music_volume: save_data.settings.music_volume,
        }
    }

    // Track laden und registrieren
    pub fn add_track(&mut self, name: &str, sound: Sound) {
        self.tracks.insert(name.to_string(), sound);
    }

    // Spielt einen Track ab - stoppt automatisch den vorherigen
    pub fn play(&mut self, name: &str) {
        // Settings neu laden für aktuelle Lautstärke
        self.refresh_settings();

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
