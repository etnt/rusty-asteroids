use bevy::prelude::Timer;
use bevy::utils::Duration;
use rand::{thread_rng, Rng};
use rusty_engine::prelude::*;
use std::{f32::consts::TAU};
const SHOT_SPEED: f32 = 200.0;
const RELOAD_TIME: u64 = 150;
const THRUST_TIME: u64 = 200;
const SEC_IN_MSEC: u64 = 1000;
const THRUST_SPEED: f32 = 10.0;
const METEOROID_SPEED: f32 = 50.0;

type Seconds = u64;

struct GameState {
    stop: bool,
    high_score: Seconds,
    start_time: Seconds,
    meteoroids: Vec<String>,
    shot_counter: u32,
    shot_timer: Timer,
    thrust_timer: Timer,
    stop_timer: Timer,
    speed: Vec2,
    sprites_to_delete: Vec<String>,
    pub max_x: f32,
    pub min_x: f32,
    pub max_y: f32,
    pub min_y: f32,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            stop: false,
            high_score: 0,
            start_time: 0,
            meteoroids: Vec::new(),
            shot_counter: 0,
            shot_timer: Timer::new(Duration::from_millis(RELOAD_TIME), false),
            thrust_timer: Timer::new(Duration::from_millis(THRUST_TIME), false),
            stop_timer: Timer::new(Duration::from_millis(SEC_IN_MSEC), false),
            speed: Vec2::new(0.0, 0.0),
            sprites_to_delete: Vec::new(),
            max_x: 720.0,
            min_x: -720.0,
            max_y: 360.0,
            min_y: -360.0,
        }
    }
}


