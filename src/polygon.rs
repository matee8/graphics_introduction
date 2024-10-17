use core::{iter, marker::PhantomData};

use thiserror::Error;

use crate::{
    line::{LineSegment, OneColorLine},
    Color, Point, Renderable, Renderer,
};

#[derive(Debug)]
pub struct Polygon<T, R>
where
    T: LineSegment + Renderable<R>,
    R: Renderer,
{
    edges: Vec<T>,
    _renderer: PhantomData<fn() -> R>,
}

#[non_exhaustive]
#[derive(Debug, Error, Clone, Copy)]
#[error("At least two points are required to create a polygon.")]
pub struct NotEnoughPointsError;

impl<R> Polygon<OneColorLine, R>
where
    R: Renderer,
{
    #[inline]
    pub fn new(
        points: &[Point],
        color: Color,
    ) -> Result<Self, NotEnoughPointsError> {
        if points.len() < 3 {
            return Err(NotEnoughPointsError);
        }

        #[expect(
            clippy::indexing_slicing,
            reason = "Points has to have at least a size of 3 at this point."
        )]
        let edges: Vec<OneColorLine> = points
            .windows(2)
            .map(|points| (&points[0], &points[1]))
            .chain(iter::once((&points[points.len() - 1], &points[0])))
            .map(|points| OneColorLine::new(*points.0, *points.1, color))
            .collect();

        Ok(Self {
            edges,
            _renderer: PhantomData,
        })
    }
}

impl<T, R> Polygon<T, R>
where
    T: LineSegment + Renderable<R>,
    R: Renderer,
{
    #[must_use]
    #[inline]
    pub fn edges(&self) -> &[T] {
        &self.edges
    }

    #[must_use]
    #[inline]
    pub fn points(&self) -> Vec<Point> {
        self.edges().iter().map(LineSegment::first_point).collect()
    }

    #[must_use]
    #[inline]
    #[expect(
        clippy::indexing_slicing,
        reason = "Polygon::points() has to have at least a size of 3 at this point."
    )]
    pub fn contains(&self, point: Point) -> bool {
        let points = self.points();

        self.points()
            .windows(2)
            .map(|edge| (edge[0], edge[1]))
            .chain(iter::once((points[points.len() - 1], points[0])))
            .filter(|&(first_point, last_point)| {
                (first_point.y > point.y) != (last_point.y > point.y)
            })
            .map(|(first_point, last_point)| {
                let slope = (last_point.x - first_point.x)
                    / (last_point.y - first_point.y);
                (point.y - first_point.y).mul_add(slope, first_point.x)
            })
            .filter(|&intersect_x| point.x < intersect_x)
            .count()
            & 1
            == 1
    }
}

#[non_exhaustive]
#[derive(Debug, Error, Clone)]
pub enum PolygonFromLinesError {
    #[error("At least 1 line is required to create a polygon.")]
    NotEnoughLines,
    #[error("The lines are required to touch to create a polygon.")]
    LinesDontTouch,
}

impl<T, R> Polygon<T, R>
where
    T: LineSegment + Renderable<R> + Clone,
    R: Renderer,
{
    #[inline]
    pub fn new_from_lines(lines: &[T]) -> Result<Self, PolygonFromLinesError> {
        if lines.len() < 3 {
            return Err(PolygonFromLinesError::NotEnoughLines);
        }

        #[expect(
            clippy::indexing_slicing,
            reason = "Lines has to have at least a size of 2 at this point."
        )]
        if !lines
            .windows(2)
            .map(|lines| (&lines[0], &lines[1]))
            .chain(iter::once((&lines[lines.len() - 1], &lines[0])))
            .all(|lines| lines.0.last_point() == lines.1.first_point())
        {
            return Err(PolygonFromLinesError::LinesDontTouch);
        }

        Ok(Self {
            edges: Vec::from(lines),
            _renderer: PhantomData,
        })
    }
}

impl<T, R> Renderable<R> for Polygon<T, R>
where
    T: LineSegment + Renderable<R>,
    R: Renderer,
{
    type Error = T::Error;

    #[inline]
    fn render(&self, renderer: &mut R) -> Result<(), Self::Error> {
        for edge in &self.edges {
            edge.render(renderer)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::iter;

    use crate::{line::OneColorLine, polygon::Polygon, Color, Point, Renderer};

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
    fn new_polygon_has_correct_points() {
        let points = [
            (100, 100).into(),
            (100, 200).into(),
            (200, 200).into(),
            (200, 100).into(),
        ];
        let polygon: Polygon<_, MockRenderer> =
            Polygon::new(&points, Color::RED).unwrap();

        assert_eq!(polygon.points(), points);
    }

    #[test]
    fn new_polygon_has_correct_edges() {
        let points = [
            (100, 100).into(),
            (100, 200).into(),
            (200, 200).into(),
            (200, 100).into(),
        ];
        let color = Color::RED;
        let polygon: Polygon<_, MockRenderer> =
            Polygon::new(&points, color).unwrap();

        let lines: Vec<OneColorLine> = points
            .windows(2)
            .map(|points| (points[0], points[1]))
            .chain(iter::once((points[points.len() - 1], points[0])))
            .map(|(start, end)| OneColorLine::new(start, end, color))
            .collect();

        for (i, edge) in polygon.edges().iter().enumerate() {
            assert_eq!(*edge, lines[i]);
        }
    }

    #[test]
    fn polygon_contains_returns_false_if_not_inside() {
        let polygon: Polygon<_, MockRenderer> = Polygon::new(
            &[
                (186, 14).into(),
                (186, 44).into(),
                (175, 115).into(),
                (175, 85).into(),
            ],
            Color::RED,
        )
        .unwrap();

        let point = (150, 85).into();

        assert!(!polygon.contains(point))
    }

    #[test]
    fn polygon_contains_returns_true_if_inside() {
        let polygon: Polygon<_, MockRenderer> = Polygon::new(
            &[(0, 0).into(), (5, 0).into(), (5, 5).into(), (0, 5).into()],
            Color::RED,
        )
        .unwrap();

        let point = (3, 3).into();

        assert!(polygon.contains(point))
    }
}
