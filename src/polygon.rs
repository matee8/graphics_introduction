use core::marker::PhantomData;

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
    _renderer: PhantomData<R>,
}

#[non_exhaustive]
#[derive(Debug, Error, Clone, Copy)]
#[error("At least two points are required to create a polygon.")]
pub struct NotEnoughPointsError;

#[non_exhaustive]
#[derive(Debug, Error, Clone)]
pub enum PolygonFromLinesError {
    #[error("At least 1 line is required to create a polygon.")]
    NotEnoughLines,
    #[error("The lines are required to touch to create a polygon.")]
    LinesDontTouch,
    #[error(
        "One color polygons can only be created from the same color lines."
    )]
    DifferentColor,
}

impl<R> Polygon<OneColorLine, R>
where
    R: Renderer,
{
    #[inline]
    pub fn new(
        points: &[Point],
        color: Color,
    ) -> Result<Self, NotEnoughPointsError> {
        if points.len() < 2 {
            return Err(NotEnoughPointsError);
        }
        let mut edges: Vec<OneColorLine> = points
            .windows(2)
            .map(|points| OneColorLine::new(points[0], points[1], color))
            .collect();

        edges.push(OneColorLine::new(
            points[points.len() - 1],
            points[0],
            color,
        ));

        Ok(Self {
            edges,
            _renderer: PhantomData,
        })
    }

    #[inline]
    pub fn new_from_lines(
        lines: &[OneColorLine],
    ) -> Result<Self, PolygonFromLinesError> {
        if lines.is_empty() {
            return Err(PolygonFromLinesError::NotEnoughLines);
        }

        if !lines
            .windows(2)
            .all(|lines| lines[0].last_point() == lines[1].first_point())
        {
            return Err(PolygonFromLinesError::LinesDontTouch);
        }

        if lines[lines.len() - 1].last_point() != lines[0].first_point() {
            return Err(PolygonFromLinesError::LinesDontTouch);
        }

        if lines.first().is_some_and(|first| {
            lines
                .iter()
                .skip(1)
                .all(|elem| elem.color() == first.color())
        }) {
            return Err(PolygonFromLinesError::DifferentColor);
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
