use sdl2::render::{Canvas, RenderTarget};

use crate::Renderable;

#[derive(Debug, Clone, Copy)]
pub struct Polygon<'edges, R>
where
    R: Renderable,
{
    edges: &'edges [R],
}

impl<'edges, R> Polygon<'edges, R>
where
    R: Renderable,
{
    #[inline]
    pub const fn new(lines: &'edges [R]) -> Self {
        Self { edges: lines }
    }
}

impl<R> Renderable for Polygon<'_, R>
where
    R: Renderable,
{
    type Error = R::Error;

    #[inline]
    fn draw<T>(&self, canvas: &mut Canvas<T>) -> Result<(), Self::Error>
    where
        T: RenderTarget,
    {
        for edge in self.edges {
            edge.draw(canvas)?;
        }

        Ok(())
    }
}
