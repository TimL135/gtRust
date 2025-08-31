use macroquad::prelude::*;
use rand::gen_range;

pub struct ExplosionParticle {
    pub x: f32,
    pub y: f32,
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub size: f32,
    pub life: f32,
    pub max_life: f32,
    pub color: Color,
}

impl ExplosionParticle {
    pub fn new(
        x: f32,
        y: f32,
        velocity_x: f32,
        velocity_y: f32,
        size: f32,
        life: f32,
        color: Color,
    ) -> Self {
        ExplosionParticle {
            x,
            y,
            velocity_x,
            velocity_y,
            size,
            life,
            max_life: life,
            color,
        }
    }

    pub fn update(&mut self) {
        self.x += self.velocity_x * get_frame_time();
        self.y += self.velocity_y * get_frame_time();

        // Verlangsamung durch "Reibung"
        self.velocity_x *= 0.98;
        self.velocity_y *= 0.98;

        // Lebensdauer verringern
        self.life -= get_frame_time();
    }

    pub fn draw(&self) {
        let alpha = (self.life / self.max_life).max(0.0);
        let mut color = self.color;
        color.a = alpha;

        // Partikel wird kleiner mit der Zeit
        let current_size = self.size * alpha;

        draw_circle(self.x, self.y, current_size, color);

        // Zusätzlicher Glow-Effekt
        if alpha > 0.5 {
            let glow_color = Color::new(color.r, color.g, color.b, alpha * 0.3);
            draw_circle(self.x, self.y, current_size * 2.0, glow_color);
        }
    }

    pub fn is_alive(&self) -> bool {
        self.life > 0.0
    }
}

pub struct Explosion {
    pub particles: Vec<ExplosionParticle>,
}

impl Explosion {
    pub fn new(
        x: f32,
        y: f32,
        size: f32,
        velocity_x: f32,
        velocity_y: f32,
        debris_type: u8,
    ) -> Self {
        let mut particles = Vec::new();
        let particle_count = (size * 0.5) as usize + 5; // Größere Debris = mehr Partikel

        for _ in 0..particle_count {
            let angle = gen_range(0.0, std::f32::consts::PI * 2.0);
            let speed = gen_range(size * 2.0, size * 8.0);
            let particle_velocity_x = angle.cos() * speed + velocity_x * 0.3; // Erbt etwas von der ursprünglichen Bewegung
            let particle_velocity_y = angle.sin() * speed + velocity_y * 0.3;

            particles.push(ExplosionParticle::new(
                x + gen_range(-size * 0.2, size * 0.2),
                y + gen_range(-size * 0.2, size * 0.2),
                particle_velocity_x,
                particle_velocity_y,
                gen_range(size * 0.05, size * 0.15),
                gen_range(0.5, 1.5),
                get_explosion_color(debris_type),
            ));
        }

        Explosion { particles }
    }

    pub fn update(&mut self) {
        for particle in &mut self.particles {
            particle.update();
        }

        // Entferne tote Partikel
        self.particles.retain(|p| p.is_alive());
    }

    pub fn draw(&self) {
        for particle in &self.particles {
            particle.draw();
        }
    }

    pub fn is_finished(&self) -> bool {
        self.particles.is_empty()
    }
}

fn get_explosion_color(debris_type: u8) -> Color {
    // Verschiedene Explosionsfarben basierend auf debris_type
    match debris_type {
        0 => Color::new(1.0, 0.6, 0.2, 1.0), // Orange
        1 => Color::new(1.0, 0.8, 0.3, 1.0), // Gelb-Orange
        2 => Color::new(0.9, 0.4, 0.1, 1.0), // Rot-Orange
        _ => Color::new(1.0, 0.7, 0.4, 1.0), // Helles Orange
    }
}