fn main() {
    let mut game = Game::new();

    // game setup goes here
    let width: f32 = 1280.0;    // FIXME read Width x Height from somewhere
    let height: f32 = 720.0;

    let mut game_state = GameState {
        max_x: width / 2.0,
        min_x: - width / 2.0,
        max_y: height / 2.0,
        min_y: - height / 2.0,
        ..Default::default()
    };

    // Stop time
    let stop_time = game.add_text("stop_time", "Time: 00:00");
    stop_time.translation = Vec2::new(550.0, 320.0);
    // Best Time
    let best_time = game.add_text("best_time", "Best Time: 00:00");
    best_time.translation = Vec2::new(-510.0, 320.0);

    let player = game.add_sprite("player", SpritePreset::RacingCarBlue);
    player.translation = Vec2::new(0.0, 0.0);
    player.rotation = RIGHT;
    player.scale = 0.5;
    player.collision = true;

    // Create all the Meteoroids
    let meteoroids = meteoroids();
    for (i, meteoroid) in meteoroids.into_iter().enumerate() {
        let sprite = game.add_sprite(format!("meteoroid{}", i), meteoroid);
        reset_meteoroid(sprite, &game_state);
        // Save the name of the Meteoroid so that we can
        // keep track of when all of them are gone.
        game_state.meteoroids.push(sprite.label.clone());
    }

    game.add_logic(game_logic);
    game.run(game_state);
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // game logic goes here

    if game_state.stop {
        // Handle potential restart of the game
        if engine.keyboard_state.pressed(KeyCode::R) {
            engine.texts.remove("game_over");
            engine.texts.remove("restart");
            reset_player_position(engine);
            // reset player speed
            game_state.speed = Vec2::new(0.0, 0.0);
            reset_meteoroids(engine, game_state);
            reset_shots(engine);
            reset_stop_time(engine, game_state);

            // Generate new Meteoroids
            let meteoroids = meteoroids();
            for (i, meteoroid) in meteoroids.into_iter().enumerate() {
                let sprite = engine.add_sprite(format!("meteoroid{}", i), meteoroid);
                reset_meteoroid(sprite, game_state);
                // Save the name of the Meteoroid so that we can
                // keep track of when all of them are gone.
                game_state.meteoroids.push(sprite.label.clone());
            }
            game_state.shot_counter = 0;
            game_state.shot_timer.reset();
            game_state.thrust_timer.reset();
            game_state.stop_timer.reset();
            game_state.start_time = engine.time_since_startup.as_secs();
            game_state.stop = false;
        } else if engine.keyboard_state.pressed(KeyCode::Q) {
            engine.should_exit = true;
        }
        return;
    }

    let mut game_over = false;
    let max_x = game_state.max_x;
    let min_x = game_state.min_x;
    let max_y = game_state.max_y;
    let min_y = game_state.min_y;

    // Update the timers.
    game_state.shot_timer.tick(engine.delta);
    game_state.thrust_timer.tick(engine.delta);
    game_state.stop_timer.tick(engine.delta);

    // Get hold of the Player info.
    let player = engine.sprites.get_mut("player").unwrap();
    let player_x = player.translation.x;
    let player_y = player.translation.y;
    let player_rotation = player.rotation;
    let mut shoot = false;
    let mut give_thrust = false;

    // Update the Timer
    if game_state.stop_timer.finished() {
        let stop_time = engine.texts.get_mut("stop_time").unwrap();
        let running_time = engine.time_since_startup.as_secs() - game_state.start_time;
        let min = running_time / 60;
        let secs = running_time % 60;
        stop_time.value = format!("Time: {:0>2}:{:0>2}", min, secs);
        game_state.stop_timer.reset();
    }

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
        engine.audio_manager.play_sfx(SfxPreset::Forcefield2, 0.2);
        game_state.speed.x += THRUST_SPEED * (player_rotation as f64).cos() as f32;
        game_state.speed.y += THRUST_SPEED * (player_rotation as f64).sin() as f32;
    }

    // Move the player
    player.translation.x += game_state.speed.x * engine.delta_f32;
    player.translation.y += game_state.speed.y * engine.delta_f32;

    // Move the shots and the meteoroids
    for sprite in engine.sprites.values_mut() {
        // bounds check
        if sprite.translation.y > 360.0
            || sprite.translation.y < -360.0
            || sprite.translation.x > 800.0
            || sprite.translation.x < -800.0
        {
            // Explanation found in the `Car Shoot` scenario description:
            //
            // We can't modify a hash map of sprites while we're
            // looping through its values, so let's create an
            // empty vector of strings and fill it with labels
            // of sprites that we want to delete. Once we're
            // done examining the hash map, we can loop through
            // the vector of labels and remove those hash map entries.
            if sprite.label.starts_with("shot") {
                game_state.sprites_to_delete.push(sprite.label.clone());
                continue;
            }
        }
        if sprite.label.starts_with("shot") {
            sprite.translation.x +=
                SHOT_SPEED * engine.delta_f32 * (sprite.rotation as f64).cos() as f32;
            sprite.translation.y +=
                SHOT_SPEED * engine.delta_f32 * (sprite.rotation as f64).sin() as f32;
        }
        if sprite.label.starts_with("meteoroid") {
            let speed = METEOROID_SPEED * thread_rng().gen_range(0.5..1.5) as f32;
            sprite.translation.x += speed * engine.delta_f32 * (sprite.rotation as f64).cos() as f32;
            sprite.translation.y += speed * engine.delta_f32 * (sprite.rotation as f64).cos() as f32;
            // bounds check, out on left side -> new random position
            if sprite.translation.x < min_x || sprite.translation.x > max_x || sprite.translation.y < min_y || sprite.translation.y > max_y {
                let mut x = 0.0;
                let mut y = 0.0;
                'random: loop {
                    x = thread_rng().gen_range(game_state.min_x..game_state.max_x);
                    y = thread_rng().gen_range(game_state.min_y..game_state.max_y);
                    // avoid starting too close to the player
                    if (x > player_x + 30.0 || x < 30.0 - player_x) && (y > player_y + 30.0 || y < player_y - 30.0) {
                        break 'random;
                    }
                }
                sprite.translation.x = x;
                sprite.translation.y = y;
            }
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
        sprite.rotation = player_rotation;
        sprite.translation.x = player_x;
        sprite.translation.y = player_y;
        sprite.collision = true;
        engine.audio_manager.play_sfx(SfxPreset::Impact1, 0.4);
    }

    // deal with collisions
    for event in engine.collision_events.drain(..) {
        // We only care about the start of collisions, not the ending of them.
        if event.state.is_end() {
            continue;
        }
        if event.pair.one_starts_with("player") {
            if event.pair.either_starts_with("shot") {
                continue;
            } else if event.pair.either_starts_with("meteoroid") {
                engine.audio_manager.play_sfx(SfxPreset::Impact3, 0.5);
                game_over = true;
                break;
            }
        } else if event.pair.one_starts_with("shot") && event.pair.one_starts_with("meteoroid"){
            engine.audio_manager.play_sfx(SfxPreset::Impact2, 0.4);

            // Remove any destroyed meteoroid from our "alive" list.
            game_state.meteoroids.retain(|x| *x != event.pair.0);
            game_state.meteoroids.retain(|x| *x != event.pair.1);

            // Push the Sprites to be removed later.
            game_state.sprites_to_delete.push(event.pair.0.clone());
            game_state.sprites_to_delete.push(event.pair.1.clone());
        }
    }

    // Remove the sprites.
    for sprite_to_delete in &game_state.sprites_to_delete {
        engine.sprites.remove(sprite_to_delete);
    }
    game_state.sprites_to_delete.drain(..);

    // check for lost/won game
    if game_over || game_state.meteoroids.is_empty() {
        game_state.stop = true;
        if game_over {
            let game_over_text = engine.add_text("game_over", "You Lost!");
            game_over_text.font_size = 128.0;
            engine.audio_manager.play_sfx(SfxPreset::Jingle3, 0.5);
        } else if game_state.meteoroids.is_empty() {
            let you_won_text = engine.add_text("game_over", "You Won!");
            you_won_text.font_size = 128.0;
            engine.audio_manager.play_sfx(SfxPreset::Jingle1, 0.5);

            // Check for an improved finish time
            let running_time = engine.time_since_startup.as_secs() - game_state.start_time;
            println!("High Score: {}  ,  Running Time: {}", game_state.high_score, running_time);
            if (game_state.high_score == 0) || (running_time < game_state.high_score) {
                game_state.high_score = running_time;
                let min = running_time / 60;
                let secs = running_time % 60;
                let best_time = engine.texts.get_mut("best_time").unwrap();
                best_time.value = format!("Best Time: {:0>2}:{:0>2}", min, secs);
            }
        }
        let restart_text = engine.add_text("restart", "press R to restart\npress Q or Esc to quit");
        restart_text.translation = Vec2::new(-30.0, -80.0);
    };

}


