use core::iter;
use std::borrow::Cow;

use thiserror::Error;

use crate::{
    curve::{HermiteArc, OneColorCurve, WrongInterval},
    polygon::NotEnoughPointsError,
    segment::OneColorSegment,
    Color, GeometricPrimitve, Point, Renderable, Renderer, Shape,
};

#[derive(Debug, Clone)]
pub struct Figure<'edges, T>
where
    T: GeometricPrimitve + Clone,
{
    edges: Cow<'edges, [T]>,
}

impl Figure<'_, OneColorSegment> {
    #[inline]
    pub fn new_from_points(
        points: &[Point],
        color: Color,
    ) -> Result<Self, NotEnoughPointsError> {
        if points.len() < 2 {
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

#[non_exhaustive]
#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FigureFromPrimitivesError {
    #[error(
        "The geometric primitives are required to touch to create a figure."
    )]
    NotTouching,
    #[error("Given Vec cannot be empty, no empty figures exist.")]
    Empty,
}

impl<'edges, T> Figure<'edges, T>
where
    T: GeometricPrimitve + Clone,
{
    fn check_primitives(
        curves: &'edges [T],
    ) -> Result<(), FigureFromPrimitivesError> {
        #[expect(
            clippy::indexing_slicing,
            reason = "Curves has to have at least a size of 2 at this point."
        )]
        match curves.len() {
            0 => return Err(FigureFromPrimitivesError::Empty),
            1 => {
                if curves[0].first_point() != curves[0].last_point() {
                    return Err(FigureFromPrimitivesError::NotTouching);
                }
            }
            _ => {
                if !curves
                    .windows(2)
                    .map(|curves| (&curves[0], &curves[1]))
                    .chain(iter::once((&curves[curves.len() - 1], &curves[0])))
                    .all(|curves| {
                        curves.0.last_point() == curves.1.first_point()
                    })
                {
                    return Err(FigureFromPrimitivesError::NotTouching);
                }
            }
        }

        Ok(())
    }

    #[inline]
    pub fn new_from_primitives(
        curves: &'edges [T],
    ) -> Result<Self, FigureFromPrimitivesError> {
        Self::check_primitives(curves)?;

        Ok(Self {
            edges: Cow::Borrowed(curves),
        })
    }
}

impl<T> Shape<T> for Figure<'_, T>
where
    T: GeometricPrimitve + Clone,
{
    #[inline]
    #[must_use]
    fn edges(&self) -> &[T] {
        &self.edges
    }
}

impl<T, R> Renderable<R> for Figure<'_, T>
where
    T: GeometricPrimitve + Clone + Renderable<R>,
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

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct HermiteArcFigureBuilder {
    arcs: Vec<HermiteArc>,
    is_closed: bool,
}

#[non_exhaustive]
#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HermiteArcFigureBuildError {
    #[error(transparent)]
    WrongInterval(#[from] WrongInterval),
    #[error(transparent)]
    FigureFromPrimitivesError(#[from] FigureFromPrimitivesError),
}

impl HermiteArcFigureBuilder {
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {
            arcs: Vec::new(),
            is_closed: false,
        }
    }

    #[inline]
    #[must_use]
    pub fn add_arc(mut self, arc: HermiteArc) -> Self {
        self.arcs.push(arc);
        self
    }

    #[inline]
    #[must_use]
    pub const fn close(mut self) -> Self {
        self.is_closed = true;
        self
    }

    #[inline]
    pub fn build(
        self,
    ) -> Result<Figure<'static, OneColorCurve>, HermiteArcFigureBuildError>
    {
        let curves: Vec<OneColorCurve> = if self.is_closed {
            self.arcs
                .iter()
                .map(|hermite_arc| (*hermite_arc).try_into())
                .collect::<Result<_, _>>()?
        } else {
            let last_given_arc = self.arcs[self.arcs.len() - 1];
            let first_given_arc = self.arcs[0];
            let last_arc = HermiteArc::new(
                *last_given_arc.color(),
                *last_given_arc.end(),
                *last_given_arc.end_tangent(),
                *first_given_arc.start(),
                *first_given_arc.start_tangent(),
                *last_given_arc.num_segments(),
            );
            self.arcs
                .iter()
                .chain(iter::once(&last_arc))
                .map(|hermite_arc| (*hermite_arc).try_into())
                .collect::<Result<_, _>>()?
        };

        Figure::check_primitives(&curves)?;

        let figure = Figure {
            edges: Cow::Owned(curves),
        };

        Ok(figure)
    }
}

#[cfg(test)]
mod tests {
    use core::iter;

    use crate::{figure::Figure, segment::OneColorSegment, Color, Shape};

    #[test]
    fn new_figure_has_correct_vertices() {
        let points = [
            (100, 100).into(),
            (100, 200).into(),
            (200, 200).into(),
            (200, 100).into(),
        ];
        let polygon = Figure::new_from_points(&points, Color::RED).unwrap();

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
        let polygon = Figure::new_from_points(&points, color).unwrap();

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
}
