use macroquad::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ItemType {
    Shield,       // Schutz für ein paar Sekunden
    SpeedBoost,   // Schneller bewegen, aber schwerer zu kontrollieren
    SlowMotion,   // Gegner werden langsamer
    Magnet,       // Zieht andere Items an
    PhaseShift,   // Kurz durch Gegner hindurchfliegen können
    TimeFreeze,   // Gegner frieren für kurze Zeit ein
    DoublePoints, // Doppelte Punkte für eine Weile
    Overdrive,    // Mehr Punkte, aber größere Hitbox
    BlackHole,    // Saugt Gegner in der Nähe weg
}

#[derive(Debug, Clone)]
pub struct Item {
    pub position: Vec2,
    pub item_type: ItemType,
    pub spawn_time: f32,
    pub lifetime: f32,
    pub blink_start_time: f32,
    pub size: f32,
    pub rotation: f32,
    pub pulse_phase: f32,
}

pub struct ItemManager {
    pub items: Vec<Item>,
    spawn_timer: f32,
    spawn_interval: f32,
    item_colors: HashMap<ItemType, Color>,
}

impl ItemManager {
    pub fn new() -> Self {
        let mut item_colors = HashMap::new();

        // Farben für verschiedene Item-Typen
        item_colors.insert(ItemType::Shield, Color::new(0.0, 0.0, 1.0, 1.0)); // Blau
        item_colors.insert(ItemType::SpeedBoost, Color::new(1.0, 1.0, 0.0, 1.0)); // Gelb
        item_colors.insert(ItemType::SlowMotion, Color::new(0.5, 0.0, 0.5, 1.0)); // Lila
        item_colors.insert(ItemType::Magnet, Color::new(1.0, 0.5, 0.0, 1.0)); // Orange
        item_colors.insert(ItemType::PhaseShift, Color::new(0.5, 0.8, 1.0, 1.0)); // Himmelblau
        item_colors.insert(ItemType::TimeFreeze, Color::new(1.0, 1.0, 1.0, 1.0)); // Weiß
        item_colors.insert(ItemType::DoublePoints, Color::new(0.0, 1.0, 0.0, 1.0)); // Grün
        item_colors.insert(ItemType::Overdrive, Color::new(1.0, 0.0, 0.0, 1.0)); // Rot
        item_colors.insert(ItemType::BlackHole, Color::new(0.3, 0.0, 0.3, 1.0)); // Dunkellila

        Self {
            items: Vec::new(),
            spawn_timer: 0.0,
            spawn_interval: 3.0, // Alle 3 Sekunden ein neues Item
            item_colors,
        }
    }

    pub fn update(&mut self, dt: f32) {
        // Spawn-Timer aktualisieren
        self.spawn_timer += dt;
        if self.spawn_timer >= self.spawn_interval {
            self.spawn_random_item();
            self.spawn_timer = 0.0;
        }

        // Items aktualisieren
        for item in &mut self.items {
            item.rotation += dt * 2.0; // Langsame Rotation
            item.pulse_phase += dt * 4.0; // Pulsieren für Animation
        }

        // Abgelaufene Items entfernen
        let current_time = get_time() as f32;
        self.items
            .retain(|item| current_time - item.spawn_time < item.lifetime);
    }

    fn spawn_random_item(&mut self) {
        let item_types = vec![
            ItemType::Shield,
            ItemType::SpeedBoost,
            ItemType::SlowMotion,
            ItemType::Magnet,
            ItemType::PhaseShift,
            ItemType::TimeFreeze,
            ItemType::DoublePoints,
            ItemType::Overdrive,
            ItemType::BlackHole,
        ];

        let random_type = item_types[rand::gen_range(0, item_types.len())].clone();
        let current_time = get_time() as f32;

        let item = Item {
            position: Vec2::new(
                rand::gen_range(50.0, screen_width() - 50.0),
                rand::gen_range(50.0, screen_height() - 50.0),
            ),
            item_type: random_type,
            spawn_time: current_time,
            lifetime: 8.0,         // 8 Sekunden Lebensdauer
            blink_start_time: 6.0, // Nach 6 Sekunden anfangen zu blinken
            size: 20.0,
            rotation: 0.0,
            pulse_phase: 0.0,
        };

        self.items.push(item);
    }

