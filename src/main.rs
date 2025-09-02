use macroquad::audio::load_sound;
use macroquad::prelude::*;
use std::vec::Vec;

mod bullet;
mod debris;
mod explosion;
mod floating_text;
mod help_fn;
mod music_manager;
mod player;
mod savegame;
mod settings;
mod star;

use bullet::Bullet;
use debris::Debris;
use explosion::Explosion;
use floating_text::FloatingText;
use music_manager::MusicManager;
use player::Player;
use savegame::{load_save, update_highscore};
use settings::SettingsUI;
use star::Star;

fn update_entities(
    player: &mut Player,
    bullets: &mut Vec<Bullet>,
    debris: &mut Vec<Debris>,
    floating_texts: &mut Vec<FloatingText>,
    explosions: &mut Vec<Explosion>,
    stars: &mut Vec<Star>,
    score: &mut i32,
    spawn_timer: &mut f32,
    difficulty_timer: &mut f32,
    spawn_rate: &mut f32,
) -> bool {
    // Sterne updaten
    for s in stars.iter_mut() {
        s.update();
    }
    // Spieler updaten
    player.update(bullets);

    // Schwierigkeit erhöhen über Zeit
    *difficulty_timer += get_frame_time();
    if *difficulty_timer > 10.0 {
        // Alle 10 Sekunden schwieriger
        *spawn_rate = (*spawn_rate * 0.9).max(0.2f32); // Mindestens alle 0.2 Sekunden
        *difficulty_timer = 0.0;
    }

    // Neuen Schrott spawnen
    *spawn_timer += get_frame_time();
    if *spawn_timer > *spawn_rate {
        debris.push(Debris::new());
        *spawn_timer = 0.0;
    }

    // Schrott updaten
    debris.retain_mut(|d| !d.update(explosions, floating_texts, score));

    debris.retain(|d| {
        if d.collides_with(player) {
            player.take_damage(d.damage);
            false // Element entfernen
        } else {
            true // Element behalten
        }
    });

    if player.is_destroyed() {
        return true; // Game over
    }

    // Bullets updaten
    for b in bullets.iter_mut() {
        b.update();
    }

    // Update der floating texts
    for ft in floating_texts.iter_mut() {
        ft.update();
    }
    floating_texts.retain(|ft| !ft.is_dead());

    // Bullet <-> Debris Kollision
    Bullet::handle_collisions(bullets, debris);

    // Update-Loop für Explosionen
    for explosion in explosions.iter_mut() {
        explosion.update();
    }
    // Entferne fertig animierte Explosionen
    explosions.retain(|e| !e.is_finished());

    // Offscreen-Bullets entfernen
    bullets.retain(|b| !b.is_off_screen());

    // Schrott außerhalb des Bildschirms entfernen und Score erhöhen
    debris.retain(|d| {
        if d.is_off_screen() {
            *score += 10;
            false
        } else {
            true
        }
    });

    false // Kein Game over
}

fn draw_entities(
    player: &Player,
    bullets: &Vec<Bullet>,
    debris: &Vec<Debris>,
    floating_texts: &Vec<FloatingText>,
    explosions: &Vec<Explosion>,
    stars: &Vec<Star>,
    score: i32,
    spawn_rate: f32,
) {
    // Sterne zeichnen
    for (i, s) in stars.iter().enumerate() {
        s.draw(i);
    }

    // Entitäten zeichnen
    player.draw();
    for b in bullets {
        b.draw();
    }
    for d in debris {
        d.draw();
    }
    for ft in floating_texts.iter() {
        ft.draw();
    }
    for explosion in explosions {
        explosion.draw();
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
}

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
    let mut music_manager = MusicManager::new();
    let menu_music = load_sound("assets/music/menu_track.ogg").await.unwrap();
    let gameplay_music = load_sound("assets/music/gameplay_track.ogg").await.unwrap();
    //let combat_music = load_sound("assets/music/combat_track.ogg").await.unwrap();
    music_manager.add_track("menu", menu_music);
    music_manager.add_track("game", gameplay_music);

    let mut last_width = screen_width();
    let mut last_height = screen_height();

    let mut player = Player::new();
    let mut debris: Vec<Debris> = Vec::new();
    let mut score = 0;
    let mut save = load_save();
    let mut highscore = save.highscore;
    let mut game_over = false;
    let mut spawn_timer = 0.0;
    let mut difficulty_timer = 0.0;
    let mut spawn_rate = 1.0f32; // Sekunden zwischen Spawns
    let mut bullets: Vec<Bullet> = Vec::new();
    let mut floating_texts: Vec<FloatingText> = Vec::new();
    let mut explosions: Vec<Explosion> = Vec::new();

    let mut settings_ui = SettingsUI::new();
    let mut stars: Vec<Star> = (0..100).map(|_| Star::new()).collect();

    music_manager.play("game");

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

        if !game_over {
            // Alle Entitäten updaten
            game_over = update_entities(
                &mut player,
                &mut bullets,
                &mut debris,
                &mut floating_texts,
                &mut explosions,
                &mut stars,
                &mut score,
                &mut spawn_timer,
                &mut difficulty_timer,
                &mut spawn_rate,
            );
        }

        if !game_over {
            // Alle Entitäten zeichnen
            draw_entities(
                &player,
                &bullets,
                &debris,
                &floating_texts,
                &explosions,
                &stars,
                score,
                spawn_rate,
            );
        } else {
            if music_manager.current_track() != Some(&"menu".to_string()) {
                music_manager.play("menu");
            }

            if score > highscore {
                highscore = score;
                save.highscore = highscore;
                update_highscore(highscore);
            }
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

            let highscore_text = &format!("Highscore: {}", highscore);
            let hs_size = measure_text(highscore_text, None, text_font as u16, 1.0);
            draw_text(
                highscore_text,
                screen_width() / 2.0 - hs_size.width / 2.0,
                screen_height() / 2.0 + screen_height() * 0.15,
                text_font,
                YELLOW,
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

            settings_ui.update_and_draw();
            if settings_ui.have_volume_changes {
                music_manager.refresh_settings();
                settings_ui.have_volume_changes = false;
            }

            // Neustart
            if is_key_pressed(KeyCode::R) {
                music_manager.play("game");
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
