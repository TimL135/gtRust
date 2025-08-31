use crate::debris::Debris;
use macroquad::prelude::*;

pub struct Bullet {
    pub x: f32,
    pub y: f32,
    pub velocity_x: f32, // NEU: Geschwindigkeit in X-Richtung
    pub velocity_y: f32, // NEU: Geschwindigkeit in Y-Richtung
    pub size: f32,
    pub damage: f32,
}

impl Bullet {
    pub fn new(x: f32, y: f32, angle: f32) -> Self {
        let speed = screen_height() * 1.0;
        // Geschwindigkeitsvektor basierend auf Winkel berechnen
        // angle - PI/2 weil das Schiff standardmäßig nach oben zeigt
        let shoot_angle = angle - std::f32::consts::FRAC_PI_2;
        Bullet {
            x,
            y,
            velocity_x: shoot_angle.cos() * speed,
            velocity_y: shoot_angle.sin() * speed,
            size: screen_width().min(screen_height()) * 0.01,
            damage: 2.5,
        }
    }

    pub fn update(&mut self) {
        self.x += self.velocity_x * get_frame_time();
        self.y += self.velocity_y * get_frame_time();
    }

    pub fn draw(&self) {
        let time = get_time() as f32;

        // Plasma-Pulsieren (sanft)
        let pulse = 1.0 + 0.15 * (time * 8.0 + self.x * 0.1 + self.y * 0.1).sin();

        // Äußerer Glow (blau-weiß, halbtransparent)
        draw_circle(
            self.x,
            self.y,
            self.size * 2.2 * pulse,
            Color::new(0.3, 0.7, 1.0, 0.2),
        );

        // Mittlerer Glow (heller blau)
        draw_circle(
            self.x,
            self.y,
            self.size * 1.6 * pulse,
            Color::new(0.5, 0.9, 1.0, 0.4),
        );

        // Innerer Kern (weiß-gelb, sehr hell)
        draw_circle(
            self.x,
            self.y,
            self.size * 1.1,
            Color::new(1.0, 1.0, 0.9, 0.9),
        );

        // Zentraler Kern (strahlend weiß)
        draw_circle(self.x, self.y, self.size * 0.6, WHITE);

        // Kleine Plasma-Funken um die Kugel
        for i in 0..4 {
            let spark_time = time * 12.0 + i as f32 * 1.57; // 90° versetzt
            let spark_distance = self.size * 1.8;
            let spark_x = self.x + spark_time.cos() * spark_distance;
            let spark_y = self.y + spark_time.sin() * spark_distance;
            let spark_size = self.size * 0.3 * (1.0 + 0.5 * (spark_time * 2.0).sin());

            draw_circle(spark_x, spark_y, spark_size, Color::new(0.8, 0.9, 1.0, 0.6));
        }
    }

    pub fn is_off_screen(&self) -> bool {
        self.x < -self.size
            || self.x > screen_width() + self.size
            || self.y < -self.size
            || self.y > screen_height() + self.size
    }

    pub fn collides_with(&self, debris: &Debris) -> bool {
        let dx = self.x - debris.x;
        let dy = self.y - debris.y;
        let distance = (dx * dx + dy * dy).sqrt();
        distance < self.size + debris.size
    }

    pub fn handle_collisions(bullets: &mut Vec<Bullet>, debris: &mut Vec<Debris>) {
        bullets.retain(|bullet| {
            let mut bullet_destroyed = false;

            for debris_piece in debris.iter_mut() {
                if bullet.collides_with(debris_piece) {
                    debris_piece.take_damage(bullet.damage);
                    bullet_destroyed = true;
                    break; // ein Bullet zerstört nur einen Gegner
                }
            }

            !bullet_destroyed // behalte Bullet nur wenn es nicht zerstört wurde
        });
    }
}