    pub fn draw(&self) {
        let current_time = get_time() as f32;

        for item in &self.items {
            let age = current_time - item.spawn_time;
            let base_color = *self
                .item_colors
                .get(&item.item_type)
                .unwrap_or(&Color::new(1.0, 1.0, 1.0, 1.0));

            // Blinken kurz vor dem Verschwinden
            let mut alpha = 1.0;
            if age > item.blink_start_time {
                let blink_speed = 8.0;
                alpha = (0.3 + 0.7 * (age * blink_speed).sin().abs()).clamp(0.0, 1.0);
            }

            // Pulsieren für Animation
            let pulse_scale = 1.0 + 0.1 * item.pulse_phase.sin();
            let draw_size = item.size * pulse_scale;

            let color = Color::new(base_color.r, base_color.g, base_color.b, alpha);

            // Item-spezifisches Design zeichnen
            match item.item_type {
                ItemType::Shield => {
                    self.draw_shield(item.position, draw_size, item.rotation, color);
                }
                ItemType::SpeedBoost => {
                    self.draw_speed_boost(item.position, draw_size, item.rotation, color);
                }
                ItemType::SlowMotion => {
                    self.draw_slow_motion(item.position, draw_size, item.rotation, color);
                }
                ItemType::Magnet => {
                    self.draw_magnet(item.position, draw_size, item.rotation, color);
                }
                ItemType::PhaseShift => {
                    self.draw_phase_shift(item.position, draw_size, item.rotation, color);
                }
                ItemType::TimeFreeze => {
                    self.draw_time_freeze(item.position, draw_size, item.rotation, color);
                }
                ItemType::DoublePoints => {
                    self.draw_double_points(item.position, draw_size, item.rotation, color);
                }
                ItemType::Overdrive => {
                    self.draw_overdrive(item.position, draw_size, item.rotation, color);
                }
                ItemType::BlackHole => {
                    self.draw_black_hole(item.position, draw_size, item.rotation, color);
                }
            }
        }
    }

    // Shield: Klassische Schild-Form
    fn draw_shield(&self, pos: Vec2, size: f32, _rot: f32, color: Color) {
        // Umriss Schild (Polygon)
        let points = [
            Vec2::new(pos.x - size * 0.6, pos.y - size * 0.4),
            Vec2::new(pos.x + size * 0.6, pos.y - size * 0.4),
            Vec2::new(pos.x + size * 0.5, pos.y + size * 0.6),
            Vec2::new(pos.x, pos.y + size * 0.9),
            Vec2::new(pos.x - size * 0.5, pos.y + size * 0.6),
        ];

        for i in 0..points.len() {
            let j = (i + 1) % points.len();
            draw_line(
                points[i].x,
                points[i].y,
                points[j].x,
                points[j].y,
                3.0,
                color,
            );
        }

        // Vertikale Teilung
        draw_line(
            pos.x,
            pos.y - size * 0.4,
            pos.x,
            pos.y + size * 0.7,
            2.0,
            color,
        );

        // leichte Füllung
        draw_circle(
            pos.x,
            pos.y,
            size * 0.4,
            Color::new(color.r, color.g, color.b, 0.2),
        );
    }

    // SpeedBoost: Deutlicher Blitz
    fn draw_speed_boost(&self, pos: Vec2, size: f32, _rot: f32, color: Color) {
        let bolt = [
            Vec2::new(pos.x - size * 0.2, pos.y - size * 0.8),
            Vec2::new(pos.x + size * 0.3, pos.y - size * 0.2),
            Vec2::new(pos.x + size * 0.0, pos.y - size * 0.2),
            Vec2::new(pos.x + size * 0.4, pos.y + size * 0.7),
            Vec2::new(pos.x - size * 0.2, pos.y + size * 0.1),
            Vec2::new(pos.x + size * 0.1, pos.y + size * 0.1),
        ];

        for i in 0..bolt.len() {
            let j = (i + 1) % bolt.len();
            draw_line(bolt[i].x, bolt[i].y, bolt[j].x, bolt[j].y, 3.5, color);
        }

        // Glüheffekt innen
        for i in 0..bolt.len() {
            let j = (i + 1) % bolt.len();
            draw_line(
                bolt[i].x,
                bolt[i].y,
                bolt[j].x,
                bolt[j].y,
                1.5,
                Color::new(1.0, 1.0, 0.7, color.a),
            );
        }
    }

