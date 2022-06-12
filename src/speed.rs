const THRUST_SPEED: f32 = 10.0;

// A burst of the rockets will create thrust,
// i.e a speed force in the point of direction.
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
    //  or else add a new Thrust vector.
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
