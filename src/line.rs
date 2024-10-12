use core::{mem, iter};

use thiserror::Error;

use crate::{polygon::Polygon, Color, Point, Renderable, Renderer};

#[derive(Debug, Clone, Copy)]
pub struct LineGeneralForm {
    a: i32,
    b: i32,
    c: i32,
}

impl LineGeneralForm {
    #[must_use]
    #[inline]
    pub const fn new(a: i32, b: i32, c: i32) -> Self {
        Self { a, b, c }
    }

    #[must_use]
    #[inline]
    pub const fn new_from_points(start: Point, end: Point) -> Self {
        Self::new(
            end.y - start.y,
            start.x - end.x,
            end.x * start.y - start.x * end.y,
        )
    }
}

impl From<(Point, Point)> for LineGeneralForm {
    #[inline]
    fn from(value: (Point, Point)) -> Self {
        Self::new_from_points(value.0, value.1)
    }
}

pub trait LineSegment {
    fn points(&self) -> &[Point];
    fn first_point(&self) -> Point;
    fn last_point(&self) -> Point;
}

#[derive(Debug, Clone)]
pub struct OneColorLine {
    color: Color,
    points: Vec<Point>,
}

impl OneColorLine {
    #[must_use]
    #[inline]
    pub fn new_45_deg(start: Point, end: Point, color: Color) -> Self {
        let distance_x = end.x - start.x;
        let distance_y = start.y - end.y;
        let mut decision = 2 * distance_y - distance_x;
        let mut points = Vec::new();
        let mut x = start.x;
        let mut y = start.y;

        for _ in 0..distance_x {
            points.push((x, y).into());

            if decision > 0 {
                y -= 1;
                decision += 2 * (distance_y - distance_x);
            } else {
                decision += 2 * distance_y;
            }

            x += 1;
        }

        Self { color, points }
    }

    #[must_use]
    #[inline]
    pub fn new(start: Point, end: Point, color: Color) -> Self {
        let mut distance_x = (end.x - start.x).abs();
        let mut distance_y = (start.y - end.y).abs();
        let sign_x = (end.x - start.x).signum();
        let sign_y = (start.y - end.y).signum();
        let swapped = if distance_x < distance_y {
            mem::swap(&mut distance_x, &mut distance_y);
            true
        } else {
            false
        };
        let mut decision = 2 * distance_y - distance_x;
        let mut x = start.x;
        let mut y = start.y;
        let mut points = Vec::from([(x, y).into()]);
        while x != end.x || y != end.y {
            if decision > 0 {
                if swapped {
                    x += sign_x;
                } else {
                    y -= sign_y;
                }
                decision -= 2 * distance_x;
            }

            if swapped {
                y -= sign_y;
            } else {
                x += sign_x;
            }

            decision += 2 * distance_y;
            points.push((x, y).into());
        }

        Self { color, points }
    }

    #[must_use]
    #[inline]
    pub fn new_inside_polygon<T, R>(
        start: Point,
        end: Point,
        color: Color,
        polygon: &Polygon<T, R>,
    ) -> Option<Self>
    where
        T: LineSegment + Renderable<R> + Into<LineGeneralForm>,
        R: Renderer,
    {
        let polygon_contains_start = polygon.contains(start);
        let polygon_contains_end = polygon.contains(end);

        if polygon_contains_start && polygon_contains_end {
            return Some(Self::new(start, end, color));
        }

        let general_form = LineGeneralForm::new_from_points(start, end);

        let signums: Vec<i32> = polygon
            .points()
            .iter()
            .map(|point| {
                (general_form.a * point.x
                    + general_form.b * point.y
                    + general_form.c)
                    .signum()
            })
            .collect();

        if signums.first().is_some_and(|first| {
            signums.iter().skip(1).all(|elem| first == elem)
        }) {
            return None;
        }

        dbg!(signums.len());
        dbg!(polygon.edges().len());

        let intersections: Vec<Point> = signums
            .windows(2)
            .map(|signum| (signum[0], signum[1]))
            .chain(iter::once((signums[signums.len() - 1], signums[0])))
            .zip(polygon.edges())
            .filter(|&(signum, _)| (signum.0 != signum.1))
            .map(|(_, edge)| {
                let edge_general_form = LineGeneralForm::new_from_points(
                    edge.first_point(),
                    edge.last_point(),
                );

                let x = (general_form.c * edge_general_form.b
                    - edge_general_form.c * general_form.b)
                    / (edge_general_form.a * general_form.b
                        - general_form.a * edge_general_form.b);
                let y = (general_form.c * edge_general_form.a
                    - edge_general_form.c * general_form.a)
                    / (general_form.a * edge_general_form.b
                        - edge_general_form.a * general_form.b);

                Point::new(x, y)
            })
            .collect();

        dbg!(intersections.len());

        if intersections.len() != 2 {
            return None;
        }

        let start = if polygon_contains_start {
            start
        } else {
            intersections[0]
        };

        let end = if polygon_contains_end {
            end
        } else {
            intersections[1]
        };

        Some(Self::new(start, end, color))
    }
}

