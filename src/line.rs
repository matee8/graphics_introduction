use core::{iter, mem};

use thiserror::Error;

use crate::{polygon::Polygon, Color, Point, Renderable, Renderer};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

    #[must_use]
    #[inline]
    pub const fn intersection(&self, other: &Self) -> Point {
        let x = (self.c * other.b - other.c * self.b)
            / (other.a * self.b - self.a * other.b);
        let y = (self.c * other.a - other.c * self.a)
            / (self.a * other.b - other.a * self.b);

        Point::new(x, y)
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OneColorLine {
    color: Color,
    points: Vec<Point>,
}

#[non_exhaustive]
#[derive(Debug, Error, Clone, Copy)]
pub enum CutLineInsidePolygonError {
    #[error("Line is fully outside polygon.")]
    Outside,
    #[error(
        "Not enough intersections between the line and the polygon's edges."
    )]
    NotEnoughIntersections,
    #[error("Intersection with polygon isn't in the original line.")]
    InvalidIntersection,
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

    #[inline]
    pub fn new_inside_polygon<T, R>(
        start: Point,
        end: Point,
        color: Color,
        polygon: &Polygon<T, R>,
    ) -> Result<Self, CutLineInsidePolygonError>
    where
        T: LineSegment + Renderable<R> + Into<LineGeneralForm>,
        R: Renderer,
    {
        let (start, end) =
            Self::get_start_end_inside_polygon(start, end, polygon)?;

        Ok(Self::new(start, end, color))
    }

    #[must_use]
    #[inline]
    pub const fn color(&self) -> Color {
        self.color
    }

    #[inline]
    pub fn cut_inside_polygon<T, R>(
        &mut self,
        polygon: &Polygon<T, R>,
    ) -> Result<(), CutLineInsidePolygonError>
    where
        T: Renderable<R> + LineSegment + Into<LineGeneralForm>,
        R: Renderer,
    {
        let (start, end) = Self::get_start_end_inside_polygon(
            self.first_point(),
            self.last_point(),
            polygon,
        )?;

        let index = self
            .points
            .iter()
            .position(|point| *point == start || *point == end)
            .ok_or(CutLineInsidePolygonError::InvalidIntersection)?;
        self.points.drain(..index);

        let index = self
            .points
            .iter()
            .rposition(|point| *point == end || *point == start)
            .ok_or(CutLineInsidePolygonError::InvalidIntersection)?;
        self.points.drain(index + 1..);

        Ok(())
    }

    fn get_start_end_inside_polygon<T, R>(
        start: Point,
        end: Point,
        polygon: &Polygon<T, R>,
    ) -> Result<(Point, Point), CutLineInsidePolygonError>
    where
        T: Renderable<R> + LineSegment + Into<LineGeneralForm>,
        R: Renderer,
    {
        let polygon_contains_start = polygon.contains(start);
        let mut polygon_contains_end = None;

        if polygon_contains_start {
            polygon_contains_end = Some(polygon.contains(end));
            if polygon_contains_end == Some(true) {
                return Ok((start, end));
            }
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
            return Err(CutLineInsidePolygonError::Outside);
        }

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

                general_form.intersection(&edge_general_form)
            })
            .collect();

        if intersections.len() != 2 {
            return Err(CutLineInsidePolygonError::NotEnoughIntersections);
        }

        let start = if polygon_contains_start {
            start
        } else {
            intersections[0]
        };

        let end = polygon_contains_end.map_or_else(
            || {
                if polygon.contains(end) {
                    end
                } else {
                    intersections[1]
                }
            },
            |flag| if flag { end } else { intersections[1] },
        );

        Ok((start, end))
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
        line::{LineSegment, OneColorLine, LineGeneralForm},
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
    fn new_line_has_correct_start_and_end_points() {
        let start = (100, 100).into();
        let end = (100, 200).into();

        let line = OneColorLine::new(start, end, Color::RED);

        assert_eq!(line.first_point(), start);
        assert_eq!(line.last_point(), end);
    }

    #[test]
    fn new_line_has_correct_general_form() {
        let start = (100, 100).into();
        let end = (100, 200).into();

        let general_form = LineGeneralForm::new_from_points(start, end);
        let line: LineGeneralForm =
            OneColorLine::new(start, end, Color::RED).into();

        assert_eq!(line, general_form);
    }

    #[test]
    fn line_not_inside_polygon_is_err() {
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

        assert!(line_inside_square.is_err());
    }

    #[test]
    fn line_inside_polygon_is_ok() {
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

        assert!(line_inside_square.is_ok());
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

        assert!(line_inside_square.is_ok());
        let line_inside_square = line_inside_square.unwrap();
        assert_eq!(line_inside_square.first_point(), Point::new(100, 150));
        assert_eq!(line_inside_square.last_point(), Point::new(200, 150));
    }

    #[test]
    fn line_fully_inside_polygon_is_ok() {
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

        assert!(line_inside_square.is_ok());
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

        assert!(line_inside_square.is_ok());
        let line_inside_square = line_inside_square.unwrap();
        assert_eq!(line_inside_square.first_point(), Point::new(150, 200));
        assert_eq!(line_inside_square.last_point(), Point::new(150, 100));
    }

    #[test]
    fn line_cut_after_creation_is_ok() {
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

        let line_inside_square =
            OneColorLine::new((150, 50).into(), (150, 250).into(), Color::RED)
                .cut_inside_polygon(&square);

        dbg!(&line_inside_square);
        assert!(line_inside_square.is_ok());
    }

    #[test]
    fn line_cut_after_creation_works() {
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

        let mut line_inside_square =
            OneColorLine::new((150, 50).into(), (150, 250).into(), Color::RED);

        let res = line_inside_square.cut_inside_polygon(&square);

        assert!(res.is_ok());
        assert_eq!(line_inside_square.first_point(), Point::new(150, 100));
        assert_eq!(line_inside_square.last_point(), Point::new(150, 200));
    }
}
