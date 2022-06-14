# rusty-asteroids: experiments with the rusty-engine

After having finished the first `road-race` exercise
in the `Ultimate Rust 2` course, I wanted to try and
see if I could write a simple `Asteroids` game.

## Install and run

After cloning, download the assets:

    cd rusty-asteroids
    curl -L https://github.com/CleanCut/rusty_engine/archive/refs/heads/main.tar.gz | tar -zxv --strip-components=1 rusty_engine-main/assets

then compile and run:

    cargo run

## Some notes

First I started to think about shooting projectiles from the Ship.
From the `Rusty Engine` I could get the rotation, in Radians, of
the Player. Assuming the projectiles travels with a certain speed
I should be able to calculate the movement in (x,y).

![](shoot-small.png)

The code became very simple, the `SHOT_SPEED` times a scaling factor
times the cos/sin of the rotation angle got me the additional (x,y)
to be added to the current position:

``` rust
    // Move the shots
    for sprite in engine.sprites.values_mut() {
        if sprite.label.starts_with("shot") {
            sprite.translation.x +=
                SHOT_SPEED * engine.delta_f32 * (sprite.rotation as f64).cos() as f32;
            sprite.translation.y +=
                SHOT_SPEED * engine.delta_f32 * (sprite.rotation as f64).sin() as f32;
        }
    }
```

Next, I wanted to move the ship like in space. I imagined
that movements in space behave somewhat similar to a boat,
i.e you can't do sharp turns and have to rotate in combination
with using the rockets to control the direction you move in.

Assuming the ship moves in one direction with speed S1,
then that translates to some movements in (x,y) for each 
frame in the game. If the ship rotates and give some thrust
to the rockets, a new speed component (vector) adds to the
existing speed vector. The sum of those vectors forms the
new speed vector that represent the new movements (x,y)
to be performed in the succeeding frames.

![](speed-small.png)

The code became almost identical; the current speed of the
ship speed as (x,y) is kept in the game state. Then we just
add the new speed vector according to our rotation angle.
Note that if no new thrust was given, we then just continue
in the same speed as before. 

``` rust
    // Give thrust
    if give_thrust {
        game_state.speed.x += THRUST_SPEED * (player_rotation as f64).cos() as f32;
        game_state.speed.y += THRUST_SPEED * (player_rotation as f64).sin() as f32;
    }
    // Move the player
    player.translation.x += game_state.speed.x * engine.delta_f32;
    player.translation.y += game_state.speed.y * engine.delta_f32;
```

