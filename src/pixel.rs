use crate::{Color, Point, Renderable, Renderer};

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
    #[inline]
    fn from(value: (Point, Color)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl From<(i32, i32, Color)> for Pixel {
    #[inline]
    fn from(value: (i32, i32, Color)) -> Self {
        Self::new((value.0, value.1).into(), value.2)
    }
}

impl<T> Renderable<T> for Pixel
where
    T: Renderer,
{
    type Error = T::DrawError;

    #[inline]
    fn render(&self, renderer: &mut T) -> Result<(), Self::Error> {
        let old_color = renderer.current_color();
        renderer.set_color(self.color);
        renderer.draw_point(self.point)?;
        renderer.set_color(old_color);
        Ok(())
    }
}
