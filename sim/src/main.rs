use crate::particles::Particles;
use nannou::prelude::Update;
use nannou::{App, Frame};

mod particle;
mod particles;

fn model(app: &App) -> Particles {
    Particles::new(
        500,
        (60000000.0, 600000000.1),
        app.window_rect().top_right(),
        &mut rand::thread_rng(),
    )
}

fn update(app: &App, model: &mut Particles, update: Update) {
    model.tick(update.since_last)
}

fn view(app: &App, model: &Particles, frame: Frame) {
    println!("done! :)");
    model.view(app, frame)
}

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}