impl From<OneColorLine> for LineGeneralForm {
    #[inline]
    fn from(value: OneColorLine) -> Self {
        Self::new(
            value.last_point().y - value.first_point().y,
            value.first_point().x - value.last_point().x,
            value.last_point().x * value.first_point().y
                - value.first_point().x * value.last_point().y,
        )
    }
}

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum LineDrawError<T>
where
    T: Renderer,
{
    #[error("Couldn't draw the line.")]
    Draw(T::DrawError),
    #[error("Line was empty.")]
    Empty,
}

impl<T> Renderable<T> for OneColorLine
where
    T: Renderer,
{
    type Error = LineDrawError<T>;

    #[inline]
    fn render(&self, renderer: &mut T) -> Result<(), Self::Error>
    where
        T: Renderer,
    {
        let old_color = renderer.current_color();

        if self.points.is_empty() {
            return Err(LineDrawError::Empty);
        }

        renderer.set_color(self.color);
        renderer
            .draw_points(&self.points)
            .map_err(LineDrawError::Draw)?;

        renderer.set_color(old_color);

        Ok(())
    }
}

impl LineSegment for OneColorLine {
    #[inline]
    fn points(&self) -> &[Point] {
        &self.points
    }

    #[inline]
    fn first_point(&self) -> Point {
        self.points[0]
    }

    #[inline]
    fn last_point(&self) -> Point {
        self.points[self.points.len() - 1]
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        line::{LineSegment, OneColorLine},
        polygon::Polygon,
        Color, Point, Renderer,
    };

    struct MockRenderer;

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

        fn draw_points(
            &mut self,
            points: &[Point],
        ) -> Result<(), Self::DrawError> {
            let _ = points;
            unimplemented!()
        }

        fn current_color(&self) -> Color {
            unimplemented!()
        }
    }

    #[test]
    fn line_not_inside_polygon_is_none() {
        let square: Polygon<_, MockRenderer> = Polygon::new(
            &[
                ((100, 100).into()),
                ((100, 200).into()),
                ((200, 200).into()),
                ((200, 100).into()),
            ],
            Color::RED,
        )
        .unwrap();

        let line_inside_square = OneColorLine::new_inside_polygon(
            (500, 500).into(),
            (500, 600).into(),
            Color::RED,
            &square,
        );

        assert!(line_inside_square.is_none());
    }

    #[test]
    fn line_inside_polygon_is_some() {
        let square: Polygon<_, MockRenderer> = Polygon::new(
            &[
                ((100, 100).into()),
                ((100, 200).into()),
                ((200, 200).into()),
                ((200, 100).into()),
            ],
            Color::RED,
        )
        .unwrap();

        let line_inside_square = OneColorLine::new_inside_polygon(
            (50, 150).into(),
            (250, 150).into(),
            Color::RED,
            &square,
        );

        assert!(line_inside_square.is_some());
    }

    #[test]
    fn line_inside_polygon_cuts() {
        let square: Polygon<_, MockRenderer> = Polygon::new(
            &[
                ((100, 100).into()),
                ((100, 200).into()),
                ((200, 200).into()),
                ((200, 100).into()),
            ],
            Color::RED,
        )
        .unwrap();

        let line_inside_square = OneColorLine::new_inside_polygon(
            (50, 150).into(),
            (250, 150).into(),
            Color::RED,
            &square,
        );

        assert!(line_inside_square.is_some());
        let line_inside_square = line_inside_square.unwrap();
        assert_eq!(line_inside_square.first_point(), Point::new(100, 150));
        assert_eq!(line_inside_square.last_point(), Point::new(200, 150));
    }

    #[test]
    fn line_fully_inside_polygon_is_some() {
        let square: Polygon<_, MockRenderer> = Polygon::new(
            &[
                ((100, 100).into()),
                ((100, 200).into()),
                ((200, 200).into()),
                ((200, 100).into()),
            ],
            Color::RED,
        )
        .unwrap();

        let line_inside_square = OneColorLine::new_inside_polygon(
            (125, 150).into(),
            (175, 150).into(),
            Color::RED,
            &square,
        );

        assert!(line_inside_square.is_some());
    }

    #[test]
    fn line_fully_inside_polygon_doesnt_cut() {
        let square: Polygon<_, MockRenderer> = Polygon::new(
            &[
                ((100, 100).into()),
                ((100, 200).into()),
                ((200, 200).into()),
                ((200, 100).into()),
            ],
            Color::RED,
        )
        .unwrap();

        let p1 = (125, 150).into();
        let p2 = (175, 150).into();
        let line_inside_square =
            OneColorLine::new_inside_polygon(p1, p2, Color::RED, &square);

        let line_inside_square = line_inside_square.unwrap();
        assert_eq!(line_inside_square.first_point(), p1);
        assert_eq!(line_inside_square.last_point(), p2);
    }

    #[test]
    fn line_vertically_cut_works() {
        let square: Polygon<_, MockRenderer> = Polygon::new(
            &[
                ((100, 100).into()),
                ((100, 200).into()),
                ((200, 200).into()),
                ((200, 100).into()),
            ],
            Color::RED,
        )
        .unwrap();

        let line_inside_square = OneColorLine::new_inside_polygon(
            (150, 50).into(),
            (150, 250).into(),
            Color::RED,
            &square,
        );

        assert!(line_inside_square.is_some());
        let line_inside_square = line_inside_square.unwrap();
        assert_eq!(line_inside_square.first_point(), Point::new(150, 200));
        assert_eq!(line_inside_square.last_point(), Point::new(150, 100));
    }
}
