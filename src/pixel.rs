use sdl2::{
    pixels::Color,
    rect::Point,
    render::{Canvas, RenderTarget},
};

#[derive(Debug, Clone, Copy)]
pub struct Pixel {
    point: Point,
    color: Color,
}

impl Pixel {
    #[inline]
    pub fn draw_to_canvas<T>(
        &self,
        canvas: &mut Canvas<T>,
    ) -> Result<(), String>
    where
        T: RenderTarget,
    {
        canvas.set_draw_color(self.color);
        canvas.draw_point(self.point)
    }
}
