use bevy::prelude::Timer;
use bevy::utils::Duration;
use rusty_engine::prelude::*;
use std::f32::consts::{FRAC_PI_2, PI, TAU};

const SHOT_SPEED: f32 = 100.0;
const RELOAD_TIME: u64 = 150;

struct GameState {
    shot_counter: u32,
    shot_timer: Timer,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            shot_counter: 0,
            shot_timer: Timer::new(Duration::from_millis(RELOAD_TIME), false),
        }
    }
}

fn main() {
    let mut game = Game::new();

    // game setup goes here

    let player = game.add_sprite("player", SpritePreset::RacingCarBlue);
    player.translation = Vec2::new(0.0, 0.0);
    player.rotation = RIGHT;
    player.scale = 0.5;
    player.collision = true;

    game.add_logic(game_logic);
    game.run(GameState::default());
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // game logic goes here
    //
    // Update the shot reload timer.
    game_state.shot_timer.tick(engine.delta);

    // Get hold of the Player info.
    let player = engine.sprites.get_mut("player").unwrap();
    let player_x = player.translation.x;
    let player_rotation = player.rotation;
    let mut shoot = false;

    // Keyboard handling
    if engine.keyboard_state.pressed(KeyCode::Space) && game_state.shot_timer.finished() {
        shoot = true;
        game_state.shot_timer.reset();
    } else if engine.keyboard_state.pressed(KeyCode::Up) {
        // Deal with positive rotation overflow
        player.rotation = (player.rotation + 0.05) % TAU;
    } else if engine.keyboard_state.pressed(KeyCode::Down) {
        player.rotation -= 0.05;
        // Deal with negative rotation overflow
        if player.rotation < 0.0 {
            player.rotation = TAU - player.rotation
        };
    }

    // Generate a shot
    if shoot {
        game_state.shot_counter += 1;
        let sprite = engine.add_sprite(
            format!("shot{}", game_state.shot_counter),
            SpritePreset::RollingBallRed,
        );
        sprite.scale = 0.1;
        sprite.translation.x = player_x;
        sprite.rotation = player_rotation;
        sprite.collision = true;
    }

    // Move the shots
    for sprite in engine.sprites.values_mut() {
        // bounds check
        if sprite.translation.y > 360.0
            || sprite.translation.y < -360.0
            || sprite.translation.x > 800.0
            || sprite.translation.x < -800.0
        {
            // FIXME remove the sprite
            continue;
        }
        if sprite.label.starts_with("shot") {
            sprite.translation.x +=
                SHOT_SPEED * engine.delta_f32 * (sprite.rotation as f64).cos() as f32;
            sprite.translation.y +=
                SHOT_SPEED * engine.delta_f32 * (sprite.rotation as f64).sin() as f32;
        }
    }
}
