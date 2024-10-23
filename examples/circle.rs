use core::f64;
use std::process;

use graphics_introduction::{
    curve::OneColorCurve, Color, Renderable,
};
use sdl2::event::Event;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

const RADIUS: f64 = 200.0;

fn main() {
    let sdl_ctx = sdl2::init().unwrap_or_else(|_| {
        eprintln!("Error initializing SDL2.");
        process::exit(1);
    });

    let vid_subsys = sdl_ctx.video().unwrap_or_else(|_| {
        eprintln!("Error initializing SDL2 video subsytem.");
        process::exit(1);
    });

    let window = vid_subsys
        .window("Introduction to computer graphics", WIDTH, HEIGHT)
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

    'running: loop {
        for event in event_pump.poll_iter() {
            if let Event::Quit { .. } = event {
                break 'running;
            }
            canvas.clear();

            let (canvas_width, canvas_height) =
                canvas.output_size().unwrap_or_else(|_| {
                    eprintln!("Drawing canvas has invalid sizes.");
                    process::exit(1);
                });
            let canvas_width: i32 =
                canvas_width.try_into().unwrap_or_else(|_| {
                    eprintln!("Invalid window width.");
                    process::exit(1);
                });
            let canvas_height: i32 =
                canvas_height.try_into().unwrap_or_else(|_| {
                    eprintln!("Invalid window height.");
                    process::exit(1);
                });

            let circle = OneColorCurve::new_parametric(
                Color::RED,
                |t| RADIUS * f64::cos(t) + f64::from(canvas_width >> 1),
                |t| RADIUS * f64::sin(t) + f64::from(canvas_height >> 1),
                0.0,
                2.0 * f64::consts::PI,
                None,
            )
            .unwrap_or_else(|_| {
                eprintln!("Invalid interval given for circle.");
                process::exit(1);
            });

            circle.render(&mut canvas).unwrap_or_else(|e| {
                eprintln!("{e}");
                eprintln!("Couldn't draw circle.");
                process::exit(1);
            });

            canvas.present();
        }
    }
}
