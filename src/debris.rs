use macroquad::prelude::*;
use rand::gen_range;

use crate::explosion::Explosion;
use crate::floating_text::FloatingText;
use crate::help_fn::lerp;

pub struct Debris {
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub hp: f32,
    pub max_hp: f32,
    pub displayed_hp_progress: f32,
    pub damage: f32,
    pub rotation: f32,
    pub rotation_speed: f32,
    pub debris_type: u8,
    pub color_variant: f32,
}

impl Debris {
    pub fn new() -> Self {
        let screen_w = screen_width();
        let screen_h = screen_height();
        let screen_size = screen_w.min(screen_h);

        let size = gen_range(screen_size * 0.02, screen_size * 0.06);
        let max_hp = ((size / (screen_size * 0.01)) as f32).max(1.0);

        // zufällige Seite wählen: 0=oben, 1=unten, 2=links, 3=rechts
        let side = gen_range(0, 4);

        // Startposition & Bewegungsrichtung festlegen
        let (x, y, dir_x, dir_y) = match side {
            // oben -> nach unten ins Spielfeld
            0 => (gen_range(0.0, screen_w), -size, 0.0, 1.0),
            // unten -> nach oben
            1 => (gen_range(0.0, screen_w), screen_h + size, 0.0, -1.0),
            // links -> nach rechts
            2 => (-size, gen_range(0.0, screen_h), 1.0, 0.0),
            // rechts -> nach links
            _ => (screen_w + size, gen_range(0.0, screen_h), -1.0, 0.0),
        };

        // kleine Variation in Richtung (damit es nicht perfekt gerade fliegt)
        let angle_offset = gen_range(-0.3f32, 0.3f32); // ± ca. 17°
        let base_angle: f32 = (dir_y as f32).atan2(dir_x as f32); // Richtung des Vektors
        let angle = base_angle + angle_offset;
        let speed = gen_range(screen_size * 0.2, screen_size * 0.6);
        let velocity_x = angle.cos() * speed;
        let velocity_y = angle.sin() * speed;

        Debris {
            x,
            y,
            size,
            velocity_x,
            velocity_y,
            hp: max_hp,
            max_hp,
            displayed_hp_progress: 1.0,
            damage: 1.0,
            rotation: gen_range(0.0, std::f32::consts::PI * 2.0),
            rotation_speed: gen_range(-2.0, 2.0),
            debris_type: gen_range(0, 4),
            color_variant: gen_range(0.0, 1.0),
        }
    }

    pub fn update(
        &mut self,
        explosions: &mut Vec<Explosion>,
        floating_texts: &mut Vec<FloatingText>,
        score: &mut i32,
    ) -> bool {
        // Bewegung
        self.x += self.velocity_x * get_frame_time();
        self.y += self.velocity_y * get_frame_time();

        // Rotation
        self.rotation += self.rotation_speed * get_frame_time();

        // HP-Balken animieren
        let target_progress = (self.hp.max(0.0) as f32) / (self.max_hp as f32);
        let lerp_speed = 8.0;
        self.displayed_hp_progress = lerp(
            self.displayed_hp_progress,
            target_progress,
            get_frame_time() * lerp_speed,
        );

        // Prüfen, ob zerstört → wenn ja: Explosion + Score + FloatingText
        if self.is_destroyed() {
            *score += 50;
            floating_texts.push(FloatingText::new(self.x, self.y, 50));
            explosions.push(Explosion::new(
                self.x,
                self.y,
                self.size,
                self.velocity_x,
                self.velocity_y,
                self.debris_type,
            ));
            return true; // signalisiert, dass dieses Debris entfernt werden soll
        }

        false
    }

    pub fn draw(&self) {
        let time = get_time() as f32;

        // Basis-Farben für Weltraum-Debris (metallisch/grau)
        let base_gray = 0.4 + self.color_variant * 0.3;
        let rust_factor = self.color_variant * 0.2;

        let main_color = Color::new(
            base_gray + rust_factor,
            base_gray,
            base_gray - rust_factor * 0.5,
            1.0,
        );

        let shadow_color = Color::new(base_gray * 0.5, base_gray * 0.5, base_gray * 0.4, 1.0);

        // Verschiedene Debris-Formen basierend auf debris_type
        match self.debris_type {
            0 => self.draw_angular_debris(main_color, shadow_color),
            1 => self.draw_rectangular_debris(main_color, shadow_color),
            2 => self.draw_irregular_debris(main_color, shadow_color),
            _ => self.draw_complex_debris(main_color, shadow_color),
        }

        // Leichtes Glitzern/Funkeln für metallische Oberfläche
        if (time * 3.0 + self.x * 0.01 + self.y * 0.01).sin() > 0.8 {
            let sparkle_offset_x = (time * 5.0 + self.rotation).cos() * self.size * 0.3;
            let sparkle_offset_y = (time * 5.0 + self.rotation).sin() * self.size * 0.3;
            draw_circle(
                self.x + sparkle_offset_x,
                self.y + sparkle_offset_y,
                self.size * 0.05,
                WHITE,
            );
        }

        // HP-Balken (jetzt mit Animation)
        if self.hp < self.max_hp || self.displayed_hp_progress < 1.0 {
            let bar_w = self.size * 2.0;
            let bar_h = self.size * 0.15;
            let bar_x = self.x - bar_w / 2.0;
            let bar_y = self.y - self.size * 1.4;

            // Hintergrund (dunkelrot für verlorene HP)
            draw_rectangle(bar_x, bar_y, bar_w, bar_h, Color::new(0.3, 0.1, 0.1, 0.8));
            // Vordergrund (grün für aktuelle HP, animiert)
            draw_rectangle(
                bar_x,
                bar_y,
                bar_w * self.displayed_hp_progress,
                bar_h,
                Color::new(0.2, 0.8, 0.3, 0.9),
            );

            // Rahmen um den Balken
            draw_rectangle_lines(
                bar_x,
                bar_y,
                bar_w,
                bar_h,
                1.0,
                Color::new(0.8, 0.8, 0.8, 0.6),
            );
        }
    }

