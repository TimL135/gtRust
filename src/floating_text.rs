use macroquad::prelude::*;

pub struct FloatingText {
    pub x: f32,
    pub y: f32,
    pub text: String,
    pub color: Color,
    pub velocity_y: f32,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub size: f32,
}

impl FloatingText {
    pub fn new(x: f32, y: f32, points: i32) -> Self {
        Self {
            x,
            y,
            text: format!("+{}", points),
            color: Color::new(1.0, 1.0, 0.0, 1.0), // Gelb
            velocity_y: -50.0,                     // Nach oben schweben
            lifetime: 2.0,
            max_lifetime: 2.0,
            size: 20.0,
        }
    }

    pub fn new_with_text(x: f32, y: f32, text: String, color: Color) -> Self {
        Self {
            x,
            y,
            text,
            color,
            velocity_y: -30.0, // Nach oben schweben
            lifetime: 2.5,
            max_lifetime: 2.5,
            size: 18.0,
        }
    }

    pub fn update(&mut self) {
        self.y += self.velocity_y * get_frame_time();
        self.lifetime -= get_frame_time();

        // Verblassen über Zeit
        let alpha = (self.lifetime / self.max_lifetime).clamp(0.0, 1.0);
        self.color.a = alpha;
    }

    pub fn draw(&self) {
        draw_text(&self.text, self.x, self.y, self.size, self.color);
    }

    pub fn is_dead(&self) -> bool {
        self.lifetime <= 0.0
    }
}
