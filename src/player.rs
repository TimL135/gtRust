use macroquad::prelude::*;

use crate::bullet::Bullet;
use crate::help_fn::lerp;
use crate::items::ItemType;

#[derive(Debug, Clone)]
pub struct ActiveEffect {
    pub effect_type: ItemType,
    pub remaining_time: f32,
    pub original_duration: f32,
}

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub base_size: f32, // Für Overdrive-Effekt
    pub shoot_cooldown: f32,
    pub max_shoot_ccooldown: f32,
    pub base_shoot_cooldown: f32, // Basis-Wert für Effekte
    pub rotation: f32,
    pub displayed_hp_progress: f32,
    pub hp: f32,
    pub max_hp: f32,
    pub speed_multiplier: f32,
    pub base_speed: f32,
    pub points_multiplier: f32,
    pub damage_reduction: f32,
    pub can_phase_through: bool,
    pub active_effects: Vec<ActiveEffect>,
    pub magnet_range: f32,

    // Skill Tree related fields
    pub damage_multiplier: f32,
    pub reload_speed_multiplier: f32,
    pub bullet_pierce_count: u8,
    pub bullet_speed_multiplier: f32,
    pub bullet_size_multiplier: f32,
    pub bullet_explosion_damage: f32,
    pub double_shot_chance: f32,
    pub armor_penetration: f32,
    pub bullet_homing_strength: f32,
    pub crit_chance: f32,
    pub knockback_resistance: f32,
    pub debris_damage_reduction: f32,
    pub shield_duration_bonus: f32,
    pub item_cooldown_reduction: f32,
    pub immunity_interval: f32,
    pub health_regen_rate: f32,
    pub adaptive_armor_rate: f32,
    pub phase_duration_bonus: f32,
    pub kinetic_shield_strength: f32,
    pub magnet_range_multiplier: f32,
    pub item_spawn_rate_multiplier: f32,
    pub item_attraction_speed_multiplier: f32,
    pub item_lifetime_bonus: f32,
    pub item_effect_duration_multiplier: f32,
    pub combo_bonus_multiplier: f32,
    pub death_drop_chance: f32,
    pub close_spawn_chance: f32,
    pub super_item_chance: f32,
    pub max_active_items: usize,
    pub speed_boost_effectiveness: f32,
    pub slow_motion_duration_bonus: f32,
    pub emp_chance: f32,
    pub black_hole_interval: f32,
    pub time_freeze_duration_bonus: f32,
    pub pulse_disruptor_strength: f32,
    pub overdrive_hitbox_reduction: f32,
    pub temporal_surge_strength: f32,
    pub energy_overflow_bonus: f32,

    // Timers and counters for skill effects
    pub last_immunity_time: f32,
    pub last_regen_time: f32,
    pub adaptive_armor_timer: f32,
    pub adaptive_armor_stacks: u8,
    pub last_black_hole_time: f32,
    pub pickup_count: u32,
}