fn reset_meteoroid(m: &mut Sprite, game_state: &GameState) {
    m.layer = 5.0;
    m.collision = true;
    m.scale = thread_rng().gen_range(0.1..1.0);
    m.rotation = thread_rng().gen_range(0.0..TAU);

    // Avoid starting too close to the player, so let's
    // continue to generate a random position until it
    // is enough far away from the Player.
    let mut x = 0.0;
    let mut y = 0.0;
    'random: loop {
        x = thread_rng().gen_range(game_state.min_x..game_state.max_x);
        y = thread_rng().gen_range(game_state.min_y..game_state.max_y);
        if !(-30.0..30.0).contains(&x) && !(-30.0..30.0).contains(&y) {
            break 'random;
        }
    }
    m.translation.x = x;
    m.translation.y = y;
}

fn meteoroids() -> Vec<SpritePreset> {
    let meteoroids = vec![
        SpritePreset::RollingBlockCorner,
        SpritePreset::RollingBlockNarrow,
        SpritePreset::RollingBlockSmall,
        SpritePreset::RollingBlockSquare,
        SpritePreset::RollingBlockCorner,
        SpritePreset::RollingBlockNarrow,
        SpritePreset::RollingBlockSmall,
        SpritePreset::RollingBlockSquare,
    ];
    meteoroids
}

fn reset_player_position(engine: &mut Engine) {
    let player = engine.sprites.get_mut("player").unwrap();
    player.translation.x = 0.0;
    player.translation.y = 0.0;
    player.rotation = RIGHT;
}

fn reset_meteoroids(engine: &mut Engine, game_state: &mut GameState) {
    for meteoroid in &game_state.meteoroids {
        engine.sprites.remove(meteoroid);
    }
    game_state.meteoroids.drain(..);
}

fn reset_shots(engine: &mut Engine) {
    let mut to_delete: Vec<String> = Vec::new();
    for sprite in engine.sprites.values_mut() {
        if sprite.label.starts_with("shot") {
                to_delete.push(sprite.label.clone());
        }
    }
    for s in &to_delete {
        engine.sprites.remove(s);
    }
}

fn reset_stop_time(engine: &mut Engine, game_state: &mut GameState) {
    let stop_time = engine.texts.get_mut("stop_time").unwrap();
    stop_time.value = format!("Time: {:0>2}:{:0>2}", 0, 0);
    game_state.stop_timer.reset();
}
