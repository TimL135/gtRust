use macroquad::audio::*;
use std::collections::HashMap;

pub struct MusicManager {
    current_music: Option<String>,
    tracks: HashMap<String, Sound>,
}

impl MusicManager {
    pub fn new() -> Self {
        Self {
            current_music: None,
            tracks: HashMap::new(),
        }
    }

    // Track laden und registrieren
    pub fn add_track(&mut self, name: &str, sound: Sound) {
        self.tracks.insert(name.to_string(), sound);
    }

    // Spielt einen Track ab - stoppt automatisch den vorherigen
    pub fn play(&mut self, name: &str, params: PlaySoundParams) {
        // Aktuellen Track stoppen, falls einer läuft
        if let Some(current_name) = &self.current_music {
            if let Some(current_sound) = self.tracks.get(current_name) {
                stop_sound(current_sound);
            }
        }

        // Neuen Track abspielen
        if let Some(sound) = self.tracks.get(name) {
            macroquad::audio::play_sound(sound, params);
            self.current_music = Some(name.to_string());
        }
    }

    // Aktuellen Track stoppen
    //pub fn stop(&mut self) {
    //  if let Some(current_name) = &self.current_music {
    //    if let Some(sound) = self.tracks.get(current_name) {
    //      stop_sound(sound);
    //}
    //    self.current_music = None;
    //}
    //}

    // Checken ob gerade ein Track läuft
    //pub fn is_playing(&self) -> bool {
    //    self.current_music.is_some()
    //}

    // Name des aktuell laufenden Tracks
    pub fn current_track(&self) -> Option<&String> {
        self.current_music.as_ref()
    }
}
