use sdl2::render::{Canvas, RenderTarget};

use crate::{Color, Point, Renderer};

impl From<Point> for sdl2::rect::Point {
    fn from(value: Point) -> Self {
        Self::new(value.x.round() as i32, value.y.round() as i32)
    }
}

impl From<sdl2::rect::Point> for Point {
    #[inline]
    fn from(value: sdl2::rect::Point) -> Self {
        Self::new(value.x.into(), value.y.into())
    }
}

impl From<Color> for sdl2::pixels::Color {
    #[inline]
    fn from(value: Color) -> Self {
        Self::RGBA(value.r, value.g, value.b, value.a)
    }
}

impl From<sdl2::pixels::Color> for Color {
    #[inline]
    fn from(value: sdl2::pixels::Color) -> Self {
        Self::new(value.r, value.g, value.b, value.a)
    }
}

impl<T> Renderer for Canvas<T>
where
    T: RenderTarget,
{
    type DrawError = String;

    #[inline]
    fn set_color(&mut self, color: Color) {
        let color: sdl2::pixels::Color = color.into();
        self.set_draw_color(color);
    }

    #[inline]
    fn current_color(&self) -> Color {
        self.draw_color().into()
    }

    #[inline]
    fn draw_point(&mut self, point: Point) -> Result<(), Self::DrawError> {
        self.draw_point(point)
    }

    #[inline]
    fn draw_points(&mut self, points: &[Point]) -> Result<(), Self::DrawError> {
        let points: Vec<sdl2::rect::Point> =
            points.iter().map(|point| (*point).into()).collect();
        self.draw_points(points.as_ref())
    }
}
