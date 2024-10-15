use thiserror::Error;

use crate::{
    line::{LineDrawError, OneColorLine},
    Color, Point, Renderable, Renderer, ERROR_MARGIN,
};

#[derive(Debug)]
pub struct OneColorParametricCurve {
    segments: Vec<OneColorLine>,
}

#[non_exhaustive]
#[derive(Debug, Error, Clone, Copy)]
#[error("Wrong interval.")]
pub struct WrongInterval;

impl OneColorParametricCurve {
    #[inline]
    pub fn new<X, Y>(
        color: Color,
        x_fn: X,
        y_fn: Y,
        start: f64,
        end: f64,
        n: i32,
    ) -> Result<Self, WrongInterval>
    where
        X: Fn(f64) -> f64,
        Y: Fn(f64) -> f64,
    {
        if end <= start {
            return Err(WrongInterval);
        }

        let mut segments = Vec::new();

        let h = end - start / f64::from(n);
        let mut t = start;
        let mut point0 = Point::new(x_fn(t), y_fn(t));

        while (t - end).abs() < ERROR_MARGIN {
            t += h;
            let point1 = Point::new(x_fn(t), y_fn(t));
            segments.push(OneColorLine::new(point0, point1, color));
            point0 = point1;
        }

        Ok(Self { segments })
    }
}

impl<T> Renderable<T> for OneColorParametricCurve
where
    T: Renderer,
{
    type Error = LineDrawError<T>;

    #[inline]
    fn render(&self, renderer: &mut T) -> Result<(), Self::Error>
    where
        T: Renderer,
    {
        let old_color = renderer.current_color();

        if self.segments.is_empty() {
            return Err(LineDrawError::Empty);
        }

        renderer.set_color(self.segments[0].color());
        for segment in &self.segments {
            segment.render(renderer)?;
        }

        renderer.set_color(old_color);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{curve::OneColorParametricCurve, Color, Point, Renderer};

    struct MockRenderer;

    impl Renderer for MockRenderer {
        type DrawError = ();

        fn set_color(&mut self, color: Color) {
            let _ = color;
            unimplemented!()
        }

        fn draw_point(&mut self, point: Point) -> Result<(), Self::DrawError> {
            let _ = point;
            unimplemented!()
        }

        fn draw_points(
            &mut self,
            points: &[Point],
        ) -> Result<(), Self::DrawError> {
            let _ = points;
            unimplemented!()
        }

        fn current_color(&self) -> Color {
            unimplemented!()
        }
    }
}
