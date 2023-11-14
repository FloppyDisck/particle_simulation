use nannou::prelude::{Point2, Pow, Vec2};

// Gravitational constant
const TOP_G: f64 = 0.00000000006674;

pub struct Particle {
    pub pos: Point2,
    pub mass: f64,
    pub net_gravitational_force: Vec2,
}

impl Particle {
    pub fn new(mass: f64, pos: Point2) -> Self {
        Self {
            pos,
            mass,
            net_gravitational_force: Vec2::default(),
        }
    }

    pub fn gravitational_pull(&mut self, part: &mut Self) {
        // Vector from self to part
        let distance_vec = part.pos - self.pos;
        let r = distance_vec.length();

        // Calculate using newtons law of universal gravitation
        let f = TOP_G * ((self.mass * part.mass) / r.pow(2) as f64);

        let force_vec = distance_vec.normalize() * f as f32;

        self.net_gravitational_force += force_vec;
        part.net_gravitational_force += force_vec * -1f32;
    }
}
