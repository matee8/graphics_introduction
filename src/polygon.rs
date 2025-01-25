use core::iter;
use std::borrow::Cow;

use thiserror::Error;

use crate::{
    segment::{LineSegment, OneColorSegment},
    Color, Point, Renderable, Renderer, Shape,
};

#[derive(Debug, Clone)]
pub struct Polygon<'edges, T>
where
    T: LineSegment + Clone,
{
    edges: Cow<'edges, [T]>,
}

#[non_exhaustive]
#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[error("At least three points are required to create a polygon.")]
pub struct NotEnoughPointsError;

impl Polygon<'_, OneColorSegment> {
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
        let edges: Vec<OneColorSegment> = points
            .windows(2)
            .map(|points| (&points[0], &points[1]))
            .chain(iter::once((&points[points.len() - 1], &points[0])))
            .map(|points| OneColorSegment::new(*points.0, *points.1, color))
            .collect();

        Ok(Self {
            edges: Cow::Owned(edges),
        })
    }
}

impl<T> Shape<T> for Polygon<'_, T>
where
    T: LineSegment + Clone,
{
    #[must_use]
    #[inline]
    fn edges(&self) -> &[T] {
        &self.edges
    }

    #[inline]
    #[must_use]
    #[expect(
        clippy::indexing_slicing,
        reason = "Polygon::points() has to have at least a size of 3 at this point."
    )]
    fn contains(&self, point: Point) -> bool {
        let points = self.vertices();

        self.vertices()
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
#[derive(Debug, Error, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PolygonFromSegmentsError {
    #[error("At least 3 segments are required to create a polygon.")]
    NotEnough,
    #[error("The segments are required to touch to create a polygon.")]
    NotTouching,
}

impl<'edges, T> Polygon<'edges, T>
where
    T: LineSegment + Clone,
{
    #[inline]
    pub fn new_from_segments(
        segments: &'edges [T],
    ) -> Result<Self, PolygonFromSegmentsError> {
        if segments.len() < 3 {
            return Err(PolygonFromSegmentsError::NotEnough);
        }

        #[expect(
            clippy::indexing_slicing,
            reason = "Segments has to have at least a size of 2 at this point."
        )]
        if !segments
            .windows(2)
            .map(|segments| (&segments[0], &segments[1]))
            .chain(iter::once((&segments[segments.len() - 1], &segments[0])))
            .all(|segments| segments.0.last_point() == segments.1.first_point())
        {
            return Err(PolygonFromSegmentsError::NotTouching);
        }

        Ok(Self {
            edges: Cow::Borrowed(segments),
        })
    }
}

impl<T, R> Renderable<R> for Polygon<'_, T>
where
    T: LineSegment + Renderable<R> + Clone,
    R: Renderer,
{
    type Error = T::Error;

    #[inline]
    fn render(&self, renderer: &mut R) -> Result<(), Self::Error> {
        for edge in &*self.edges {
            edge.render(renderer)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::iter;

    use crate::{polygon::Polygon, segment::OneColorSegment, Color, Shape};

    #[test]
    fn new_polygon_has_correct_vertices() {
        let points = [
            (100, 100).into(),
            (100, 200).into(),
            (200, 200).into(),
            (200, 100).into(),
        ];
        let polygon = Polygon::new(&points, Color::RED).unwrap();

        assert_eq!(polygon.vertices(), points);
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
        let polygon = Polygon::new(&points, color).unwrap();

        let segments: Vec<OneColorSegment> = points
            .windows(2)
            .map(|points| (points[0], points[1]))
            .chain(iter::once((points[points.len() - 1], points[0])))
            .map(|(start, end)| OneColorSegment::new(start, end, color))
            .collect();

        for (i, edge) in polygon.edges().iter().enumerate() {
            assert_eq!(*edge, segments[i]);
        }
    }

    #[test]
    fn polygon_contains_returns_false_if_not_inside() {
        let polygon = Polygon::new(
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
        let polygon = Polygon::new(
            &[(0, 0).into(), (5, 0).into(), (5, 5).into(), (0, 5).into()],
            Color::RED,
        )
        .unwrap();

        let point = (3, 3).into();

        assert!(polygon.contains(point))
    }
}
