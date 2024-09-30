use sdl2::render::{Canvas, RenderTarget};

use crate::line::{DrawError, OneColorLine};

#[derive(Debug, Clone)]
pub struct OneColorPolygon<'lines> {
    lines: &'lines [OneColorLine],
}

impl<'lines> OneColorPolygon<'lines> {
    #[inline]
    pub const fn new(
        lines: &'lines [OneColorLine],
    ) -> Self {
        Self {
            lines
        }
    }

    #[inline]
    pub fn draw<T>(&self, canvas: &mut Canvas<T>) -> Result<(), DrawError>
    where
        T: RenderTarget
    {
        for line in self.lines {
            line.draw(canvas)?;
        }

        Ok(())
    }
}
