pub mod line;
pub mod pixel;
pub mod polygon;
#[cfg(feature = "sdl2")]
pub mod sdl2;

const MAX_POSITION: i32 = i32::MAX >> 1;
const MIN_POSITION: i32 = i32::MIN >> 1;

const fn clamp_position(value: i32) -> i32 {
    if value > MAX_POSITION {
        MAX_POSITION
    } else if value < MIN_POSITION {
        MIN_POSITION
    } else {
        value
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    #[must_use]
    #[inline]
    pub const fn new(x: i32, y: i32) -> Self {
        Self {
            x: clamp_position(x),
            y: clamp_position(y),
        }
    }
}

impl From<(i32, i32)> for Point {
    #[inline]
    fn from(value: (i32, i32)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl From<Point> for (i32, i32) {
    #[inline]
    fn from(value: Point) -> Self {
        (value.x, value.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    pub const BLACK: Self = Self::new(0, 0, 0, 255);
    pub const BLUE: Self = Self::new(0, 0, 255, 255);
    pub const CYAN: Self = Self::new(0, 255, 255, 255);
    pub const GRAY: Self = Self::new(128, 128, 128, 255);
    pub const GREEN: Self = Self::new(0, 255, 0, 255);
    pub const GREY: Self = Self::GRAY;
    pub const MAGENTA: Self = Self::new(255, 0, 255, 255);
    pub const RED: Self = Self::new(255, 0, 0, 255);
    pub const WHITE: Self = Self::new(255, 255, 255, 255);
    pub const YELLOW: Self = Self::new(255, 255, 0, 255);

    #[must_use]
    #[inline]
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    #[must_use]
    #[inline]
    pub const fn new_rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            r,
            g,
            b,
            a: u8::MAX,
        }
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    #[inline]
    fn from(value: (u8, u8, u8, u8)) -> Self {
        Self::new(value.0, value.1, value.2, value.3)
    }
}

impl From<(u8, u8, u8)> for Color {
    #[inline]
    fn from(value: (u8, u8, u8)) -> Self {
        Self::new_rgb(value.0, value.1, value.2)
    }
}

pub trait Renderer {
    type DrawError;
    fn draw_point(&mut self, point: Point) -> Result<(), Self::DrawError>;
    fn draw_points(&mut self, points: &[Point]) -> Result<(), Self::DrawError> {
        for point in points {
            self.draw_point(*point)?;
        }
        Ok(())
    }
    fn set_color(&mut self, color: Color);
    fn current_color(&self) -> Color;
}

pub trait Renderable<T>
where
    T: Renderer,
{
    type Error;
    fn render(&self, renderer: &mut T) -> Result<(), Self::Error>;
}
