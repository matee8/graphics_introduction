use core::array::TryFromSliceError;

use sdl2::render::{Canvas, RenderTarget};

use crate::Renderable;

#[derive(Debug, Clone)]
pub struct Polygon<R, const N: usize>
where
    R: Renderable,
{
    edges: [R; N],
}

impl<R, const N: usize> Polygon<R, N>
where
    R: Renderable,
{
    #[inline]
    pub const fn new(lines: [R; N]) -> Self {
        Self { edges: lines }
    }
}

impl<R, const N: usize> From<[R; N]> for Polygon<R, N>
where
    R: Renderable,
{
    #[inline]
    fn from(value: [R; N]) -> Self {
        Self::new(value)
    }
}

impl<R, const N: usize> TryFrom<&[R]> for Polygon<R, N>
where
    R: Renderable + Clone + Copy,
{
    type Error = TryFromSliceError;

    #[inline]
    fn try_from(value: &[R]) -> Result<Self, Self::Error> {
        let edges = value.try_into()?;
        Ok(Self { edges })
    }
}

impl<R, const N: usize> Renderable for Polygon<R, N>
where
    R: Renderable,
{
    type Error = R::Error;

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
