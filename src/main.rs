use std::process;

use graphics_introduction::{
    line::OneColorLine, polygon::Polygon, App, Renderable,
};
use sdl2::{event::Event, pixels::Color};

fn main() {
    let mut app = App::build().unwrap_or_else(|_| {
        eprintln!("Couldn't run app.");
        process::exit(1);
    });

    let line = OneColorLine::new_45_deg(
        (
            graphics_introduction::WIDTH >> 1,
            graphics_introduction::HEIGHT >> 1,
        )
            .into(),
        (graphics_introduction::WIDTH - 1, 0).into(),
        Color::RED,
    );

    let line2 = OneColorLine::new_all_deg(
        (
            graphics_introduction::WIDTH >> 1,
            graphics_introduction::HEIGHT >> 1,
        )
            .into(),
        (
            graphics_introduction::WIDTH - 1,
            graphics_introduction::HEIGHT - 1,
        )
            .into(),
        Color::RED,
    );

    let polygon = Polygon::new([line, line2]);

    'running: loop {
        for event in app.event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                break 'running;
            }
            app.canvas.clear();

            polygon.draw(&mut app.canvas).unwrap_or_else(|_| {
                eprintln!("Couldn't draw polygon.");
                process::exit(1);
            });

            app.canvas.present();
        }
    }
}