impl Player {
    pub fn new() -> Self {
        let max_hp = 3.0;
        let base_size = screen_width().min(screen_height()) * 0.03;
        let base_shoot_cooldown = 0.5;
        let base_speed = screen_width().max(screen_height()) * 0.25;

        Player {
            x: screen_width() / 2.0,
            y: screen_height() - screen_height() * 0.1,
            size: base_size,
            base_size,
            shoot_cooldown: 0.0,
            max_shoot_ccooldown: base_shoot_cooldown,
            base_shoot_cooldown,
            rotation: 0.0,
            displayed_hp_progress: 1.0,
            hp: max_hp,
            max_hp,
            speed_multiplier: 1.0,
            base_speed,
            points_multiplier: 1.0,
            damage_reduction: 0.0,
            can_phase_through: false,
            active_effects: Vec::new(),
            magnet_range: 0.0,

            // Initialize skill tree fields with default values
            damage_multiplier: 1.0,
            reload_speed_multiplier: 1.0,
            bullet_pierce_count: 0,
            bullet_speed_multiplier: 1.0,
            bullet_size_multiplier: 1.0,
            bullet_explosion_damage: 0.0,
            double_shot_chance: 0.0,
            armor_penetration: 0.0,
            bullet_homing_strength: 0.0,
            crit_chance: 0.0,
            knockback_resistance: 0.0,
            debris_damage_reduction: 0.0,
            shield_duration_bonus: 0.0,
            item_cooldown_reduction: 0.0,
            immunity_interval: 0.0,
            health_regen_rate: 0.0,
            adaptive_armor_rate: 0.0,
            phase_duration_bonus: 0.0,
            kinetic_shield_strength: 0.0,
            magnet_range_multiplier: 1.0,
            item_spawn_rate_multiplier: 1.0,
            item_attraction_speed_multiplier: 1.0,
            item_lifetime_bonus: 0.0,
            item_effect_duration_multiplier: 1.0,
            combo_bonus_multiplier: 0.0,
            death_drop_chance: 0.0,
            close_spawn_chance: 0.0,
            super_item_chance: 0.0,
            max_active_items: 1,
            speed_boost_effectiveness: 1.0,
            slow_motion_duration_bonus: 0.0,
            emp_chance: 0.0,
            black_hole_interval: 0.0,
            time_freeze_duration_bonus: 0.0,
            pulse_disruptor_strength: 0.0,
            overdrive_hitbox_reduction: 0.0,
            temporal_surge_strength: 0.0,
            energy_overflow_bonus: 0.0,

            // Initialize timers
            last_immunity_time: 0.0,
            last_regen_time: 0.0,
            adaptive_armor_timer: 0.0,
            adaptive_armor_stacks: 0,
            last_black_hole_time: 0.0,
            pickup_count: 0,
        }
    }

    pub fn update(&mut self, bullets: &mut Vec<Bullet>) {
        // Aktive Effekte updaten
        self.update_effects();

        // Update skill-based timers and effects
        self.update_skill_effects();

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

            // Create primary bullet with skill modifiers
            let mut bullet = Bullet::new(bullet_x, bullet_y, self.rotation);
            bullet.damage *= self.damage_multiplier;
            bullet.speed *= self.bullet_speed_multiplier;
            bullet.size *= self.bullet_size_multiplier;
            bullet.pierce_count = self.bullet_pierce_count;
            bullets.push(bullet);

            // Check for double shot chance
            if rand::gen_range(0.0, 1.0) < self.double_shot_chance {
                let spread_angle = 0.2; // Small spread for twin cannons
                let mut bullet1 = Bullet::new(bullet_x, bullet_y, self.rotation - spread_angle);
                bullet1.damage *= self.damage_multiplier;
                bullet1.speed *= self.bullet_speed_multiplier;
                bullet1.size *= self.bullet_size_multiplier;
                bullet1.pierce_count = self.bullet_pierce_count;
                bullets.push(bullet1);

                let mut bullet2 = Bullet::new(bullet_x, bullet_y, self.rotation + spread_angle);
                bullet2.damage *= self.damage_multiplier;
                bullet2.speed *= self.bullet_speed_multiplier;
                bullet2.size *= self.bullet_size_multiplier;
                bullet2.pierce_count = self.bullet_pierce_count;
                bullets.push(bullet2);
            }

            self.shoot_cooldown = self.max_shoot_ccooldown;
        }

        let speed = self.base_speed * self.speed_multiplier;

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

