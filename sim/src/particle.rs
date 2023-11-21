use nannou::prelude::{Point2, Rgba, Vec2};
use std::time::Duration;

pub struct Particle {
    pub color: Rgba,
    pub pos: Point2,
    pub mass: f32,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub spring: f32,
    pub terminal_vel: Option<f32>,
}

impl Particle {
    pub fn new(mass: f32, pos: Point2) -> Self {
        Self {
            color: Rgba::new(1.0, 1.0, 1.0, 1.0),
            pos,
            mass,
            velocity: Default::default(),
            acceleration: Default::default(),
            spring: 0.0,
            terminal_vel: None,
        }
    }

    pub fn with_color(mut self, color: Rgba) -> Self {
        self.color = color;
        self
    }

    pub fn with_spring(mut self, bounce: f32) -> Self {
        self.spring = bounce;
        self
    }

    pub fn with_terminal_velocity(mut self, limit: Option<f32>) -> Self {
        self.terminal_vel = limit;
        self
    }

    // Add velocity to the position
    pub fn displace(&mut self, elapsed: Duration) {
        self.pos += self.velocity * elapsed.as_secs_f32()
    }

    // Get velocity from the acceleration
    pub fn calculate_velocity(&mut self, elapsed: Duration) {
        self.velocity += self.acceleration * elapsed.as_secs_f32();
        if let Some(limit) = self.terminal_vel {
            self.velocity = self.velocity.clamp_length(0.0, limit);
        }
    }
}
