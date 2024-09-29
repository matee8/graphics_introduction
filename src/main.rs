use graphics_introduction::{line::OneColored, App};
use sdl2::{event::Event, pixels::Color};

fn main() {
    let mut app = App::build().unwrap();

    let line = OneColored::new_45_deg(
        (
            graphics_introduction::WIDTH >> 1,
            graphics_introduction::HEIGHT >> 1,
        )
            .into(),
        (graphics_introduction::WIDTH - 1, 0).into(),
        Color::RED,
    );

    let line2 = OneColored::new_all_deg(
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

    'running: loop {
        for event in app.event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                break 'running;
            }
            app.canvas.clear();

            line.draw(&mut app.canvas).unwrap();
            line2.draw(&mut app.canvas).unwrap();

            app.canvas.present();
        }
    }
}
