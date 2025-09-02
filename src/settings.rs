use crate::savegame::{GameSettings, load_save, update_settings};
use macroquad::prelude::*;

pub struct SettingsUI {
    settings: GameSettings, // persistierte/aktive Settings
    modal: Modal,
    pub have_volume_changes: bool,
}

impl SettingsUI {
    pub fn new() -> Self {
        let save_data = load_save();
        Self {
            settings: save_data.settings.clone(),
            modal: Modal::new(),
            have_volume_changes: false,
        }
    }

    pub fn update_and_draw(&mut self) {
        if !self.modal.is_open {
            self.draw_main_ui();
            self.handle_button_click();
        }

        if let Some(action) = self.modal.draw() {
            match action {
                ModalAction::Save(new_settings) => {
                    // Übernehmen + speichern
                    self.settings = new_settings.clone();
                    update_settings(self.settings.clone());
                    self.modal.close();
                    self.have_volume_changes = true;
                }
                ModalAction::Cancel => {
                    // Verwerfen
                    self.modal.close();
                }
            }
        }

        if self.modal.is_open && is_key_pressed(KeyCode::Escape) {
            // ESC = Cancel
            self.modal.close();
        }
    }

    fn draw_main_ui(&self) {
        draw_text("Macroquad Audio Settings Demo", 20.0, 30.0, 24.0, BLACK);

        // Button top-right, responsive
        let button_w = (screen_width() * 0.15).clamp(120.0, 260.0);
        let button_h = (screen_height() * 0.08).clamp(40.0, 72.0);
        let button_x = screen_width() - button_w - 20.0;
        let button_y = 20.0;
        let button_rect = Rect::new(button_x, button_y, button_w, button_h);

        let mouse_pos = mouse_position();
        let hovered = button_rect.contains(Vec2::new(mouse_pos.0, mouse_pos.1));
        draw_button(button_rect, "Settings", hovered);

        // Current values bottom-left
        draw_text(
            &format!("Music: {:.0}%", self.settings.music_volume * 100.0),
            20.0,
            screen_height() - 60.0,
            20.0,
            BLACK,
        );
        draw_text(
            &format!("Sound: {:.0}%", self.settings.sound_volume * 100.0),
            20.0,
            screen_height() - 30.0,
            20.0,
            BLACK,
        );
    }

    fn handle_button_click(&mut self) {
        let button_w = (screen_width() * 0.15).clamp(120.0, 260.0);
        let button_h = (screen_height() * 0.08).clamp(40.0, 72.0);
        let button_x = screen_width() - button_w - 20.0;
        let button_y = 20.0;
        let button_rect = Rect::new(button_x, button_y, button_w, button_h);

        let mouse_pos = mouse_position();
        let hovered = button_rect.contains(Vec2::new(mouse_pos.0, mouse_pos.1));
        if hovered && is_mouse_button_pressed(MouseButton::Left) {
            // Modal mit aktuellem Stand öffnen (Working Copy)
            self.modal.open_with(self.settings.clone());
        }
    }
}

// Ergebnis-Aktion des Modals
enum ModalAction {
    Save(GameSettings),
    Cancel,
}

struct Modal {
    is_open: bool,
    working: GameSettings, // temporäre Werte während das Modal offen ist
}

impl Modal {
    fn new() -> Self {
        Self {
            is_open: false,
            working: GameSettings {
                music_volume: 0.7,
                sound_volume: 0.8,
            },
        }
    }

    fn open_with(&mut self, current: GameSettings) {
        self.is_open = true;
        self.working = current.clone(); // Working Copy starten
    }

    fn close(&mut self) {
        self.is_open = false;
    }

