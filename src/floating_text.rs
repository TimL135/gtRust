use macroquad::prelude::*;
pub struct FloatingText {
    pub x: f32,
    pub y: f32,
    pub text: String,
    pub lifetime: f32,
}

impl FloatingText {
    pub fn new(x: f32, y: f32, value: i32) -> Self {
        FloatingText {
            x,
            y,
            text: format!("+{}", value),
            lifetime: 2.0, // 1 Sekunde sichtbar
        }
    }

    pub fn update(&mut self) {
        self.y -= 20.0 * get_frame_time(); // ein bisschen nach oben schweben
        self.lifetime -= get_frame_time(); // Zeit runterzählen
    }

    pub fn is_dead(&self) -> bool {
        self.lifetime <= 0.0
    }

    pub fn draw(&self) {
        let alpha = (self.lifetime / 1.0).clamp(0.0, 1.0); // verblassen
        draw_text(
            &self.text,
            self.x,
            self.y,
            30.0,                             // Schriftgröße
            Color::new(1.0, 1.0, 0.0, alpha), // Gelb mit Alpha
        );
    }
}
