use core::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign,
};

pub mod curve;
pub mod pixel;
pub mod polygon;
#[cfg(feature = "sdl2")]
pub mod sdl2;
pub mod segment;

const SMALL_ERROR_MARGIN: f64 = 0.001;
const ERROR_MARGIN: f64 = 0.7;

pub trait Renderer {
    type DrawError;
    fn draw_point(&mut self, point: Point) -> Result<(), Self::DrawError>;
    #[inline]
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

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    #[must_use]
    #[inline]
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl From<(i32, i32)> for Point {
    #[inline]
    fn from(value: (i32, i32)) -> Self {
        Self::new(value.0.into(), value.1.into())
    }
}

impl From<(f64, f64)> for Point {
    #[inline]
    fn from(value: (f64, f64)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl From<Point> for (f64, f64) {
    #[inline]
    fn from(value: Point) -> Self {
        (value.x, value.y)
    }
}

impl Add for Point {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Point {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Div<f64> for Point {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl DivAssign<f64> for Point {
    #[inline]
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl Mul<f64> for Point {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl MulAssign<f64> for Point {
    #[inline]
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Sub for Point {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Point {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

impl Add for Color {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r.saturating_add(rhs.r),
            g: self.g.saturating_add(rhs.g),
            b: self.b.saturating_add(rhs.b),
            a: self.a.saturating_add(rhs.a),
        }
    }
}

impl AddAssign for Color {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.r = self.r.saturating_add(rhs.r);
        self.g = self.g.saturating_add(rhs.g);
        self.b = self.b.saturating_add(rhs.b);
        self.a = self.a.saturating_add(rhs.a);
    }
}

impl Div<u8> for Color {
    type Output = Self;

    #[inline]
    fn div(self, rhs: u8) -> Self::Output {
        Self {
            r: self.r.saturating_div(rhs),
            g: self.g.saturating_div(rhs),
            b: self.b.saturating_div(rhs),
            a: self.a.saturating_div(rhs),
        }
    }
}

impl DivAssign<u8> for Color {
    #[inline]
    fn div_assign(&mut self, rhs: u8) {
        self.r = self.r.saturating_div(rhs);
        self.g = self.g.saturating_div(rhs);
        self.b = self.b.saturating_div(rhs);
        self.a = self.a.saturating_div(rhs);
    }
}

impl Mul<u8> for Color {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: u8) -> Self::Output {
        Self {
            r: self.r.saturating_mul(rhs),
            g: self.g.saturating_mul(rhs),
            b: self.b.saturating_mul(rhs),
            a: self.a.saturating_mul(rhs),
        }
    }
}

impl MulAssign<u8> for Color {
    #[inline]
    fn mul_assign(&mut self, rhs: u8) {
        self.r = self.r.saturating_mul(rhs);
        self.g = self.g.saturating_mul(rhs);
        self.b = self.b.saturating_mul(rhs);
        self.a = self.a.saturating_mul(rhs);
    }
}

impl Sub for Color {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            r: self.r.saturating_sub(rhs.r),
            g: self.g.saturating_sub(rhs.g),
            b: self.b.saturating_sub(rhs.b),
            a: self.a.saturating_sub(rhs.a),
        }
    }
}

impl SubAssign for Color {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.r = self.r.saturating_sub(rhs.r);
        self.g = self.g.saturating_sub(rhs.g);
        self.b = self.b.saturating_sub(rhs.b);
        self.a = self.a.saturating_sub(rhs.a);
    }
}
