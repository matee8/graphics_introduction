pub mod line;
pub mod pixel;
pub mod polygon;

use core::{
    fmt::{self, Debug, Formatter},
    num::TryFromIntError,
};

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

impl Debug for App {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("App")
            .field("canvas", &"Canvas<Window>")
            .field("event_pump", &"EventPump")
            .finish()
    }
}

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum BuildError {
    #[error("{0}")]
    SdlInit(String),
    #[error("Value was not a valid signed integer.")]
    WidthHeightConversion(#[from] TryFromIntError),
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
                u32::try_from(WIDTH)?,
                u32::try_from(HEIGHT)?,
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