    fn update_effects(&mut self) {
        let dt = get_frame_time();

        // Effekte zeitlich reduzieren
        for effect in &mut self.active_effects {
            effect.remaining_time -= dt;
        }

        // Abgelaufene Effekte entfernen
        self.active_effects
            .retain(|effect| effect.remaining_time > 0.0);

        // Werte zurücksetzen
        self.speed_multiplier = 1.0;
        self.points_multiplier = 1.0;
        self.damage_reduction = 0.0;
        self.can_phase_through = false;
        self.magnet_range = 0.0;
        self.max_shoot_ccooldown = self.base_shoot_cooldown;
        self.size = self.base_size;

        // Aktive Effekte anwenden
        for effect in &self.active_effects {
            let mut duration = effect.original_duration;

            // Apply skill bonuses to effect duration
            match effect.effect_type {
                ItemType::Shield => {
                    duration += self.shield_duration_bonus;
                    self.damage_reduction = 0.5; // 50% weniger Schaden
                }
                ItemType::SpeedBoost => {
                    self.speed_multiplier = 2.0 * self.speed_boost_effectiveness;
                }
                ItemType::SlowMotion => {
                    duration += self.slow_motion_duration_bonus;
                    // Wird in main.rs für Gegner angewendet
                }
                ItemType::Magnet => {
                    self.magnet_range =
                        (screen_width().min(screen_height())) * 0.15 * self.magnet_range_multiplier;
                }
                ItemType::PhaseShift => {
                    duration += self.phase_duration_bonus;
                    self.can_phase_through = true;
                }
                ItemType::TimeFreeze => {
                    duration += self.time_freeze_duration_bonus;
                    // Wird in main.rs für Gegner angewendet
                }
                ItemType::DoublePoints => {
                    self.points_multiplier *= 2.0;
                }
                ItemType::Overdrive => {
                    self.points_multiplier *= 3.0;
                    let hitbox_reduction = 1.0 - self.overdrive_hitbox_reduction;
                    self.size = self.base_size * 1.5 * hitbox_reduction; // Größere Hitbox mit Skill-Reduktion
                    self.max_shoot_ccooldown = self.base_shoot_cooldown * 0.3; // Schneller schießen
                }
            }

            // Apply general item duration multiplier
            duration *= self.item_effect_duration_multiplier;
        }
    }

    pub fn apply_item_effect(&mut self, item_type: ItemType) {
        let mut duration = match item_type {
            ItemType::Shield => 5.0,
            ItemType::SpeedBoost => 4.0,
            ItemType::SlowMotion => 6.0,
            ItemType::Magnet => 8.0,
            ItemType::PhaseShift => 3.0,
            ItemType::TimeFreeze => 4.0,
            ItemType::DoublePoints => 10.0,
            ItemType::Overdrive => 5.0,
        };

        // Apply skill bonuses to base duration
        match item_type {
            ItemType::Shield => duration += self.shield_duration_bonus,
            ItemType::SlowMotion => duration += self.slow_motion_duration_bonus,
            ItemType::PhaseShift => duration += self.phase_duration_bonus,
            ItemType::TimeFreeze => duration += self.time_freeze_duration_bonus,
            _ => {}
        }

        // Apply general duration multiplier
        duration *= self.item_effect_duration_multiplier;

        // Entferne gleiche Effekte (kein Stacking)
        self.active_effects
            .retain(|effect| effect.effect_type != item_type);

        // Füge neuen Effekt hinzu
        self.active_effects.push(ActiveEffect {
            effect_type: item_type,
            remaining_time: duration,
            original_duration: duration,
        });
    }

    pub fn has_effect(&self, effect_type: &ItemType) -> bool {
        self.active_effects
            .iter()
            .any(|effect| &effect.effect_type == effect_type)
    }