    // Zeichnet das Modal und gibt optional eine Aktion zurück (Save/Cancel)
    fn draw(&mut self) -> Option<ModalAction> {
        if !self.is_open {
            return None;
        }

        // Maße/Position
        let modal_w = (screen_width() * 0.5).clamp(420.0, 900.0);
        let modal_h = (screen_height() * 0.5).clamp(320.0, 720.0);
        let modal_x = (screen_width() - modal_w) / 2.0;
        let modal_y = (screen_height() - modal_h) / 2.0;
        let modal_rect = Rect::new(modal_x, modal_y, modal_w, modal_h);

        // Overlay
        draw_rectangle(
            0.0,
            0.0,
            screen_width(),
            screen_height(),
            Color::new(0.0, 0.0, 0.0, 0.5),
        );

        // Background
        draw_rectangle(
            modal_rect.x,
            modal_rect.y,
            modal_rect.w,
            modal_rect.h,
            WHITE,
        );
        draw_rectangle_lines(
            modal_rect.x,
            modal_rect.y,
            modal_rect.w,
            modal_rect.h,
            2.0,
            BLACK,
        );

        // Title
        draw_text(
            "Audio Settings",
            modal_rect.x + 24.0,
            modal_rect.y + 48.0,
            28.0,
            BLACK,
        );

        // Slider Layout
        let left_pad = 40.0;
        let right_pad = 40.0;
        let slider_w = modal_rect.w - left_pad - right_pad - 80.0; // Platz für Prozent-Text
        let slider_h = 22.0;
        let first_y = modal_rect.y + 110.0;
        let gap = 80.0;

        // Music Slider
        let music_rect = Rect::new(modal_rect.x + left_pad, first_y, slider_w, slider_h);
        self.working.music_volume =
            draw_slider(music_rect, self.working.music_volume, "Music Volume:");

        // Sound Slider
        let sound_rect = Rect::new(modal_rect.x + left_pad, first_y + gap, slider_w, slider_h);
        self.working.sound_volume =
            draw_slider(sound_rect, self.working.sound_volume, "Sound Volume:");

        // Buttons unten rechts: Cancel und Save
        let btn_h = 44.0;
        let btn_w = (modal_rect.w * 0.22).clamp(120.0, 220.0);
        let spacing = 16.0;
        let btn_y = modal_rect.y + modal_rect.h - btn_h - 28.0;

        let save_rect = Rect::new(
            modal_rect.x + modal_rect.w - btn_w - 24.0,
            btn_y,
            btn_w,
            btn_h,
        );
        let cancel_rect = Rect::new(save_rect.x - spacing - btn_w, btn_y, btn_w, btn_h);

        let mouse = mouse_position();
        let save_hover = save_rect.contains(Vec2::new(mouse.0, mouse.1));
        let cancel_hover = cancel_rect.contains(Vec2::new(mouse.0, mouse.1));

        // Cancel (links), Save (rechts)
        draw_button(cancel_rect, "Cancel", cancel_hover);
        draw_button(save_rect, "Save", save_hover);

        if is_mouse_button_pressed(MouseButton::Left) {
            if save_hover {
                return Some(ModalAction::Save(self.working.clone()));
            }
            if cancel_hover {
                return Some(ModalAction::Cancel);
            }
        }

        None
    }
}

// --- Helpers ---
fn draw_button(rect: Rect, text: &str, hovered: bool) {
    let base = if hovered {
        Color::new(0.85, 0.85, 0.85, 1.0)
    } else {
        LIGHTGRAY
    };
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, base);
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, BLACK);

    let text_size = (rect.h * 0.5).min(26.0).max(16.0);
    let text_dims = measure_text(text, None, text_size as u16, 1.0);
    draw_text(
        text,
        rect.x + (rect.w - text_dims.width) / 2.0,
        rect.y + (rect.h + text_dims.height) / 2.0,
        text_size,
        BLACK,
    );
}

fn draw_slider(rect: Rect, value: f32, label: &str) -> f32 {
    let mut new_value = value;

    // Label
    draw_text(label, rect.x, rect.y - 12.0, 18.0, BLACK);

    // Track
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, DARKGRAY);
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1.0, BLACK);

    // Handle
    let handle_w = 20.0;
    let handle_x = rect.x + (rect.w - handle_w) * new_value;
    let handle_rect = Rect::new(handle_x, rect.y - 6.0, handle_w, rect.h + 12.0);
    draw_rectangle(
        handle_rect.x,
        handle_rect.y,
        handle_rect.w,
        handle_rect.h,
        WHITE,
    );
    draw_rectangle_lines(
        handle_rect.x,
        handle_rect.y,
        handle_rect.w,
        handle_rect.h,
        1.0,
        BLACK,
    );

    // Interaction
    if is_mouse_button_down(MouseButton::Left) {
        let (mx, my) = mouse_position();
        if my >= rect.y - 12.0
            && my <= rect.y + rect.h + 12.0
            && mx >= rect.x
            && mx <= rect.x + rect.w
        {
            new_value = (((mx - rect.x) / rect.w).clamp(0.0, 1.0) * 100.0).round() / 100.0;
        }
    }

    // Percentage text
    draw_text(
        &format!("{:.0}%", new_value * 100.0),
        rect.x + rect.w + 12.0,
        rect.y + rect.h - 2.0,
        18.0,
        BLACK,
    );

    new_value
}
