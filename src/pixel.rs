use sdl2::{
    pixels::Color,
    rect::Point,
    render::{Canvas, RenderTarget},
};

use crate::Renderable;

#[derive(Debug, Clone, Copy)]
pub struct Pixel {
    point: Point,
    color: Color,
}

impl Pixel {
    #[must_use]
    #[inline]
    pub const fn new(point: Point, color: Color) -> Self {
        Self { point, color }
    }
}

impl From<(Point, Color)> for Pixel {
    fn from(value: (Point, Color)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl From<(i32, i32, Color)> for Pixel {
    fn from(value: (i32, i32, Color)) -> Self {
        Self::new((value.0, value.1).into(), value.2)
    }
}

impl Renderable for Pixel {
    type Error = String;

    #[inline]
    fn draw<T>(&self, canvas: &mut Canvas<T>) -> Result<(), Self::Error>
    where
        T: RenderTarget,
    {
        let old_color = canvas.draw_color();
        canvas.set_draw_color(self.color);
        canvas.draw_point(self.point)?;
        canvas.set_draw_color(old_color);
        Ok(())
    }
}
