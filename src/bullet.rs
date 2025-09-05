use crate::debris::Debris;
use crate::player::Player;
use macroquad::prelude::*;

pub struct Bullet {
    pub x: f32,
    pub y: f32,
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub size: f32,
    pub damage: f32,
    pub speed: f32,
    pub pierce_count: u8,
    pub remaining_pierces: u8,
    pub explosion_damage: f32,
    pub armor_penetration: f32,
    pub homing_strength: f32,
    pub is_critical: bool,
    pub lifetime: f32,
    pub max_lifetime: f32,
}

impl Bullet {
    pub fn new(x: f32, y: f32, angle: f32) -> Self {
        let base_speed = screen_height() * 1.0;
        let shoot_angle = angle - std::f32::consts::FRAC_PI_2;

        Bullet {
            x,
            y,
            velocity_x: shoot_angle.cos() * base_speed,
            velocity_y: shoot_angle.sin() * base_speed,
            size: screen_width().min(screen_height()) * 0.01,
            damage: 2.5,
            speed: base_speed,
            pierce_count: 0,
            remaining_pierces: 0,
            explosion_damage: 0.0,
            armor_penetration: 0.0,
            homing_strength: 0.0,
            is_critical: false,
            lifetime: 0.0,
            max_lifetime: 5.0, // 5 seconds max lifetime
        }
    }

    pub fn new_with_player_skills(x: f32, y: f32, angle: f32, player: &Player) -> Self {
        let mut bullet = Self::new(x, y, angle);

        // Apply player skill modifiers
        bullet.damage *= player.damage_multiplier;
        bullet.speed *= player.bullet_speed_multiplier;
        bullet.size *= player.bullet_size_multiplier;
        bullet.pierce_count = player.bullet_pierce_count;
        bullet.remaining_pierces = player.bullet_pierce_count;
        bullet.explosion_damage = player.bullet_explosion_damage;
        bullet.armor_penetration = player.armor_penetration;
        bullet.homing_strength = player.bullet_homing_strength;

        // Check for critical hit
        if rand::gen_range(0.0, 1.0) < player.crit_chance {
            bullet.is_critical = true;
            bullet.damage *= 2.0; // Critical hits do double damage
        }

        // Update velocity with new speed
        let shoot_angle = angle - std::f32::consts::FRAC_PI_2;
        bullet.velocity_x = shoot_angle.cos() * bullet.speed;
        bullet.velocity_y = shoot_angle.sin() * bullet.speed;

        bullet
    }

    pub fn update(&mut self, debris: &Vec<Debris>) {
        let dt = get_frame_time();
        self.lifetime += dt;

        // Apply homing if enabled
        if self.homing_strength > 0.0 && !debris.is_empty() {
            self.apply_homing(debris, dt);
        }

        // Update position
        self.x += self.velocity_x * dt;
        self.y += self.velocity_y * dt;
    }

    fn apply_homing(&mut self, debris: &Vec<Debris>, dt: f32) {
        // Find closest debris
        let mut closest_distance = f32::INFINITY;
        let mut closest_debris: Option<&Debris> = None;

        for debris_piece in debris {
            let dx = debris_piece.x - self.x;
            let dy = debris_piece.y - self.y;
            let distance = (dx * dx + dy * dy).sqrt();

            if distance < closest_distance {
                closest_distance = distance;
                closest_debris = Some(debris_piece);
            }
        }

        if let Some(target) = closest_debris {
            // Calculate direction to target
            let dx = target.x - self.x;
            let dy = target.y - self.y;
            let distance = (dx * dx + dy * dy).sqrt();

            if distance > 0.0 {
                let target_vel_x = (dx / distance) * self.speed;
                let target_vel_y = (dy / distance) * self.speed;

                // Lerp towards target direction based on homing strength
                let homing_factor = self.homing_strength * dt * 5.0; // Adjust multiplier for responsiveness
                self.velocity_x = lerp(self.velocity_x, target_vel_x, homing_factor);
                self.velocity_y = lerp(self.velocity_y, target_vel_y, homing_factor);
            }
        }
    }

