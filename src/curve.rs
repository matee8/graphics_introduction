use thiserror::Error;

use crate::{
    line::{LineDrawError, OneColorLine},
    Color, Point, Renderable, Renderer, SMALL_ERROR_MARGIN,
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
        num_segments: Option<i32>,
    ) -> Result<Self, WrongInterval>
    where
        X: Fn(f64) -> f64,
        Y: Fn(f64) -> f64,
    {
        if end <= start {
            return Err(WrongInterval);
        }

        let num_segments = num_segments.unwrap_or(500);

        let mut segments = Vec::new();

        let h = (end - start) / f64::from(num_segments);
        let mut t = start;
        let mut point0 = Point::new(x_fn(t), y_fn(t));

        #[expect(
            clippy::while_float,
            reason = "Algorithm has to be implemented this way."
        )]
        while (t - end).abs() > SMALL_ERROR_MARGIN {
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

        let line_color = if let Some(first_segment) = self.segments.first() {
            first_segment.color()
        } else {
            return Err(LineDrawError::Empty);
        };

        renderer.set_color(line_color);
        for segment in &self.segments {
            segment.render(renderer)?;
        }

        renderer.set_color(old_color);

        Ok(())
    }
}

#[derive(Debug)]
pub struct OneColorImplicitCurve {
    color: Color,
    points: Vec<Point>,
}

impl OneColorImplicitCurve {
    #[inline]
    pub fn new<F>(curve: F, color: Color, width: i32, height: i32) -> Self
    where
        F: Fn(f64, f64) -> f64,
    {
        let mut points = Vec::new();

        for i in 0..width {
            for j in 0..height {
                if curve(f64::from(i), f64::from(j)).abs() < SMALL_ERROR_MARGIN
                {
                    points.push((i, j).into());
                }
            }
        }

        Self { color, points }
    }
}

impl<T> Renderable<T> for OneColorImplicitCurve
where
    T: Renderer,
{
    type Error = T::DrawError;

    #[inline]
    fn render(&self, renderer: &mut T) -> Result<(), Self::Error>
    where
        T: Renderer,
    {
        let old_color = renderer.current_color();

        renderer.set_color(self.color);
        for point in &self.points {
            println!("{:?}", point);
            renderer.draw_point(*point)?;
        }

        renderer.set_color(old_color);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        curve::{OneColorImplicitCurve, OneColorParametricCurve},
        line::LineSegment,
        Color, Point, Renderer, ERROR_MARGIN,
    };

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

    #[test]
    fn new_parametric_curve_is_ok() {
        let curve = OneColorParametricCurve::new(
            Color::RED,
            |t| t,
            |t| t,
            100.0,
            200.0,
            None,
        );

        assert!(curve.is_ok());
    }

    #[test]
    fn new_parametric_curve_has_correct_endpoints() {
        let start = Point::new(100.0, 100.0);
        let end = Point::new(200.0, 200.0);

        let curve = OneColorParametricCurve::new(
            Color::RED,
            |t| t,
            |t| t,
            100.0,
            200.0,
            None,
        );

        let curve = curve.unwrap();
        let first_segment = curve.segments.first().unwrap();
        let (x, y) =
            (first_segment.first_point().x, first_segment.first_point().y);
        assert!(
            (x - start.x).abs() < ERROR_MARGIN
                && (y - start.y).abs() < ERROR_MARGIN
        );

        let last_segment = curve.segments.iter().last().unwrap();
        let (x, y) =
            (last_segment.first_point().x, last_segment.first_point().y);
        assert!(
            (x - end.x).abs() < ERROR_MARGIN
                && (y - end.y).abs() < ERROR_MARGIN
        );
    }

    #[test]
    fn new_implicit_curve_has_correct_endpoints() {
        let curve =
            OneColorImplicitCurve::new(|x, _| x, Color::RED, 1000, 1000);

        assert_eq!(curve.points.first(), Some(&Point::new(0.0, 0.0)));
        assert_eq!(curve.points.iter().last(), Some(&Point::new(0.0, 999.0)));
    }
}