    fn draw_angular_debris(&self, main_color: Color, shadow_color: Color) {
        // Eckiges Trümmerstück
        let points = [
            Vec2::new(-0.8, -0.6),
            Vec2::new(0.9, -0.4),
            Vec2::new(0.7, 0.8),
            Vec2::new(-0.5, 0.9),
            Vec2::new(-0.9, 0.2),
        ];

        self.draw_rotated_polygon(&points, main_color, shadow_color);
    }

    fn draw_rectangular_debris(&self, main_color: Color, shadow_color: Color) {
        // Rechteckiges Trümmerstück mit Einbuchtungen
        let points = [
            Vec2::new(-0.9, -0.7),
            Vec2::new(0.9, -0.7),
            Vec2::new(0.9, -0.2),
            Vec2::new(0.4, -0.2),
            Vec2::new(0.4, 0.7),
            Vec2::new(-0.6, 0.7),
            Vec2::new(-0.6, 0.2),
            Vec2::new(-0.9, 0.2),
        ];

        self.draw_rotated_polygon(&points, main_color, shadow_color);
    }

    fn draw_irregular_debris(&self, main_color: Color, shadow_color: Color) {
        // Unregelmäßiges Trümmerstück
        let points = [
            Vec2::new(-0.7, -0.9),
            Vec2::new(0.3, -0.8),
            Vec2::new(0.9, -0.3),
            Vec2::new(0.8, 0.4),
            Vec2::new(0.2, 0.9),
            Vec2::new(-0.4, 0.7),
            Vec2::new(-0.8, 0.1),
            Vec2::new(-0.9, -0.4),
        ];

        self.draw_rotated_polygon(&points, main_color, shadow_color);
    }

    fn draw_complex_debris(&self, main_color: Color, shadow_color: Color) {
        // Komplexes Trümmerstück mit Löchern
        let outer_points = [
            Vec2::new(-0.9, -0.8),
            Vec2::new(0.8, -0.9),
            Vec2::new(0.9, 0.7),
            Vec2::new(-0.7, 0.9),
        ];

        self.draw_rotated_polygon(&outer_points, main_color, shadow_color);

        // Loch in der Mitte
        let hole_points = [
            Vec2::new(-0.3, -0.3),
            Vec2::new(0.3, -0.3),
            Vec2::new(0.3, 0.3),
            Vec2::new(-0.3, 0.3),
        ];

        self.draw_rotated_polygon(&hole_points, Color::new(0.0, 0.0, 0.0, 0.0), shadow_color);
    }

    fn draw_rotated_polygon(&self, points: &[Vec2], main_color: Color, shadow_color: Color) {
        let cos_r = self.rotation.cos();
        let sin_r = self.rotation.sin();

        // Transformiere und zeichne das Polygon
        let mut transformed_points = Vec::new();
        for point in points {
            let rotated_x = point.x * cos_r - point.y * sin_r;
            let rotated_y = point.x * sin_r + point.y * cos_r;

            transformed_points.push(Vec2::new(
                self.x + rotated_x * self.size,
                self.y + rotated_y * self.size,
            ));
        }

        // Zeichne das Hauptpolygon
        if transformed_points.len() >= 3 {
            for i in 1..transformed_points.len() - 1 {
                draw_triangle(
                    transformed_points[0],
                    transformed_points[i],
                    transformed_points[i + 1],
                    main_color,
                );
            }

            // Zeichne Kanten für 3D-Effekt
            for i in 0..transformed_points.len() {
                let next_i = (i + 1) % transformed_points.len();
                draw_line(
                    transformed_points[i].x,
                    transformed_points[i].y,
                    transformed_points[next_i].x,
                    transformed_points[next_i].y,
                    2.0,
                    shadow_color,
                );
            }
        }
    }

    pub fn is_off_screen(&self) -> bool {
        let margin = self.size * 2.0;
        self.x < -margin
            || self.x > screen_width() + margin
            || self.y < -margin
            || self.y > screen_height() + margin
    }

    pub fn collides_with(&self, player: &crate::player::Player) -> bool {
        let dx = self.x - player.x;
        let dy = self.y - player.y;
        (dx * dx + dy * dy).sqrt() < self.size + player.size
    }

    pub fn take_damage(&mut self, damage: f32) -> bool {
        self.hp -= damage;
        self.hp <= 0.0
    }

    pub fn is_destroyed(&self) -> bool {
        self.hp <= 0.0
    }
}
