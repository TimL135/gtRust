use macroquad::prelude::*;

pub struct FpsCounter {
    frame_times: Vec<f32>,
    last_update: f64,
    current_fps: f32,
}

impl FpsCounter {
    pub fn new() -> Self {
        Self {
            frame_times: Vec::with_capacity(60),
            last_update: get_time(),
            current_fps: 0.0,
        }
    }

    pub fn update(&mut self) {
        let current_time = get_time();
        let delta_time = (current_time - self.last_update) as f32;
        self.last_update = current_time;

        self.frame_times.push(delta_time);

        if self.frame_times.len() > 60 {
            self.frame_times.remove(0);
        }

        if !self.frame_times.is_empty() {
            let avg_frame_time: f32 =
                self.frame_times.iter().sum::<f32>() / self.frame_times.len() as f32;
            self.current_fps = if avg_frame_time > 0.0 {
                1.0 / avg_frame_time
            } else {
                0.0
            };
        }
    }

    pub fn draw(&self) {
        let text = format!("FPS: {:.0}", self.current_fps);
        let font_size = 20.0;

        // Textbreite berechnen
        let text_dim = measure_text(&text, None, font_size as u16, 1.0);

        // Rechts oben, mit kleinem Rand
        let x = screen_width() - text_dim.width - 10.0;
        let y = 20.0 + font_size / 2.5;

        draw_text(&text, x, y, font_size, WHITE);
    }
}