    pub fn draw(&self) {
        let angle = self.rotation;

        // Rotationsfunktion
        let rotate = |p: Vec2, angle: f32| -> Vec2 {
            let cos_a = angle.cos();
            let sin_a = angle.sin();
            Vec2::new(p.x * cos_a - p.y * sin_a, p.x * sin_a + p.y * cos_a)
        };

        // Shield-Effekt zeichnen
        if self.damage_reduction > 0.0 {
            let time = get_time() as f32;
            let shield_pulse = 0.8 + 0.2 * (time * 4.0).sin();
            let shield_color = Color::new(0.0, 0.5, 1.0, 0.3 * shield_pulse);
            draw_circle(self.x, self.y, self.size * 2.0 * shield_pulse, shield_color);

            // Shield-Ring
            draw_circle_lines(
                self.x,
                self.y,
                self.size * 1.8,
                3.0,
                Color::new(0.0, 0.8, 1.0, shield_pulse),
            );
        }

        // PhaseShift-Effekt
        if self.can_phase_through {
            let time = get_time() as f32;
            let phase_alpha = 0.3 + 0.4 * (time * 6.0).sin().abs();
            // Zeichne Spieler halbtransparent
            self.draw_ship_with_alpha(angle, rotate, phase_alpha);

            // Zusätzliche Geist-Ringe
            for i in 0..3 {
                let ring_size = self.size * (2.0 + i as f32 * 0.5);
                let ring_alpha = phase_alpha * (0.5 - i as f32 * 0.1);
                draw_circle_lines(
                    self.x,
                    self.y,
                    ring_size,
                    2.0,
                    Color::new(0.5, 0.8, 1.0, ring_alpha),
                );
            }
        } else {
            // Normal zeichnen
            self.draw_ship_with_alpha(angle, rotate, 1.0);
        }

        // Magnet-Effekt visualisieren
        if self.magnet_range > 0.0 {
            let time = get_time() as f32;
            let magnet_pulse = 0.5 + 0.3 * (time * 3.0).sin();
            draw_circle_lines(
                self.x,
                self.y,
                self.magnet_range * magnet_pulse,
                2.0,
                Color::new(1.0, 0.5, 0.0, 0.3),
            );
        }

        // Overdrive-Effekt
        if self.has_effect(&ItemType::Overdrive) {
            let time = get_time() as f32;
            // Rote Aura um das Schiff
            for i in 0..3 {
                let aura_size = self.size * (2.5 + i as f32 * 0.3);
                let aura_alpha = 0.2 - i as f32 * 0.05;
                draw_circle(
                    self.x,
                    self.y,
                    aura_size,
                    Color::new(1.0, 0.2, 0.0, aura_alpha * (time * 8.0).sin().abs()),
                );
            }
        }
    }

