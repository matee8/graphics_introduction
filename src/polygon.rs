use sdl2::{
    pixels::Color,
    rect::Point,
    render::{Canvas, RenderTarget},
};
use thiserror::Error;

use crate::{
    line::OneColorLine,
    Renderable,
};

#[derive(Debug, Clone)]
pub struct OneColorPolygon {
    edges: Vec<OneColorLine>,
}

#[non_exhaustive]
#[derive(Debug, Error, Clone, Copy)]
#[error("At least two points are required to create a polygon.")]
pub struct NotEnoughPointsError;

impl OneColorPolygon {
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
            .map(|value| OneColorLine::new_all_deg(value[0], value[1], color))
            .collect();

        edges.push(OneColorLine::new_all_deg(
            points[points.len() - 1],
            points[0],
            color,
        ));

        Ok(Self { edges })
    }
}

impl Renderable for OneColorPolygon
{
    type Error = <OneColorLine as Renderable>::Error;

    #[inline]
    fn draw<T>(&self, canvas: &mut Canvas<T>) -> Result<(), Self::Error>
    where
        T: RenderTarget,
    {
        for edge in &self.edges {
            edge.draw(canvas)?;
        }

        Ok(())
    }
}
