use crate::particle::Particle;
use crate::particles::{ForceMap, Particles};
use nannou::geom::Vec2;
use nannou::noise::{NoiseFn, Perlin, Seedable};
use nannou::prelude::*;
use nannou::{App, Frame};
use rand::Rng;
use std::time::Duration;

mod particle;
mod particles;

struct Gravity {
    pub force: Vec2,
}

impl Gravity {
    pub fn new() -> Self {
        Self {
            force: Vec2::new(0.0, -9.81),
        }
    }
}

impl ForceMap for Gravity {
    fn apply_force(&self, particle: &mut Particle, elapsed: Duration) {
        particle.acceleration += self.force * elapsed.as_secs_f32();
    }
}

struct ForceField {
    perlin: Perlin,
    force: f32,
    scale: f32,
}

impl ForceField {
    pub fn new(seed: u32) -> Self {
        Self {
            perlin: Perlin::new().set_seed(seed),
            force: 25.0,
            scale: 0.01,
        }
    }
}

impl ForceMap for ForceField {
    fn apply_force(&self, particle: &mut Particle, elapsed: Duration) {
        let noise = self.perlin.get([
            (particle.pos.x * self.scale) as f64,
            (particle.pos.y * self.scale) as f64,
        ]);

        let angle = Vec2::ONE.rotate((noise * 6.28319) as f32);

        particle.acceleration = angle * self.force;
    }
}

fn model(app: &App) -> Particles<ForceField> {
    let mut rng = &mut rand::thread_rng();

    let mut field = ForceField::new(rng.gen());

    // Default is the squarish design

    // Super smooth
    // field.scale = 0.0005;

    // Flowy field
    field.scale = 0.0009;

    // Linear
    // field.scale = 0.0;

    //field.scale = 1.0;

    // Abstract
    // field.scale = 10.0;

    Particles::new(
        field,
        1000,
        (1.0, 3.0),
        app.window_rect().top_right(),
        Some(25.0),
        // None,
        vec![
            Rgba::new(29.0 / 255.0, 43.0 / 255.0, 83.0 / 255.0, 1.0),
            Rgba::new(126.0 / 255.0, 37.0 / 255.0, 83.0 / 255.0, 1.0),
            Rgba::new(0.0 / 255.0, 135.0 / 255.0, 81.0 / 255.0, 1.0),
            Rgba::new(171.0 / 255.0, 82.0 / 255.0, 54.0 / 255.0, 1.0),
            Rgba::new(95.0 / 255.0, 87.0 / 255.0, 79.0 / 255.0, 1.0),
            Rgba::new(194.0 / 255.0, 195.0 / 255.0, 199.0 / 255.0, 1.0),
            Rgba::new(255.0 / 255.0, 241.0 / 255.0, 232.0 / 255.0, 1.0),
            Rgba::new(255.0 / 255.0, 0.0 / 255.0, 77.0 / 255.0, 1.0),
            Rgba::new(255.0 / 255.0, 163.0 / 255.0, 0.0 / 255.0, 1.0),
            Rgba::new(255.0 / 255.0, 236.0 / 255.0, 39.0 / 255.0, 1.0),
            Rgba::new(0.0 / 255.0, 228.0 / 255.0, 54.0 / 255.0, 1.0),
            Rgba::new(41.0 / 255.0, 173.0 / 255.0, 255.0 / 255.0, 1.0),
            Rgba::new(131.0 / 255.0, 118.0 / 255.0, 156.0 / 255.0, 1.0),
            Rgba::new(255.0 / 255.0, 119.0 / 255.0, 168.0 / 255.0, 1.0),
            Rgba::new(255.0 / 255.0, 204.0 / 255.0, 170.0 / 255.0, 1.0),
        ],
        &mut rng,
    )
    .with_trail(0.01)
}

fn update(app: &App, model: &mut Particles<ForceField>, update: Update) {
    model.tick(update.since_last)
}

fn view(app: &App, model: &Particles<ForceField>, frame: Frame) {
    model.view(app, frame)
}

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}
