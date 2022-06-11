use rusty_engine::prelude::*;

const SHOT_SPEED: f32 = 100.0;

struct GameState {
    shot_counter: u32,
}

impl Default for GameState {
    fn default() -> Self {
        Self { shot_counter: 0 }
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

    let mut shoot = false;

    // Shot fired?
    if engine.keyboard_state.pressed(KeyCode::Space) {
        shoot = true;
    }

    // Generate a shot
    if shoot {
        let player = engine.sprites.get_mut("player").unwrap();
        game_state.shot_counter += 1;
        let sprite = engine.add_sprite(
            format!("shot{}", game_state.shot_counter),
            SpritePreset::RollingBallRed,
        );
        sprite.scale = 0.1;
        sprite.translation = player.translation;
        sprite.rotation = player.rotation;
        sprite.collision = true;
    }

    // Move the shots
    for sprite in engine.sprites.values_mut() {
        if sprite.label.starts_with("shot") {
            sprite.translation.x += SHOT_SPEED * engine.delta_f32;
        }
    }
}
