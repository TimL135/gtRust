use macroquad::prelude::*;
use std::vec::Vec;

mod bullet;
mod debris;
mod explosion;
mod floating_text;
mod fps;
mod help_fn;
mod items;
mod music_manager;
mod player;
mod savegame;
mod settings;
mod skill_tree;
mod star;

use bullet::Bullet;
use debris::Debris;
use explosion::Explosion;
use floating_text::FloatingText;
use fps::FpsCounter;
use items::{ItemManager, ItemType};
use music_manager::MusicManager;
use player::Player;
use savegame::{load_save, update_highscore};
use settings::SettingsUI;
use skill_tree::SkillTreeManager;
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
    fps_counter: &mut FpsCounter,
    item_manager: &mut ItemManager,
) -> bool {
    let dt = get_frame_time();

    // Sterne updaten
    for s in stars.iter_mut() {
        s.update();
    }

    // Items updaten (mit Spieler für Magnet-Effekt)
    item_manager.update(dt, player);

    // Item-Pickups prüfen
    let picked_up_items = item_manager.check_pickups(player, floating_texts);

    // Handle item pickups for skill effects
    if !picked_up_items.is_empty() {
        player.on_item_pickup();
    }

    // Spieler updaten
    player.update(bullets);

    // Schwierigkeit erhöhen über Zeit
    *difficulty_timer += dt;
    if *difficulty_timer > 10.0 {
        // Alle 10 Sekunden schwieriger
        *spawn_rate = (*spawn_rate * 0.9).max(0.2f32); // Mindestens alle 0.2 Sekunden
        *difficulty_timer = 0.0;
    }

    // Effekt-basierte Spawn-Rate Modifikation
    let mut effective_spawn_rate = *spawn_rate;

    // SlowMotion und TimeFreeze beeinflussen Gegner-Spawn
    if player.has_effect(&ItemType::SlowMotion) {
        effective_spawn_rate *= 1.5; // Langsameres Spawning
    }
    if player.has_effect(&ItemType::TimeFreeze) {
        effective_spawn_rate *= 3.0; // Sehr langsameres Spawning
    }

    // Neuen Schrott spawnen
    *spawn_timer += dt;
    if *spawn_timer > effective_spawn_rate {
        debris.push(Debris::new());
        *spawn_timer = 0.0;
    }

    // Schrott updaten mit Effekt-Modifikatoren
    let mut debris_speed_multiplier = 1.0;

    if player.has_effect(&ItemType::SlowMotion) {
        debris_speed_multiplier = 0.3; // 30% Geschwindigkeit
    }
    if player.has_effect(&ItemType::TimeFreeze) {
        debris_speed_multiplier = 0.0; // Komplett eingefroren
    }

    // Debris-Geschwindigkeit modifizieren

    for debris_piece in debris.iter_mut() {
        debris_piece.speed_multiplier = debris_speed_multiplier;
    }

    debris.retain_mut(|d| !d.update(explosions, floating_texts, score, player.points_multiplier));

    // Kollision mit Spieler (außer bei PhaseShift)
    if !player.can_phase_through {
        debris.retain(|d| {
            if d.collides_with(player) {
                player.take_damage(d.damage);
                false // Element entfernen
            } else {
                true // Element behalten
            }
        });
    }

    if player.is_destroyed() {
        return true; // Game over
    }

    // Bullets updaten
    for b in bullets.iter_mut() {
        b.update(&debris);
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
    let base_points = 10;
    debris.retain(|d| {
        if d.is_off_screen() {
            let points = (base_points as f32 * player.points_multiplier) as i32;
            *score += points;

            // Floating text für Bonus-Punkte
            if player.points_multiplier > 1.0 {
                floating_texts.push(FloatingText::new_with_text(
                    d.x,
                    d.y,
                    format!("+{} ({}x)", points, player.points_multiplier as i32),
                    Color::new(1.0, 1.0, 0.0, 1.0),
                ));
            }
            false
        } else {
            true
        }
    });

    fps_counter.update();

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
    fps_counter: &FpsCounter,
    item_manager: &ItemManager,
) {
    // Sterne zeichnen
    for (i, s) in stars.iter().enumerate() {
        s.draw(i);
    }

    // Items zeichnen
    item_manager.draw();

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

    // Score anzeigen (mit Multiplikator)
    let score_text = if player.points_multiplier > 1.0 {
        format!("Score: {} ({}x)", score, player.points_multiplier as i32)
    } else {
        format!("Score: {}", score)
    };

    draw_text(
        &score_text,
        screen_width() * 0.02,
        screen_height() * 0.06,
        font_size,
        if player.points_multiplier > 1.0 {
            YELLOW
        } else {
            WHITE
        },
    );

    // Spawn-Rate anzeigen
    draw_text(
        &format!("Spawn Rate: {:.1}s", spawn_rate),
        screen_width() * 0.02,
        screen_height() * 0.12,
        small_font,
        GRAY,
    );

    // Aktive Effekte in der oberen rechten Ecke anzeigen
    let mut effect_y = screen_height() * 0.06;
    for effect in &player.active_effects {
        let effect_name = match effect.effect_type {
            ItemType::Shield => "SHIELD",
            ItemType::SpeedBoost => "SPEED",
            ItemType::SlowMotion => "SLOW-MO",
            ItemType::Magnet => "MAGNET",
            ItemType::PhaseShift => "PHASE",
            ItemType::TimeFreeze => "FREEZE",
            ItemType::DoublePoints => "2X POINTS",
            ItemType::Overdrive => "OVERDRIVE",
        };

        let effect_text = format!("{}: {:.1}s", effect_name, effect.remaining_time);
        let text_width = measure_text(&effect_text, None, small_font as u16, 1.0).width;

        draw_text(
            &effect_text,
            screen_width() - text_width - screen_width() * 0.02,
            effect_y,
            small_font,
            YELLOW,
        );
        effect_y += small_font * 1.2;
    }

    // Steuerung
    draw_text(
        "WASD or arrow keys to move | SPACE = Shoot | ESC = Quit",
        screen_width() * 0.02,
        screen_height() - screen_height() * 0.03,
        small_font,
        GRAY,
    );
    fps_counter.draw();
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
    let mut music_manager = MusicManager::new().await;

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
    let mut fps_counter = FpsCounter::new();
    let mut stars: Vec<Star> = (0..100).map(|_| Star::new()).collect();
    let mut item_manager = ItemManager::new();
    let mut skill_tree_manager = SkillTreeManager::new();
    let mut show_skill_tree = false;

    music_manager.play("gameplay");

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
                &mut fps_counter,
                &mut item_manager,
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
                &fps_counter,
                &item_manager,
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

            // Check if player earned skill points
            let skill_points_earned =
                SkillTreeManager::calculate_skill_points_from_score(highscore);
            if skill_points_earned > skill_tree_manager.total_skill_points_earned {
                let new_points = skill_points_earned - skill_tree_manager.total_skill_points_earned;
                for _ in 0..new_points {
                    skill_tree_manager.earn_skill_point();
                }
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

            // Skill Points anzeigen
            let skill_points_text = &format!(
                "Available Skill Points: {}",
                skill_tree_manager.available_skill_points
            );
            let sp_size = measure_text(skill_points_text, None, text_font as u16, 1.0);
            draw_text(
                skill_points_text,
                screen_width() / 2.0 - sp_size.width / 2.0,
                screen_height() / 2.0 + screen_height() * 0.05,
                text_font,
                GREEN,
            );

            let restart_text = "Press R to Restart | Press T for Skill Tree | ESC to Quit";
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

            // Skill Tree anzeigen/verstecken
            if is_key_pressed(KeyCode::T) {
                show_skill_tree = !show_skill_tree;
            }

            if show_skill_tree {
                skill_tree_manager.draw_and_handle_input();
            }

            // Neustart
            if is_key_pressed(KeyCode::R) {
                music_manager.play("gameplay");
                player = Player::new();
                // Apply skills to new player
                skill_tree_manager.apply_to_player(&mut player);

                debris.clear();
                bullets.clear();
                floating_texts.clear();
                explosions.clear();
                item_manager = ItemManager::new();
                score = 0;
                game_over = false;
                spawn_timer = 0.0;
                difficulty_timer = 0.0;
                spawn_rate = 1.0;
                show_skill_tree = false;
            }
        }

        // ESC zum Beenden
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await
    }
}