    // SlowMotion: Uhr mit Zeigern
    fn draw_slow_motion(&self, pos: Vec2, size: f32, rotation: f32, color: Color) {
        // Äußerer Uhrenkreis
        draw_circle_lines(pos.x, pos.y, size, 3.0, color);

        // Stundenmarkierungen (12, 3, 6, 9 Uhr)
        for i in 0..12 {
            let angle = i as f32 * std::f32::consts::PI / 6.0;
            let inner_radius = if i % 3 == 0 { size * 0.8 } else { size * 0.9 };
            let outer_radius = size * 0.95;

            let inner = Vec2::new(
                pos.x + angle.cos() * inner_radius,
                pos.y + angle.sin() * inner_radius,
            );
            let outer = Vec2::new(
                pos.x + angle.cos() * outer_radius,
                pos.y + angle.sin() * outer_radius,
            );

            let thickness = if i % 3 == 0 { 3.0 } else { 1.0 };
            draw_line(inner.x, inner.y, outer.x, outer.y, thickness, color);
        }

        // Uhrzeiger (langsam)
        let hour_angle = rotation * 0.1; // sehr langsam
        let minute_angle = rotation * 0.5; // etwas schneller

        let hour_hand = Vec2::new(
            pos.x + hour_angle.cos() * size * 0.5,
            pos.y + hour_angle.sin() * size * 0.5,
        );
        let minute_hand = Vec2::new(
            pos.x + minute_angle.cos() * size * 0.7,
            pos.y + minute_angle.sin() * size * 0.7,
        );

        draw_line(pos.x, pos.y, hour_hand.x, hour_hand.y, 4.0, color);
        draw_line(pos.x, pos.y, minute_hand.x, minute_hand.y, 2.0, color);

        // Mittelpunkt
        draw_circle(pos.x, pos.y, size * 0.1, color);
    }

    // Magnet: Hufeisen mit Magnetfeld
    fn draw_magnet(&self, pos: Vec2, size: f32, rotation: f32, color: Color) {
        // U-Form (Hufeisen)
        let center_y = pos.y - size * 0.2;
        draw_circle_lines(pos.x, center_y, size * 0.6, 4.0, color);

        // Pole (Enden des Hufeisens)
        draw_rectangle(
            pos.x - size * 0.6 - 4.0,
            pos.y - size * 0.8,
            8.0,
            size * 0.6,
            color,
        );
        draw_rectangle(
            pos.x + size * 0.6 - 4.0,
            pos.y - size * 0.8,
            8.0,
            size * 0.6,
            color,
        );

        // N und S Pole (farblich unterschiedlich)
        draw_rectangle(
            pos.x - size * 0.6 - 4.0,
            pos.y - size * 0.8,
            8.0,
            size * 0.3,
            Color::new(1.0, 0.0, 0.0, color.a),
        ); // Rot (N)
        draw_rectangle(
            pos.x + size * 0.6 - 4.0,
            pos.y - size * 0.8,
            8.0,
            size * 0.3,
            Color::new(0.0, 0.0, 1.0, color.a),
        ); // Blau (S)

        // Magnetfeld-Linien (animiert)
        for i in 0..5 {
            let offset = (i as f32 - 2.0) * 6.0;
            let wave_offset = (rotation * 3.0 + i as f32).sin() * 3.0;
            draw_line(
                pos.x + offset,
                pos.y - size * 1.2,
                pos.x + offset + wave_offset,
                pos.y - size * 1.5,
                1.0,
                Color::new(color.r, color.g, color.b, color.a * 0.7),
            );
        }

        // Anziehende Partikel
        for i in 0..3 {
            let particle_angle = rotation + i as f32 * 2.0;
            let particle_distance = size * 1.2 + (rotation * 2.0).sin() * 5.0;
            let particle_pos = Vec2::new(
                pos.x + particle_angle.cos() * particle_distance,
                pos.y + particle_angle.sin() * particle_distance,
            );
            draw_circle(
                particle_pos.x,
                particle_pos.y,
                2.0,
                Color::new(color.r, color.g, color.b, color.a * 0.8),
            );
        }
    }

