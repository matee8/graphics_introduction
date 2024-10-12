use std::process;

use graphics_introduction::{
    line::OneColorLine, polygon::Polygon, Color, Renderable,
};
use sdl2::event::Event;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

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

            let square = Polygon::new(
                &[
                    ((100, 100).into()),
                    ((100, canvas_height - 100).into()),
                    ((canvas_width - 100, canvas_height - 100).into()),
                    ((canvas_width - 100, 100).into()),
                ],
                Color::RED,
            )
            .unwrap_or_else(|_| {
                eprintln!("Invalid positions given for square polygon.");
                process::exit(1);
            });
            square.render(&mut canvas).unwrap_or_else(|_| {
                eprintln!("Couldn't draw square.");
                process::exit(1);
            });

            let line_inside_square = OneColorLine::new_inside_polygon(
                (0, canvas_height >> 2).into(),
                (canvas_width, canvas_height >> 2).into(),
                Color::RED,
                &square,
            )
            .unwrap_or_else(|| {
                eprintln!(
                    "Invalid positions given for line inside square polygon."
                );
                process::exit(1);
            });
            line_inside_square.render(&mut canvas).unwrap_or_else(|_| {
                eprintln!("Couldn't draw line inside square.");
                process::exit(1);
            });

            canvas.present();
        }
    }
}
