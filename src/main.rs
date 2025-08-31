use macroquad::prelude::*;
use std::vec::Vec;

mod bullet;
mod debris;
mod explosion;
mod floating_text;
mod help_fn;
mod player;
mod star;

use bullet::Bullet;
use debris::Debris;
use explosion::Explosion;
use floating_text::FloatingText;
use player::Player;
use star::Star;

fn window_conf() -> Conf {
    Conf {
        window_title: "gtRust".to_owned(),
        window_width: 1920,
        window_height: 1080,
        fullscreen: false,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut last_width = screen_width();
    let mut last_height = screen_height();

    let mut player = Player::new();
    let mut debris: Vec<Debris> = Vec::new();
    let mut score = 0;
    let mut game_over = false;
    let mut spawn_timer = 0.0;
    let mut difficulty_timer = 0.0;
    let mut spawn_rate = 1.0f32; // Sekunden zwischen Spawns
    let mut bullets: Vec<Bullet> = Vec::new();
    let mut floating_texts: Vec<FloatingText> = Vec::new();
    let mut explosions: Vec<Explosion> = Vec::new();

    let mut stars: Vec<Star> = (0..100).map(|_| Star::new()).collect();

    loop {
        let current_width = screen_width();
        let current_height = screen_height();

        if current_width != last_width || current_height != last_height {
            let star_count = ((current_width * current_height) / 8000.0) as usize;

            stars.clear(); // alle alten Sterne löschen
            stars.extend((0..star_count).map(|_| Star::new())); // komplett neu

            last_width = current_width;
            last_height = current_height;

            game_over = true;
        }

        for s in stars.iter_mut() {
            s.update();
        }
        for (i, s) in stars.iter().enumerate() {
            s.draw(i);
        }
        if !game_over {
            // Spieler updaten
            player.update(&mut bullets);

            // Schwierigkeit erhöhen über Zeit
            difficulty_timer += get_frame_time();
            if difficulty_timer > 10.0 {
                // Alle 10 Sekunden schwieriger
                spawn_rate = (spawn_rate * 0.9).max(0.2f32); // Mindestens alle 0.2 Sekunden
                difficulty_timer = 0.0;
            }

            // Neuen Schrott spawnen
            spawn_timer += get_frame_time();
            if spawn_timer > spawn_rate {
                debris.push(Debris::new());
                spawn_timer = 0.0;
            }

            // Schrott updaten
            debris.retain_mut(|d| !d.update(&mut explosions, &mut floating_texts, &mut score));

            debris.retain(|d| {
                if d.collides_with(&player) {
                    player.take_damage(d.damage);
                    false // Element entfernen
                } else {
                    true // Element behalten
                }
            });

            if player.is_destroyed() {
                game_over = true;
            }

            // Bullets updaten
            for b in &mut bullets {
                b.update();
            }

            // Update und Draw der floating texts
            for ft in floating_texts.iter_mut() {
                ft.update();
            }
            floating_texts.retain(|ft| !ft.is_dead());

            // Bullet <-> Debris Kollision
            Bullet::handle_collisions(&mut bullets, &mut debris);

            // Update-Loop
            for explosion in explosions.iter_mut() {
                explosion.update();
            }
            // Entferne fertig animierte Explosionen
            explosions.retain(|e| !e.is_finished());

            // Draw-Loop
            for explosion in &explosions {
                explosion.draw();
            }

            // Offscreen-Bullets entfernen
            bullets.retain(|b| !b.is_off_screen());

            // Schrott außerhalb des Bildschirms entfernen und Score erhöhen
            debris.retain(|d| {
                if d.is_off_screen() {
                    score += 10;
                    false
                } else {
                    true
                }
            });
        }

        if !game_over {
            player.draw();
            for b in &bullets {
                b.draw();
            }
            for d in &debris {
                d.draw();
            }

            for ft in floating_texts.iter() {
                ft.draw();
            }

            // UI skaliert mit Bildschirmgröße
            let font_size = screen_height() * 0.04; // 4% der Bildschirmhöhe
            let small_font = screen_height() * 0.025; // 2.5% der Bildschirmhöhe

            // Score anzeigen
            draw_text(
                &format!("Score: {}", score),
                screen_width() * 0.02,
                screen_height() * 0.06,
                font_size,
                WHITE,
            );

            // Spawn-Rate anzeigen
            draw_text(
                &format!("Spawn Rate: {:.1}s", spawn_rate),
                screen_width() * 0.02,
                screen_height() * 0.12,
                small_font,
                GRAY,
            );

            // Steuerung
            draw_text(
                "WASD oder Pfeiltasten zum Bewegen | ESC = Beenden",
                screen_width() * 0.02,
                screen_height() - screen_height() * 0.03,
                small_font,
                GRAY,
            );
        } else {
            // Game Over Screen
            let title_font = screen_height() * 0.08;
            let text_font = screen_height() * 0.04;
            let small_font = screen_height() * 0.025;

            let text = "GAME OVER!";
            let text_size = measure_text(text, None, title_font as u16, 1.0);
            draw_text(
                text,
                screen_width() / 2.0 - text_size.width / 2.0,
                screen_height() / 2.0 - screen_height() * 0.1,
                title_font,
                RED,
            );

            let score_text = &format!("Final Score: {}", score);
            let score_size = measure_text(score_text, None, text_font as u16, 1.0);
            draw_text(
                score_text,
                screen_width() / 2.0 - score_size.width / 2.0,
                screen_height() / 2.0,
                text_font,
                WHITE,
            );

            let restart_text = "Drücke R zum Neustarten | ESC zum Beenden";
            let restart_size = measure_text(restart_text, None, small_font as u16, 1.0);
            draw_text(
                restart_text,
                screen_width() / 2.0 - restart_size.width / 2.0,
                screen_height() / 2.0 + screen_height() * 0.08,
                small_font,
                GRAY,
            );

            // Neustart
            if is_key_pressed(KeyCode::R) {
                player = Player::new();
                debris.clear();
                bullets.clear();
                floating_texts.clear();
                score = 0;
                game_over = false;
                spawn_timer = 0.0;
                difficulty_timer = 0.0;
                spawn_rate = 1.0;
            }
        }

        // ESC zum Beenden
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await
    }
}