    fn draw_ship_with_alpha(&self, angle: f32, rotate: fn(Vec2, f32) -> Vec2, alpha: f32) {
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
        draw_triangle(rp1, rp2, rp3, Color::new(0.2, 0.4, 0.8, alpha));
        draw_triangle(rp2, rp4, rp5, Color::new(0.2, 0.4, 0.8, alpha));
        draw_triangle(rp2, rp5, rp3, Color::new(0.2, 0.4, 0.8, alpha));

        // === COCKPIT (heller Bereich vorne) ===
        let cockpit_points = [
            Vec2::new(0.0, -self.size * 0.8),
            Vec2::new(-self.size * 0.15, -self.size * 0.2),
            Vec2::new(self.size * 0.15, -self.size * 0.2),
        ];
        let cp1 = rotate(cockpit_points[0], angle) + Vec2::new(self.x, self.y);
        let cp2 = rotate(cockpit_points[1], angle) + Vec2::new(self.x, self.y);
        let cp3 = rotate(cockpit_points[2], angle) + Vec2::new(self.x, self.y);

        draw_triangle(cp1, cp2, cp3, Color::new(0.6, 0.8, 1.0, 0.9 * alpha));

        // Die Flügel müssen hinten am Rumpf ein bisschen näher ans Raumschiff
        // === FLÜGEL ===
        // Linker Flügel (kleines Dreieck, das direkt an der linken Seite des Rumpfes ansetzt)
        let wing_l1 = rotate(Vec2::new(-self.size * 0.25, self.size * 0.0), angle)
            + Vec2::new(self.x, self.y); // Ansatz links
        let wing_l2 = rotate(Vec2::new(-self.size * 0.8, self.size * 0.30), angle)
            + Vec2::new(self.x, self.y); // außen, hinten (näher)
        let wing_l3 =
            rotate(Vec2::new(-self.size * 0.2, self.size * 0.5), angle) + Vec2::new(self.x, self.y); // hinten, an Rumpf (näher)
        draw_triangle(wing_l1, wing_l2, wing_l3, Color::new(0.3, 0.5, 0.9, alpha));

        // Rechter Flügel (spiegelsymmetrisch)
        let wing_r1 =
            rotate(Vec2::new(self.size * 0.25, self.size * 0.0), angle) + Vec2::new(self.x, self.y); // Ansatz rechts
        let wing_r2 =
            rotate(Vec2::new(self.size * 0.8, self.size * 0.3), angle) + Vec2::new(self.x, self.y); // außen, hinten (näher)
        let wing_r3 =
            rotate(Vec2::new(self.size * 0.2, self.size * 0.5), angle) + Vec2::new(self.x, self.y); // hinten, an Rumpf (näher)
        draw_triangle(wing_r1, wing_r2, wing_r3, Color::new(0.3, 0.5, 0.9, alpha));

        // === BLINKENDE POSITIONSLICHTER ===
        let time = get_time() as f32;
        let blink = (time * 3.0).sin() > 0.0;

        if blink {
            // Licht an linker Flügelspitze
            draw_circle(
                wing_l2.x,
                wing_l2.y,
                self.size * 0.1,
                Color::new(1.0, 0.0, 0.0, alpha),
            );

            // Licht an rechter Flügelspitze
            draw_circle(
                wing_r2.x,
                wing_r2.y,
                self.size * 0.1,
                Color::new(0.0, 1.0, 0.0, alpha),
            );
        }

        // Pulsierendes Cockpit-Licht
        let pulse = (time * 5.0).sin() * 0.3 + 0.7;
        let cockpit_center =
            rotate(Vec2::new(0.0, -self.size * 0.4), angle) + Vec2::new(self.x, self.y);
        draw_circle(
            cockpit_center.x,
            cockpit_center.y,
            self.size * 0.1,
            Color::new(0.8, 0.9, 1.0, pulse * alpha),
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
            let flame_alpha = (1.0 - i as f32 * 0.3) * alpha;

            let color = match i {
                0 => Color::new(1.0, 0.3, 0.0, flame_alpha),
                1 => Color::new(1.0, 0.5, 0.0, flame_alpha),
                _ => Color::new(1.0, 0.8, 0.2, flame_alpha),
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
            Color::new(0.8, 0.9, 1.0, 0.8 * alpha),
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
            let color = Color::new(1.0, 0.7, 0.2, life * 0.8 * alpha);

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

        // Aktive Effekte anzeigen
        self.draw_active_effects();
    }

    fn draw_active_effects(&self) {
        let effect_bar_width = 100.0;
        let effect_bar_height = 8.0;
        let start_x = self.x - (effect_bar_width + 5.0) / 2.0;
        let start_y = self.y + self.size * 1.5;

        for (i, effect) in self.active_effects.iter().enumerate() {
            let bar_y = start_y + i as f32 * (effect_bar_height + 1.0);
            let progress = effect.remaining_time / effect.original_duration;

            let color = match effect.effect_type {
                ItemType::Shield => Color::new(0.0, 0.5, 1.0, 0.8),
                ItemType::SpeedBoost => Color::new(1.0, 1.0, 0.0, 0.8),
                ItemType::SlowMotion => Color::new(0.5, 0.0, 0.5, 0.8),
                ItemType::Magnet => Color::new(1.0, 0.5, 0.0, 0.8),
                ItemType::PhaseShift => Color::new(0.5, 0.8, 1.0, 0.8),
                ItemType::TimeFreeze => Color::new(1.0, 1.0, 1.0, 0.8),
                ItemType::DoublePoints => Color::new(0.0, 1.0, 0.0, 0.8),
                ItemType::Overdrive => Color::new(1.0, 0.0, 0.0, 0.8),
            };

            // Hintergrund
            draw_rectangle(
                start_x,
                bar_y,
                effect_bar_width,
                effect_bar_height,
                Color::new(0.2, 0.2, 0.2, 0.8),
            );

            // Fortschritt
            draw_rectangle(
                start_x,
                bar_y,
                effect_bar_width * progress,
                effect_bar_height,
                color,
            );

            // Rahmen
            draw_rectangle_lines(
                start_x,
                bar_y,
                effect_bar_width,
                effect_bar_height,
                1.0,
                WHITE,
            );
        }
    }

    pub fn take_damage(&mut self, damage: f32) -> bool {
        let current_time = get_time() as f32;

        // Check reactive armor immunity
        if self.immunity_interval > 0.0
            && current_time - self.last_immunity_time < self.immunity_interval
        {
            return false; // Immune to damage
        }

        // Apply damage with all reductions
        let mut final_damage = damage;
        final_damage *= 1.0 - self.damage_reduction; // Item-based reduction
        final_damage *= 1.0 - self.debris_damage_reduction; // Skill-based debris reduction
        final_damage *= 1.0 - (self.adaptive_armor_stacks as f32 * self.adaptive_armor_rate); // Adaptive plating

        self.hp -= final_damage;

        // Reset adaptive armor stacks when taking damage
        self.adaptive_armor_stacks = 0;
        self.adaptive_armor_timer = 0.0;

        // Update immunity timer
        if self.immunity_interval > 0.0 {
            self.last_immunity_time = current_time;
        }

        // Trigger kinetic shielding if available
        if self.kinetic_shield_strength > 0.0 {
            // This would trigger a temporary speed boost - handled in update_skill_effects
        }

        self.hp <= 0.0
    }

    pub fn is_destroyed(&self) -> bool {
        self.hp <= 0.0
    }

    pub fn get_position(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    pub fn get_pickup_radius(&self) -> f32 {
        self.size * 1.5
    }

    fn update_skill_effects(&mut self) {
        let dt = get_frame_time();
        let current_time = get_time() as f32;

        // Health regeneration
        if self.health_regen_rate > 0.0 && current_time - self.last_regen_time >= 1.0 {
            self.hp = (self.hp + self.health_regen_rate).min(self.max_hp);
            self.last_regen_time = current_time;
        }

        // Adaptive armor buildup (when not taking damage)
        self.adaptive_armor_timer += dt;
        if self.adaptive_armor_timer >= 3.0 && self.adaptive_armor_rate > 0.0 {
            self.adaptive_armor_stacks = (self.adaptive_armor_stacks + 1).min(5); // Max 5 stacks
            self.adaptive_armor_timer = 0.0;
        }

        // Mini black hole effect
        if self.black_hole_interval > 0.0
            && current_time - self.last_black_hole_time >= self.black_hole_interval
        {
            // This would trigger a gravitational pull effect - to be implemented in main.rs
            self.last_black_hole_time = current_time;
        }
    }

    pub fn on_item_pickup(&mut self) {
        self.pickup_count += 1;

        // Golden Touch effect (every 10th pickup)
        if self.pickup_count % 10 == 0 {
            // This would add bonus points - to be handled in main.rs
        }

        // Energy overflow effect
        if self.energy_overflow_bonus > 0.0 {
            // Extend all active effects
            for effect in &mut self.active_effects {
                effect.remaining_time += effect.original_duration * self.energy_overflow_bonus;
            }
        }

        // Temporal surge effect
        if self.temporal_surge_strength > 0.0 {
            // This would slow down the game temporarily - to be handled in main.rs
        }
    }
}
