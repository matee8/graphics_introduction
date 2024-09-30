pub mod line;
pub mod pixel;
pub mod polygon;

use sdl2::{
    pixels::Color,
    rect::Point,
    render::{Canvas, RenderTarget},
    video::{Window, WindowBuildError},
    EventPump, IntegerOrSdlError,
};
use thiserror::Error;

pub const WIDTH: i32 = 640;
pub const HEIGHT: i32 = 480;

#[non_exhaustive]
pub struct App {
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
}

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum BuildError {
    #[error("{0}")]
    SdlInit(String),
    #[error("{0}")]
    VideoSubsystem(String),
    #[error(transparent)]
    Window(#[from] WindowBuildError),
    #[error(transparent)]
    Integer(#[from] IntegerOrSdlError),
    #[error("{0}")]
    EventPump(String),
}

impl App {
    #[inline]
    pub fn build() -> Result<Self, BuildError> {
        let sdl_ctx = sdl2::init().map_err(BuildError::SdlInit)?;
        let vid_subsys = sdl_ctx.video().map_err(BuildError::VideoSubsystem)?;

        let window = vid_subsys
            .window(
                "Introduction to computer graphics",
                WIDTH as u32,
                HEIGHT as u32,
            )
            .position_centered()
            .build()?;

        let mut canvas = window.into_canvas().build()?;

        canvas.set_draw_color(Color::WHITE);
        canvas.clear();
        canvas.present();

        let event_pump = sdl_ctx.event_pump().map_err(BuildError::EventPump)?;

        Ok(Self { canvas, event_pump })
    }
}

pub trait Renderable {
    type Error;
    fn draw<T>(&self, canvas: &mut Canvas<T>) -> Result<(), Self::Error>
    where
        T: RenderTarget;
}

impl Renderable for Point {
    type Error = String;

    #[inline]
    fn draw<T>(&self, canvas: &mut Canvas<T>) -> Result<(), Self::Error>
    where
        T: RenderTarget,
    {
        canvas.draw_point(*self)
    }
}
