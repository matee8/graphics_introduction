use thiserror::Error;

use crate::{
    segment::{GeometricPrimitve, OneColorSegment},
    Color, Point, Renderable, Renderer, SMALL_ERROR_MARGIN,
};

#[derive(Debug, Clone, PartialEq)]
pub struct OneColorCurve {
    points: Vec<Point>,
    color: Color,
}

#[non_exhaustive]
#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[error("Wrong interval.")]
pub struct WrongInterval;

#[non_exhaustive]
#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CurveFromSegmentsError {
    #[error("At least 2 segments are required to create a curve.")]
    NotEnough,
    #[error("The segments are required to touch to create a curve.")]
    NotTouching,
}

impl OneColorCurve {
    #[inline]
    pub fn new_parametric<X, Y>(
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

        let mut points = Vec::new();

        let h = (end - start) / f64::from(num_segments);
        let mut t = start;
        let mut first_point = Point::new(x_fn(t), y_fn(t));

        #[expect(
            clippy::while_float,
            reason = "Algorithm has to be implemented this way."
        )]
        while (t - end).abs() > SMALL_ERROR_MARGIN {
            t += h;
            let last_point = Point::new(x_fn(t), y_fn(t));
            let segment = OneColorSegment::new(first_point, last_point, color);
            points.extend_from_slice(segment.points());
            first_point = last_point;
        }

        Ok(Self { points, color })
    }

    #[inline]
    pub fn new_implicit<F>(
        curve: F,
        color: Color,
        width: i32,
        height: i32,
    ) -> Self
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

        Self { points, color }
    }

    #[inline]
    pub fn new_from_segments<T>(
        segments: &[T],
        color: Color,
    ) -> Result<Self, CurveFromSegmentsError>
    where
        T: GeometricPrimitve + Clone,
    {
        if segments.len() < 2 {
            return Err(CurveFromSegmentsError::NotEnough);
        }

        #[expect(
            clippy::indexing_slicing,
            reason = "Segments has to have at least a size of 2 at this point."
        )]
        if !segments
            .windows(2)
            .map(|segments| (&segments[0], &segments[1]))
            .all(|segments| segments.0.last_point() == segments.1.first_point())
        {
            return Err(CurveFromSegmentsError::NotTouching);
        }

        let points = segments
            .iter()
            .flat_map(|segments| segments.points().iter().copied())
            .collect();

        Ok(Self { points, color })
    }

    #[must_use]
    #[inline]
    pub fn points(&self) -> &[Point] {
        &self.points
    }
}

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum CurveDrawError<T>
where
    T: Renderer,
{
    #[error("Couldn't draw the curve.")]
    Draw(T::DrawError),
    #[error("Curve was empty.")]
    Empty,
}

impl<R> Renderable<R> for OneColorCurve
where
    R: Renderer,
{
    type Error = CurveDrawError<R>;

    #[inline]
    fn render(&self, renderer: &mut R) -> Result<(), Self::Error> {
        let old_color = renderer.current_color();

        if self.points.is_empty() {
            return Err(CurveDrawError::Empty);
        }

        renderer.set_color(self.color);
        renderer
            .draw_points(&self.points)
            .map_err(CurveDrawError::Draw)?;

        renderer.set_color(old_color);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{curve::OneColorCurve, Color, Point, ERROR_MARGIN};

    #[test]
    fn new_parametric_curve_is_ok() {
        let curve = OneColorCurve::new_parametric(
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

        let curve = OneColorCurve::new_parametric(
            Color::RED,
            |t| t,
            |t| t,
            100.0,
            200.0,
            None,
        )
        .unwrap();

        let first = curve.points.first().unwrap();
        assert!(
            (first.x - start.x).abs() < ERROR_MARGIN
                && (first.y - start.y).abs() < ERROR_MARGIN
        );

        let last = curve.points.iter().last().unwrap();
        assert!(
            (last.x - end.x).abs() < ERROR_MARGIN
                && (last.y - end.y).abs() < ERROR_MARGIN
        );
    }

    #[test]
    fn new_implicit_curve_has_correct_endpoints() {
        let curve =
            OneColorCurve::new_implicit(|x, _| x, Color::RED, 1000, 1000);

        assert_eq!(curve.points.first(), Some(&Point::new(0.0, 0.0)));
        assert_eq!(curve.points.iter().last(), Some(&Point::new(0.0, 999.0)));
    }
}