    // PhaseShift: Geist-Effekt mit Ringen
    fn draw_phase_shift(&self, pos: Vec2, size: f32, rotation: f32, color: Color) {
        // Mehrere pulsierende Ringe
        for i in 0..4 {
            let ring_size = size * (0.4 + i as f32 * 0.2);
            let alpha =
                (color.a * (0.8 - i as f32 * 0.15) * (rotation * 2.0 + i as f32).sin().abs())
                    .clamp(0.0, 1.0);
            let ring_color = Color::new(color.r, color.g, color.b, alpha);

            draw_circle_lines(pos.x, pos.y, ring_size, 2.0, ring_color);
        }

        // Zentrale transparente Kugel
        draw_circle(
            pos.x,
            pos.y,
            size * 0.3,
            Color::new(color.r, color.g, color.b, 0.3),
        );

        // Kleine Sterne um das Item
        for i in 0..6 {
            let star_angle = rotation * 2.0 + i as f32 * std::f32::consts::PI / 3.0;
            let star_distance = size * 1.2;
            let star_pos = Vec2::new(
                pos.x + star_angle.cos() * star_distance,
                pos.y + star_angle.sin() * star_distance,
            );

            // Kleiner Stern (4-zackig)
            let star_size = 3.0;
            draw_line(
                star_pos.x - star_size,
                star_pos.y,
                star_pos.x + star_size,
                star_pos.y,
                1.0,
                color,
            );
            draw_line(
                star_pos.x,
                star_pos.y - star_size,
                star_pos.x,
                star_pos.y + star_size,
                1.0,
                color,
            );
        }
    }

    // TimeFreeze: Schneeflocke mit Ästen
    fn draw_time_freeze(&self, pos: Vec2, size: f32, _rot: f32, color: Color) {
        let arms = 6;
        for i in 0..arms {
            let angle = i as f32 * std::f32::consts::PI * 2.0 / arms as f32;
            let end = Vec2::new(pos.x + angle.cos() * size, pos.y + angle.sin() * size);

            // Hauptarm
            draw_line(pos.x, pos.y, end.x, end.y, 2.0, color);

            // kleine Zweige am Ende
            let branch_angle1 = angle + std::f32::consts::PI / 6.0;
            let branch_angle2 = angle - std::f32::consts::PI / 6.0;
            let branch_size = size * 0.3;

            let b1 = Vec2::new(
                end.x + branch_angle1.cos() * branch_size,
                end.y + branch_angle1.sin() * branch_size,
            );
            let b2 = Vec2::new(
                end.x + branch_angle2.cos() * branch_size,
                end.y + branch_angle2.sin() * branch_size,
            );

            draw_line(end.x, end.y, b1.x, b1.y, 1.5, color);
            draw_line(end.x, end.y, b2.x, b2.y, 1.5, color);
        }

        // Schneeflocken-Kern
        draw_circle(pos.x, pos.y, size * 0.15, color);
    }

    // DoublePoints: Zwei goldene Sterne
    fn draw_double_points(&self, pos: Vec2, size: f32, rotation: f32, color: Color) {
        // Erster Stern (größer, hinten)
        let star1_pos = Vec2::new(pos.x - 3.0, pos.y - 3.0);
        self.draw_star(star1_pos, size, rotation, color);

        // Zweiter Stern (kleiner, vorne)
        let star2_pos = Vec2::new(pos.x + 4.0, pos.y + 4.0);
        let bright_color = Color::new(1.0, 1.0, 0.0, color.a); // Goldgelb
        self.draw_star(star2_pos, size * 0.7, rotation + 0.5, bright_color);

        // "x2" Andeutung in der Mitte
        draw_circle(pos.x, pos.y, 3.0, Color::new(0.0, 1.0, 0.0, color.a * 0.8));
        draw_circle(
            pos.x + 2.0,
            pos.y,
            2.0,
            Color::new(0.0, 1.0, 0.0, color.a * 0.8),
        );
    }

    // Overdrive: Funkenstern mit Flammen
    fn draw_overdrive(&self, pos: Vec2, size: f32, rotation: f32, color: Color) {
        // Äußere Flammenzacken (8-zackig)
        for i in 0..8 {
            let angle = rotation + i as f32 * std::f32::consts::PI / 4.0;
            let tip = Vec2::new(pos.x + angle.cos() * size, pos.y + angle.sin() * size);
            let base1 = Vec2::new(
                pos.x + (angle - 0.4).cos() * size * 0.4,
                pos.y + (angle - 0.4).sin() * size * 0.4,
            );
            let base2 = Vec2::new(
                pos.x + (angle + 0.4).cos() * size * 0.4,
                pos.y + (angle + 0.4).sin() * size * 0.4,
            );

            // Flammen-Farbe (rot-orange Gradient)
            let flame_color = if i % 2 == 0 {
                Color::new(1.0, 0.0, 0.0, color.a)
            } else {
                Color::new(1.0, 0.5, 0.0, color.a)
            };

            draw_triangle(tip, base1, base2, flame_color);
        }

        // Innerer Kern (gefährlich aussehend)
        draw_circle(pos.x, pos.y, size * 0.4, Color::new(1.0, 0.2, 0.0, color.a));
        draw_circle(pos.x, pos.y, size * 0.2, Color::new(1.0, 1.0, 0.0, color.a));

        // Warnung: Blinkender Rand
        let warning_alpha = (rotation * 8.0).sin().abs() * color.a;
        draw_circle_lines(
            pos.x,
            pos.y,
            size * 0.6,
            2.0,
            Color::new(1.0, 0.0, 0.0, warning_alpha),
        );
    }

