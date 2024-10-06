use std::process;

use graphics_introduction::{
    line::OneColorLine, polygon::OneColorPolygon, Color, Renderable,
};
use sdl2::event::Event;

const WIDTH: i32 = 640;
const HEIGHT: i32 = 480;

fn main() {
    let sdl_ctx = sdl2::init().unwrap_or_else(|_| {
        eprintln!("Error initializing SDL2.");
        process::exit(1);
    });

    let vid_subsys = sdl_ctx.video().unwrap_or_else(|_| {
        eprintln!("Error initializing SDL2 video subsytem.");
        process::exit(1);
    });

    let window_width: u32 = WIDTH.try_into().unwrap_or_else(|_| {
        eprintln!("Invalid window width.");
        process::exit(1);
    });
    let window_height: u32 = HEIGHT.try_into().unwrap_or_else(|_| {
        eprintln!("Invalid window height.");
        process::exit(1);
    });
    let window = vid_subsys
        .window(
            "Introduction to computer graphics",
            window_width,
            window_height,
        )
        .resizable()
        .build()
        .unwrap_or_else(|_| {
            eprintln!("Error creating window.");
            process::exit(1);
        });

    let mut canvas = window.into_canvas().build().unwrap_or_else(|_| {
        eprintln!("Couldn't turn window into canvas.");
        process::exit(1);
    });

    let mut event_pump = sdl_ctx.event_pump().unwrap_or_else(|_| {
        eprintln!("Couldn't create event pump to get the events from SDL.");
        process::exit(1);
    });

    canvas.set_draw_color(sdl2::pixels::Color::WHITE);
    canvas.clear();
    canvas.present();

    let line = OneColorLine::new_45_deg(
        (WIDTH >> 1, HEIGHT >> 1).into(),
        (WIDTH - 1, 0).into(),
        Color::RED,
    );

    let line2 = OneColorLine::new(
        (WIDTH >> 1, HEIGHT >> 1).into(),
        (WIDTH - 1, HEIGHT - 1).into(),
        Color::RED,
    );

    let square = OneColorPolygon::new(
        &[
            ((100, 100).into()),
            ((100, 200).into()),
            ((200, 200).into()),
            ((200, 100).into()),
        ],
        Color::RED,
    )
    .unwrap_or_else(|_| {
        eprintln!("Invalid positions given for square polygon.");
        process::exit(1);
    });

    'running: loop {
        for event in event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                break 'running;
            }
            canvas.clear();

            line.render(&mut canvas).unwrap_or_else(|_| {
                eprintln!("Couldn't draw line.");
                process::exit(1);
            });
            line2.render(&mut canvas).unwrap_or_else(|_| {
                eprintln!("Couldn't draw line.");
                process::exit(1);
            });
            square.render(&mut canvas).unwrap_or_else(|_| {
                eprintln!("Couldn't draw square.");
                process::exit(1);
            });

            canvas.present();
        }
    }
}
