use crate::particle::Particle;
use nannou::geom::Vec2;
use nannou::prelude::{Point2, Rgba, BLACK, BLEND_SUBTRACT};
use nannou::{App, Frame};
use rand::rngs::ThreadRng;
use rand::Rng;
use rayon::prelude::*;
use std::time::{Duration, Instant};

pub enum OutOfBoundsEffect {
    WrapAround,
    Stop,
}

pub struct Particles<T: ForceMap> {
    pub wrap: OutOfBoundsEffect,
    pub fps_counter: bool,
    pub trail: Option<f32>,
    pub particles: Vec<Particle>,
    pub force_map: T,
    pub window: Point2,
}

impl<T: ForceMap + Sync> Particles<T> {
    pub fn new(
        force_map: T,
        particle_qty: usize,
        mass_clamp: (f32, f32),
        size: Point2,
        limit: Option<f32>,
        colors: Vec<Rgba>,
        rng: &mut ThreadRng,
    ) -> Self {
        let mut particles = vec![];

        for _ in 0..particle_qty {
            particles.push(
                Particle::new(
                    rng.gen_range(mass_clamp.0..=mass_clamp.1),
                    Point2::new(
                        rng.gen_range((-1.0 * size.x)..size.x),
                        rng.gen_range((-1.0 * size.y)..size.y),
                    ),
                )
                .with_spring(-0.4)
                .with_terminal_velocity(limit)
                .with_color(colors.get(rng.gen_range(0..colors.len())).unwrap().clone()),
            )
        }

        Self {
            force_map,
            fps_counter: true,
            trail: None,
            wrap: OutOfBoundsEffect::WrapAround,
            particles,
            window: size,
        }
    }

    pub fn with_trail(mut self, trail: f32) -> Self {
        self.trail = Some(trail);
        self
    }

    pub fn set_bounds(&mut self, bounds: OutOfBoundsEffect) {
        self.wrap = bounds
    }

    pub fn tick(&mut self, elapsed: Duration) {
        let force = &self.force_map;
        let window = &self.window;

        let now = Instant::now();
        let _ = self.particles.par_iter_mut().for_each(|particle| {
            force.apply_force(particle, elapsed);
            particle.calculate_velocity(elapsed);
            particle.displace(elapsed);

            // Bounds check
            if particle.pos.x > window.x
                || particle.pos.x < window.x * -1.0
                || particle.pos.y > window.y
                || particle.pos.y < window.y * -1.0
            {
                let mut rng = rand::thread_rng();
                particle.pos.x = rng.gen_range(window.x * -1.0..window.x);
                particle.pos.y = rng.gen_range(window.y * -1.0..window.y);
                particle.velocity = Vec2::ZERO;
                particle.acceleration = Vec2::ZERO;
            }
        });
        if self.fps_counter {
            println!("PARALLEL: {}", now.elapsed().as_secs_f32());
        }

        // let now = Instant::now();
        // for particle in self.particles.iter_mut() {
        //     force.apply_force(particle, elapsed);
        //     particle.calculate_velocity(elapsed);
        //     particle.displace(elapsed);
        //
        //     // Bounds check
        //     if particle.pos.x > window.x {
        //         particle.pos.x = window.x * -1.0;
        //     } else if particle.pos.x < window.x * -1.0 {
        //         particle.pos.x = window.x;
        //     } else if particle.pos.y > window.y {
        //         particle.pos.y = window.y * -1.0;
        //     } else if particle.pos.y < window.y * -1.0 {
        //         particle.pos.y = window.y;
        //     }
        // }
        // if self.fps_counter {
        //     println!("LINEAR: {}", now.elapsed().as_secs_f32());
        // }

        if self.fps_counter {
            println!("FPS: {}", elapsed.as_secs_f32());
        }
    }

    pub fn view(&self, app: &App, frame: Frame) {
        let draw = app.draw();
        let draw = draw.alpha_blend(BLEND_SUBTRACT);

        if let Some(trail) = self.trail {
            draw.rect()
                .xy(Point2::default())
                .wh(app.window_rect().wh())
                .rgba(0., 0., 0., trail);
        } else {
            draw.background().color(BLACK);
        }

        for p in self.particles.iter() {
            draw.ellipse()
                .resolution(15.0)
                .xy(p.pos)
                .w_h(p.mass, p.mass)
                .color(p.color);
        }

        // Write to the window frame.
        draw.to_frame(app, &frame).unwrap();
    }
}

pub trait ForceMap {
    fn apply_force(&self, particle: &mut Particle, elapsed: Duration);
}