    pub fn draw(&self) {
        let time = get_time() as f32;
        let pulse_base = 1.0 + 0.15 * (time * 8.0 + self.x * 0.1 + self.y * 0.1).sin();

        // Critical bullets have different visual effects
        let (core_color, glow_color, spark_color) = if self.is_critical {
            (
                Color::new(1.0, 0.3, 0.3, 1.0), // Red core for crits
                Color::new(1.0, 0.5, 0.5, 0.4), // Red glow
                Color::new(1.0, 0.7, 0.7, 0.6), // Red sparks
            )
        } else {
            (
                Color::new(1.0, 1.0, 0.9, 0.9), // Normal white-yellow core
                Color::new(0.5, 0.9, 1.0, 0.4), // Blue glow
                Color::new(0.8, 0.9, 1.0, 0.6), // Blue sparks
            )
        };

        // Piercing bullets have additional visual effects
        let pierce_multiplier = if self.pierce_count > 0 { 1.3 } else { 1.0 };
        let pulse = pulse_base * pierce_multiplier;

        // Äußerer Glow (angepasst für verschiedene Bullet-Typen)
        draw_circle(
            self.x,
            self.y,
            self.size * 2.2 * pulse,
            Color::new(glow_color.r, glow_color.g, glow_color.b, 0.2),
        );

        // Mittlerer Glow
        draw_circle(self.x, self.y, self.size * 1.6 * pulse, glow_color);

        // Innerer Kern
        draw_circle(self.x, self.y, self.size * 1.1, core_color);

        // Zentraler Kern
        draw_circle(self.x, self.y, self.size * 0.6, WHITE);

        // Plasma-Funken (mehr für piercing bullets)
        let spark_count = if self.pierce_count > 0 { 6 } else { 4 };
        for i in 0..spark_count {
            let spark_time = time * 12.0 + i as f32 * (6.28 / spark_count as f32);
            let spark_distance = self.size * 1.8 * pierce_multiplier;
            let spark_x = self.x + spark_time.cos() * spark_distance;
            let spark_y = self.y + spark_time.sin() * spark_distance;
            let spark_size = self.size * 0.3 * (1.0 + 0.5 * (spark_time * 2.0).sin());

            draw_circle(spark_x, spark_y, spark_size, spark_color);
        }

        // Explosion preview for explosive bullets
        if self.explosion_damage > 0.0 {
            let explosion_radius = self.size * 3.0;
            let explosion_alpha = 0.1 + 0.05 * (time * 10.0).sin();
            draw_circle(
                self.x,
                self.y,
                explosion_radius,
                Color::new(1.0, 0.5, 0.0, explosion_alpha),
            );
        }
    }

    pub fn is_off_screen(&self) -> bool {
        self.lifetime > self.max_lifetime
            || self.x < -self.size
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
        bullets.retain_mut(|bullet| {
            let mut bullet_hit = false;

            for debris_piece in debris.iter_mut() {
                if bullet.collides_with(debris_piece) {
                    // Calculate final damage with armor penetration
                    let mut final_damage = bullet.damage;

                    // Apply armor penetration (reduces enemy damage reduction)
                    if bullet.armor_penetration > 0.0 {
                        // This would reduce debris armor if implemented
                        final_damage *= 1.0 + bullet.armor_penetration;
                    }

                    debris_piece.take_damage(final_damage);

                    // Handle explosion damage
                    if bullet.explosion_damage > 0.0 {
                        Self::apply_explosion_damage(bullet, debris);
                    }

                    bullet_hit = true;

                    // Handle piercing
                    if bullet.remaining_pierces > 0 {
                        bullet.remaining_pierces -= 1;
                        bullet_hit = false; // Don't destroy bullet yet
                    }

                    break; // Only hit one enemy per frame
                }
            }

            !bullet_hit // Keep bullet if it didn't hit or still has pierces
        });
    }

    fn apply_explosion_damage(bullet: &Bullet, debris: &mut Vec<Debris>) {
        let explosion_radius = bullet.size * 4.0; // Explosion radius
        let explosion_damage = bullet.damage * bullet.explosion_damage;

        for debris_piece in debris.iter_mut() {
            let dx = debris_piece.x - bullet.x;
            let dy = debris_piece.y - bullet.y;
            let distance = (dx * dx + dy * dy).sqrt();

            if distance <= explosion_radius {
                // Damage falls off with distance
                let damage_multiplier = 1.0 - (distance / explosion_radius);
                let final_explosion_damage = explosion_damage * damage_multiplier;
                debris_piece.take_damage(final_explosion_damage);
            }
        }
    }
}

// Helper function for linear interpolation
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t.clamp(0.0, 1.0)
}
