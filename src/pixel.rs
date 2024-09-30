use sdl2::{
    pixels::Color,
    rect::Point,
    render::{Canvas, RenderTarget},
};

use crate::Renderable;

#[derive(Debug, Clone, Copy)]
pub struct Pixel {
    point: Point,
    color: Color,
}

impl Renderable for Pixel {
    type Error = String;

    #[inline]
    fn draw<T>(&self, canvas: &mut Canvas<T>) -> Result<(), Self::Error>
    where
        T: RenderTarget,
    {
        let old_color = canvas.draw_color();
        canvas.set_draw_color(self.color);
        canvas.draw_point(self.point)?;
        canvas.set_draw_color(old_color);
        Ok(())
    }
}
