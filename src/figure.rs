use core::iter;
use std::borrow::Cow;

use thiserror::Error;

use crate::{GeometricPrimitve, Renderable, Renderer};

#[derive(Debug, Clone)]
pub struct Figure<'edges, T>
where
    T: GeometricPrimitve + Clone,
{
    edges: Cow<'edges, Vec<T>>,
}

#[non_exhaustive]
#[derive(Debug, Error, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    #[inline]
    pub fn new_from_primitives(
        curves: &'edges Vec<T>,
    ) -> Result<Self, FigureFromPrimitivesError> {
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

        Ok(Self {
            edges: Cow::Borrowed(curves),
        })
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
