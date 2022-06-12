const THRUST_SPEED: f32 = 10.0;
const THRUST_DECAY: f32 = THRUST_SPEED * 0.1;

// A burst of the rockets will create thrust,
// i.e a speed force in the point of direction.
#[derive(Clone, Copy)]
struct Thrust {
    speed: f32,
    rotation: f32,
}

impl Thrust {
    pub fn new(speed: f32, rotation: f32) -> Self {
        Self { speed, rotation }
    }
}
// The speed of the Ship is determined by all the Thrust components
// that has been generated.
pub struct Speed {
    speed: Vec<Thrust>,
}

impl Speed {
    pub fn new() -> Self {
        Self { speed: Vec::new() }
    }
    pub fn calculate_movement(&self) -> (f32, f32) {
        let mut x = 0.0;
        let mut y = 0.0;

        for thrust in &self.speed {
            x += thrust.speed * (thrust.rotation as f64).cos() as f32;
            y += thrust.speed * (thrust.rotation as f64).sin() as f32;
        }
        (x, y)
    }
    pub fn give_thrust(&mut self, rotation: f32) {
        self.add(THRUST_SPEED, rotation);
    }
    pub fn decay_thrust(&mut self) {
        self.subtract(THRUST_DECAY);
    }
    pub fn len(&mut self) -> u32 {
        let mut len = 0;
        for _ in &mut self.speed {
            len += 1;
        }
        len
    }
    pub fn exists(&mut self, rotation: f32) -> bool {
        let mut exists = false;
        for x in &mut self.speed {
            if x.rotation == rotation {
                exists = true;
            }
        }
        exists
    }
    // Add speed to an already existing Thrust vector,
    // or else add a new Thrust vector.
    pub fn add(&mut self, speed: f32, rotation: f32) {
        let mut is_new = true;
        for thrust in &mut self.speed {
            if thrust.rotation == rotation {
                thrust.speed += speed;
                is_new = false;
            }
        }
        if is_new {
            let thrust = Thrust::new(speed, rotation);
            self.speed.push(thrust);
        }
    }
    // Subtract speed from all existing Thrust vectors.
    pub fn subtract(&mut self, speed: f32) {
        for thrust in &mut self.speed {
            thrust.speed -= speed;
        }
        self.speed.retain(|&x| x.speed < 0.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new_thrust() {
        let mut speed = Speed::new();
        speed.add(THRUST_SPEED, 0.15);

        assert_eq!(speed.speed.get(0).unwrap().rotation, 0.15);
    }

    #[test]
    fn add_to_existing_thrust() {
        let mut speed = Speed::new();
        speed.add(THRUST_SPEED, 0.15);
        speed.add(THRUST_SPEED, 0.15);

        assert_eq!(speed.speed.get(0).unwrap().speed, 2.0 * THRUST_SPEED);
    }

    #[test]
    fn add_second_thrust() {
        let mut speed = Speed::new();
        speed.add(THRUST_SPEED, 0.15);
        speed.add(THRUST_SPEED, 0.25);

        assert_eq!(speed.speed.get(0).unwrap().rotation, 0.15);
        assert_eq!(speed.speed.get(0).unwrap().speed, THRUST_SPEED);
        assert_eq!(speed.speed.get(1).unwrap().rotation, 0.25);
    }
}
