use bevy::prelude::Timer;
use bevy::utils::Duration;
use rusty_engine::prelude::*;
use std::f32::consts::TAU;
const SHOT_SPEED: f32 = 200.0;
const RELOAD_TIME: u64 = 150;
const THRUST_TIME: u64 = 200;
const THRUST_SPEED: f32 = 10.0;

struct GameState {
    shot_counter: u32,
    shot_timer: Timer,
    thrust_timer: Timer,
    speed: Vec2,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            shot_counter: 0,
            shot_timer: Timer::new(Duration::from_millis(RELOAD_TIME), false),
            thrust_timer: Timer::new(Duration::from_millis(THRUST_TIME), false),
            speed: Vec2::new(0.0, 0.0),
        }
    }
}

fn main() {
    let mut game = Game::new();

    // game setup goes here

    //let player = game.add_sprite("player", "kenny/Retina/ship_A.png");
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
    // Update the timers.
    game_state.shot_timer.tick(engine.delta);
    game_state.thrust_timer.tick(engine.delta);

    // Get hold of the Player info.
    let player = engine.sprites.get_mut("player").unwrap();
    let player_x = player.translation.x;
    let player_y = player.translation.y;
    let player_rotation = player.rotation;
    let mut shoot = false;
    let mut give_thrust = false;

    // Keyboard handling
    if engine.keyboard_state.pressed(KeyCode::Space) && game_state.shot_timer.finished() {
        shoot = true;
        game_state.shot_timer.reset();
    } else if engine.keyboard_state.pressed(KeyCode::Left) {
        // Deal with positive rotation overflow
        player.rotation = (player.rotation + 0.05) % TAU;
    } else if engine.keyboard_state.pressed(KeyCode::Right) {
        player.rotation -= 0.05;
        // Deal with negative rotation overflow
        if player.rotation < 0.0 {
            player.rotation = TAU - player.rotation
        };
    } else if engine.keyboard_state.pressed(KeyCode::Up) && game_state.thrust_timer.finished() {
        give_thrust = true;
        game_state.thrust_timer.reset();
    }

    // Give thrust
    if give_thrust {
        game_state.speed.x += THRUST_SPEED * (player_rotation as f64).cos() as f32;
        game_state.speed.y += THRUST_SPEED * (player_rotation as f64).sin() as f32;
    }
    // Move the player
    player.translation.x += game_state.speed.x * engine.delta_f32;
    player.translation.y += game_state.speed.y * engine.delta_f32;

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

    // Generate a new shot
    if shoot {
        game_state.shot_counter += 1;
        let sprite = engine.add_sprite(
            format!("shot{}", game_state.shot_counter),
            SpritePreset::RollingBallRed,
        );
        sprite.scale = 0.1;
        sprite.translation.x = player_x;
        sprite.translation.y = player_y;
        sprite.rotation = player_rotation;
        sprite.collision = true;
        engine.audio_manager.play_sfx(SfxPreset::Impact1, 0.4);
    }
}
