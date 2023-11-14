use crate::particle::Particle;
use nannou::geom::Vec2;
use nannou::prelude::{Point2, BLACK, WHITE};
use nannou::{App, Frame};
use rand::rngs::ThreadRng;
use rand::Rng;
use std::time::Duration;

const GALACTIC_ZOOM: f32 = 100.0;

pub struct Particles {
    pub particles: Vec<Particle>,
    pub window: Point2,
}

impl Particles {
    pub fn new(
        particle_qty: usize,
        mass_clamp: (f64, f64),
        size: Point2,
        rng: &mut ThreadRng,
    ) -> Self {
        let mut particles = vec![];

        for _ in 0..particle_qty {
            particles.push(Particle::new(
                rng.gen_range(mass_clamp.0..=mass_clamp.1),
                Point2::new(
                    rng.gen_range((-1.0 * size.x * GALACTIC_ZOOM)..(size.x * GALACTIC_ZOOM)),
                    rng.gen_range((-1.0 * size.y * GALACTIC_ZOOM)..size.y * GALACTIC_ZOOM),
                ),
            ))
        }

        Self {
            particles,
            window: size,
        }
    }

    pub fn tick(&mut self, elapsed: Duration) {
        // for p in self.particles.iter_mut() {
        //     p.net_gravitational_force = Vec2::default();
        // }

        for i in (0..self.particles.len() - 1) {
            for j in (i + 1..self.particles.len()) {
                let mut p1 = self.particles.remove(i);
                let mut p2 = self.particles.get_mut(j - 1).unwrap();

                p1.gravitational_pull(&mut p2);
                self.particles.insert(i, p1);
            }
            let mut p1 = self.particles.get_mut(i).unwrap();

            p1.net_gravitational_force *= elapsed.as_secs_f32();
            // println!("Pre: {}", p1.net_gravitational_force.length());
            // p1.net_gravitational_force = p1.net_gravitational_force.clamp_length(-1.0, 1.0);
            // println!("Pos: {}", p1.net_gravitational_force.length());

            p1.pos = p1.pos + p1.net_gravitational_force;

            println!("{}", p1.pos);
            // if p1.pos.x > self.window.x {
            //     p1.pos.x = self.window.x * -1.0;
            // } else if p1.pos.x < self.window.x * -1.0 {
            //     p1.pos.x = self.window.x
            // }
            //
            // if p1.pos.y > self.window.y {
            //     p1.pos.y = self.window.y * -1.0;
            // } else if p1.pos.y < self.window.y * -1.0 {
            //     p1.pos.y = self.window.y
            // }
        }
    }

    pub fn view(&self, app: &App, frame: Frame) {
        let draw = app.draw();
        draw.background().color(BLACK);

        for p in self.particles.iter() {
            let size = p.mass as f32 / 60000000.0;
            draw.ellipse()
                .resolution(15.0)
                .xy(p.pos / GALACTIC_ZOOM)
                .w_h(size, size)
                .color(WHITE);
        }

        // Write to the window frame.
        draw.to_frame(app, &frame).unwrap();
    }
}
