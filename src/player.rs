use macroquad::prelude::*;

use crate::bullet::Bullet;
use crate::help_fn::lerp;

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub shoot_cooldown: f32,
    pub max_shoot_ccooldown: f32,
    pub rotation: f32,
    pub displayed_hp_progress: f32,
    pub hp: f32,
    pub max_hp: f32,
}

impl Player {
    pub fn new() -> Self {
        let max_hp = 3.0;
        Player {
            x: screen_width() / 2.0,
            y: screen_height() - screen_height() * 0.1,
            size: screen_width().min(screen_height()) * 0.03,
            shoot_cooldown: 0.0,
            max_shoot_ccooldown: 0.5,
            rotation: 0.0,
            displayed_hp_progress: 1.0,
            hp: max_hp,
            max_hp,
        }
    }

    pub fn update(&mut self, bullets: &mut Vec<Bullet>) {
        // Schießen mit Space (mit Rotation)
        if is_key_down(KeyCode::Space) && self.shoot_cooldown <= 0.0 {
            // Startposition vorne am Schiff (rotiert)
            let front_offset = Vec2::new(0.0, -self.size);
            let cos_a = self.rotation.cos();
            let sin_a = self.rotation.sin();
            let rotated_offset = Vec2::new(
                front_offset.x * cos_a - front_offset.y * sin_a,
                front_offset.x * sin_a + front_offset.y * cos_a,
            );

            let bullet_x = self.x + rotated_offset.x;
            let bullet_y = self.y + rotated_offset.y;

            bullets.push(Bullet::new(bullet_x, bullet_y, self.rotation));
            self.shoot_cooldown = self.max_shoot_ccooldown;
        }
        let speed = screen_width().max(screen_height()) * 0.25;

        // Bewegung mit Pfeiltasten oder WASD
        let mut dx = 0.0f32;
        let mut dy = 0.0f32;

        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            dx -= 1.0;
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            dx += 1.0;
        }
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            dy -= 1.0;
        }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            dy += 1.0;
        }

        // Normieren um diagonale Bewegung nicht schneller zu machen
        if dx != 0.0 || dy != 0.0 {
            let len = (dx * dx + dy * dy).sqrt();
            dx /= len;
            dy /= len;
        }

        // Geschwindigkeit anwenden
        self.x += dx * speed * get_frame_time();
        self.y += dy * speed * get_frame_time();

        // Neuen Winkel speichern (arctangent)
        if dx != 0.0 || dy != 0.0 {
            self.rotation = dy.atan2(dx) + std::f32::consts::FRAC_PI_2;
        }

        // Bildschirmgrenzen
        self.x = self.x.clamp(self.size, screen_width() - self.size);
        self.y = self.y.clamp(self.size, screen_height() - self.size);

        // Cooldown reduzieren
        if self.shoot_cooldown > 0.0 {
            self.shoot_cooldown -= get_frame_time();
        }

        // NEU: Animation des HP-Balkens
        let target_progress = (self.hp.max(0.0) as f32) / (self.max_hp as f32);
        let lerp_speed = 8.0; // Wie schnell der Balken dem Ziel folgt (höher = schneller)
        self.displayed_hp_progress = lerp(
            self.displayed_hp_progress,
            target_progress,
            get_frame_time() * lerp_speed,
        );
    }

    pub fn draw(&self) {
        let angle = self.rotation;

        // Rotationsfunktion
        let rotate = |p: Vec2, angle: f32| -> Vec2 {
            let cos_a = angle.cos();
            let sin_a = angle.sin();
            Vec2::new(p.x * cos_a - p.y * sin_a, p.x * sin_a + p.y * cos_a)
        };

        // === HAUPTKÖRPER (Rumpf) ===
        let body_points = [
            Vec2::new(0.0, -self.size),                   // Spitze vorne
            Vec2::new(-self.size * 0.3, self.size * 0.2), // links mitte
            Vec2::new(-self.size * 0.2, self.size * 0.6), // links hinten
            Vec2::new(self.size * 0.2, self.size * 0.6),  // rechts hinten
            Vec2::new(self.size * 0.3, self.size * 0.2),  // rechts mitte
        ];

        // Rumpf zeichnen (mehrere Dreiecke für Form)
        let rp1 = rotate(body_points[0], angle) + Vec2::new(self.x, self.y);
        let rp2 = rotate(body_points[1], angle) + Vec2::new(self.x, self.y);
        let rp3 = rotate(body_points[4], angle) + Vec2::new(self.x, self.y);
        let rp4 = rotate(body_points[2], angle) + Vec2::new(self.x, self.y);
        let rp5 = rotate(body_points[3], angle) + Vec2::new(self.x, self.y);

        // Hauptrumpf (dunkelblau)
        draw_triangle(rp1, rp2, rp3, Color::new(0.2, 0.4, 0.8, 1.0));
        draw_triangle(rp2, rp4, rp5, Color::new(0.2, 0.4, 0.8, 1.0));
        draw_triangle(rp2, rp5, rp3, Color::new(0.2, 0.4, 0.8, 1.0));

        // === COCKPIT (heller Bereich vorne) ===
        let cockpit_points = [
            Vec2::new(0.0, -self.size * 0.8),
            Vec2::new(-self.size * 0.15, -self.size * 0.2),
            Vec2::new(self.size * 0.15, -self.size * 0.2),
        ];
        let cp1 = rotate(cockpit_points[0], angle) + Vec2::new(self.x, self.y);
        let cp2 = rotate(cockpit_points[1], angle) + Vec2::new(self.x, self.y);
        let cp3 = rotate(cockpit_points[2], angle) + Vec2::new(self.x, self.y);

        draw_triangle(cp1, cp2, cp3, Color::new(0.6, 0.8, 1.0, 0.9));

        // Die Flügel müssen hinten am Rumpf ein bisschen näher ans Raumschiff
        // === FLÜGEL ===
        // Linker Flügel (kleines Dreieck, das direkt an der linken Seite des Rumpfes ansetzt)
        let wing_l1 = rotate(Vec2::new(-self.size * 0.25, self.size * 0.0), angle)
            + Vec2::new(self.x, self.y); // Ansatz links
        let wing_l2 = rotate(Vec2::new(-self.size * 0.8, self.size * 0.30), angle)
            + Vec2::new(self.x, self.y); // außen, hinten (näher)
        let wing_l3 =
            rotate(Vec2::new(-self.size * 0.2, self.size * 0.5), angle) + Vec2::new(self.x, self.y); // hinten, an Rumpf (näher)
        draw_triangle(wing_l1, wing_l2, wing_l3, Color::new(0.3, 0.5, 0.9, 1.0));

        // Rechter Flügel (spiegelsymmetrisch)
        let wing_r1 =
            rotate(Vec2::new(self.size * 0.25, self.size * 0.0), angle) + Vec2::new(self.x, self.y); // Ansatz rechts
        let wing_r2 =
            rotate(Vec2::new(self.size * 0.8, self.size * 0.3), angle) + Vec2::new(self.x, self.y); // außen, hinten (näher)
        let wing_r3 =
            rotate(Vec2::new(self.size * 0.2, self.size * 0.5), angle) + Vec2::new(self.x, self.y); // hinten, an Rumpf (näher)
        draw_triangle(wing_r1, wing_r2, wing_r3, Color::new(0.3, 0.5, 0.9, 1.0));

        // === BLINKENDE POSITIONSLICHTER ===
        let time = get_time() as f32;
        let blink = (time * 3.0).sin() > 0.0;

        if blink {
            // Licht an linker Flügelspitze
            draw_circle(wing_l2.x, wing_l2.y, self.size * 0.1, RED);

            // Licht an rechter Flügelspitze
            draw_circle(wing_r2.x, wing_r2.y, self.size * 0.1, GREEN);
        }

        // Pulsierendes Cockpit-Licht
        let pulse = (time * 5.0).sin() * 0.3 + 0.7;
        let cockpit_center =
            rotate(Vec2::new(0.0, -self.size * 0.4), angle) + Vec2::new(self.x, self.y);
        draw_circle(
            cockpit_center.x,
            cockpit_center.y,
            self.size * 0.1,
            Color::new(0.8, 0.9, 1.0, pulse),
        );

        // --- Triebwerk Offset (lokal nach "hinten") ---
        let engine_offset = Vec2::new(0.0, self.size * 0.7);
        let world_engine = rotate(engine_offset, angle) + Vec2::new(self.x, self.y);

        // Flammenflickern
        let time = get_time() as f32;
        let flame_flicker = (time * 15.0).sin() * 0.1 + 1.0;
        let flame_length = self.size * 0.4 * flame_flicker;

        // Hauptflame (mehrere Schichten)
        for i in 0..3 {
            let offset = Vec2::new(0.0, self.size * 0.2 + i as f32 * flame_length * 0.3);
            let world_offset = rotate(offset, angle) + world_engine;

            let flame_width = self.size * (0.25 - i as f32 * 0.05) * flame_flicker;
            let alpha = 1.0 - i as f32 * 0.3;

            let color = match i {
                0 => Color::new(1.0, 0.3, 0.0, alpha),
                1 => Color::new(1.0, 0.5, 0.0, alpha),
                _ => Color::new(1.0, 0.8, 0.2, alpha),
            };

            draw_circle(world_offset.x, world_offset.y, flame_width, color);
        }

        // Innere heiße Flamme
        let inner_flame_size = self.size * 0.15 * flame_flicker;
        let inner_offset = Vec2::new(0.0, self.size * 0.3);
        let world_inner = rotate(inner_offset, angle) + world_engine;

        draw_circle(
            world_inner.x,
            world_inner.y,
            inner_flame_size,
            Color::new(0.8, 0.9, 1.0, 0.8),
        );

        // --- Funken-Effekt ---
        let spark_count = 8;
        for i in 0..spark_count {
            let spark_time = (time * 2.0 + i as f32 * 1.7) % 1.0;
            let life = 1.0 - spark_time;

            // zufällige Drift
            let rand_x = ((i as f32 * 17.57 + time * 3.1).sin()) * self.size * 0.2;
            let rand_y = spark_time * (self.size * 1.2);

            // lokaler Startpunkt (hinten am Schiff)
            let local_pos = Vec2::new(rand_x, self.size * 0.8 + rand_y);

            // Rotation ins Weltkoordinatensystem
            let world_pos = rotate(local_pos, angle) + Vec2::new(self.x, self.y);

            let size = (1.5 + 0.5) * life;
            let color = Color::new(1.0, 0.7, 0.2, life * 0.8);

            if life > 0.1 {
                draw_circle(world_pos.x, world_pos.y, size, color);
            }
        }

        // Schuss-Cooldown-Balken
        let bar_width = self.size * 2.0;
        let bar_height = self.size * 0.2;
        let bar_x = self.x - bar_width / 2.0;
        let bar_y = self.y - self.size * 1.2;
        let progress = 1.0 - (self.shoot_cooldown / self.max_shoot_ccooldown).max(0.0);
        draw_rectangle(bar_x, bar_y, bar_width, bar_height, GRAY);
        draw_rectangle(bar_x, bar_y, bar_width * progress, bar_height, BLUE);

        // HP-Balken (jetzt mit Animation)
        if self.hp < self.max_hp || self.displayed_hp_progress < 1.0 {
            // Auch anzeigen, wenn Balken noch animiert
            let bar_w = self.size * 2.0;
            let bar_h = self.size * 0.2;
            let bar_x = self.x - bar_w / 2.0;
            let bar_y = self.y - self.size * 1.5;

            // Hintergrund (rot für verlorene HP)
            draw_rectangle(bar_x, bar_y, bar_w, bar_h, RED);
            // Vordergrund (grün für aktuelle HP, animiert)
            draw_rectangle(
                bar_x,
                bar_y,
                bar_w * self.displayed_hp_progress,
                bar_h,
                GREEN,
            );

            // Optional: Rahmen um den Balken
            draw_rectangle_lines(bar_x, bar_y, bar_w, bar_h, 1.0, WHITE);
        }
    }

    pub fn take_damage(&mut self, damage: f32) -> bool {
        self.hp -= damage;
        self.hp <= 0.0
    }

    pub fn is_destroyed(&self) -> bool {
        self.hp <= 0.0
    }
}