    // BlackHole: Spirale mit Sog-Effekt
    fn draw_black_hole(&self, pos: Vec2, size: f32, rotation: f32, color: Color) {
        // Äußerer Ereignishorizont
        draw_circle_lines(pos.x, pos.y, size, 2.0, color);

        // Spirale nach innen (Akkretionsscheibe)
        let mut last_pos = pos;
        for i in 0..25 {
            let t = i as f32 / 25.0;
            let spiral_angle = rotation * 2.0 + t * 8.0 * std::f32::consts::PI;
            let spiral_radius = size * (1.0 - t * t) * 0.9; // Quadratisch nach innen
            let current_pos = Vec2::new(
                pos.x + spiral_angle.cos() * spiral_radius,
                pos.y + spiral_angle.sin() * spiral_radius,
            );

            if i > 0 {
                let alpha = color.a * (1.0 - t);
                let spiral_color = Color::new(
                    color.r + t * 0.5, // Wird heller zur Mitte
                    color.g,
                    color.b + t * 0.7,
                    alpha,
                );
                draw_line(
                    last_pos.x,
                    last_pos.y,
                    current_pos.x,
                    current_pos.y,
                    2.0,
                    spiral_color,
                );
            }
            last_pos = current_pos;
        }

        // Schwarzes Zentrum
        draw_circle(pos.x, pos.y, size * 0.3, Color::new(0.0, 0.0, 0.0, 1.0));

        // Sog-Partikel um das Schwarze Loch
        for i in 0..6 {
            let particle_angle = rotation * 3.0 + i as f32 * std::f32::consts::PI / 3.0;
            let particle_distance = size * 1.5 + (rotation * 4.0 + i as f32).sin() * 10.0;
            let particle_pos = Vec2::new(
                pos.x + particle_angle.cos() * particle_distance,
                pos.y + particle_angle.sin() * particle_distance,
            );

            let particle_alpha = (1.0 - particle_distance / (size * 2.0)).clamp(0.0, 1.0) * color.a;
            draw_circle(
                particle_pos.x,
                particle_pos.y,
                2.0,
                Color::new(0.8, 0.4, 0.8, particle_alpha),
            );
        }
    }

    // Hilfsfunktion für Sterne
    fn draw_star(&self, pos: Vec2, size: f32, rotation: f32, color: Color) {
        let points = 5;
        let outer_radius = size;
        let inner_radius = size * 0.4;

        let mut star_points = Vec::new();

        for i in 0..points * 2 {
            let angle = rotation + (i as f32 * std::f32::consts::PI / points as f32);
            let radius = if i % 2 == 0 {
                outer_radius
            } else {
                inner_radius
            };

            star_points.push(Vec2::new(
                pos.x + angle.cos() * radius,
                pos.y + angle.sin() * radius,
            ));
        }

        // Stern-Körper zeichnen
        for i in 0..star_points.len() {
            let next_i = (i + 1) % star_points.len();
            draw_line(
                star_points[i].x,
                star_points[i].y,
                star_points[next_i].x,
                star_points[next_i].y,
                2.0,
                color,
            );
        }

        // Stern-Füllung
        draw_circle(
            pos.x,
            pos.y,
            inner_radius,
            Color::new(color.r, color.g, color.b, color.a * 0.3),
        );
    }

    // Getter für Collision Detection
    pub fn get_items(&self) -> &Vec<Item> {
        &self.items
    }

    // Item entfernen (für Collision)
    pub fn remove_item(&mut self, index: usize) {
        if index < self.items.len() {
            self.items.remove(index);
        }
    }
}
