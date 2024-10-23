use graphics_introduction::{Color, Point, Renderer};

pub struct MockRenderer;

impl Renderer for MockRenderer {
    type DrawError = ();

    fn set_color(&mut self, color: Color) {
        let _ = color;
        unimplemented!()
    }

    fn draw_point(&mut self, point: Point) -> Result<(), Self::DrawError> {
        let _ = point;
        unimplemented!()
    }

    fn draw_points(&mut self, points: &[Point]) -> Result<(), Self::DrawError> {
        let _ = points;
        unimplemented!()
    }

    fn current_color(&self) -> Color {
        unimplemented!()
    }
}
