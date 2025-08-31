use macroquad::prelude::*;
use rand::gen_range;

pub struct Star {
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub speed: f32,
    pub base_brightness: f32,
}

impl Star {
    pub fn new() -> Self {
        Star {
            x: gen_range(0.0, screen_width()),
            y: gen_range(0.0, screen_height()),
            size: gen_range(0.5, 2.5),
            speed: gen_range(10.0, 60.0),
            base_brightness: gen_range(0.5, 1.0),
        }
    }

    pub fn update(&mut self) {
        self.y += self.speed * get_frame_time();

        // Wenn der Stern unten den Bildschirm verlässt, respawne ihn oben
        if self.y > screen_height() + self.size {
            self.x = gen_range(0.0, screen_width());
            self.y = -self.size;
        }
    }

    pub fn draw(&self, index: usize) {
        let time = get_time() as f32;

        // Twinkle-Effekt (Sternenflimmern)
        let twinkle = (time * (2.0 + (index as f32 % 5.0)) + self.x * 0.1).sin() * 0.3 + 0.7;
        let brightness = (self.base_brightness * twinkle).clamp(0.0, 1.0);

        draw_circle(
            self.x,
            self.y,
            self.size,
            Color::new(brightness, brightness, brightness, 0.9),
        );
    }
}
