use core::{iter, mem};

use thiserror::Error;

use crate::{
    polygon::Polygon, Color, Point, Renderable, Renderer, ERROR_MARGIN,
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Line {
    a: f64,
    b: f64,
    c: f64,
}

impl Line {
    #[must_use]
    #[inline]
    pub const fn new(a: f64, b: f64, c: f64) -> Self {
        Self { a, b, c }
    }

    #[must_use]
    #[inline]
    pub fn new_from_points(start: Point, end: Point) -> Self {
        Self::new(
            end.y - start.y,
            start.x - end.x,
            end.x.mul_add(start.y, -(start.x * end.y)),
        )
    }

    #[must_use]
    #[inline]
    pub fn intersection(&self, other: &Self) -> Point {
        let x = self.c.mul_add(other.b, -(other.c * self.b))
            / other.a.mul_add(self.b, -(self.a * other.b));
        let y = self.c.mul_add(other.a, -(other.c * self.a))
            / self.a.mul_add(other.b, -(other.a * self.b));

        Point::new(x, y)
    }
}

impl From<(Point, Point)> for Line {
    #[inline]
    fn from(value: (Point, Point)) -> Self {
        Self::new_from_points(value.0, value.1)
    }
}

pub trait GeometricPrimitve {
    fn points(&self) -> &[Point];

    #[inline]
    fn first_point(&self) -> Point {
        #[expect(
            clippy::indexing_slicing,
            reason = "A geometric primitive cannot be created without points."
        )]
        self.points()[0]
    }

    #[inline]
    fn last_point(&self) -> Point {
        #[expect(
            clippy::indexing_slicing,
            reason = "A geometric primitive cannot be created without points."
        )]
        self.points()[self.points().len() - 1]
    }

    #[inline]
    fn length(&self) -> usize {
        self.points().len()
    }
}

pub trait LineSegment: GeometricPrimitve {}

#[derive(Debug, Clone, PartialEq)]
pub struct OneColorSegment {
    color: Color,
    points: Vec<Point>,
}

#[non_exhaustive]
#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[error("Points are too far apart.")]
pub struct InvalidPoints;

#[non_exhaustive]
#[derive(Debug, Error, Clone, Copy)]
pub enum CutSegmentInsidePolygonError {
    #[error("Segment is fully outside polygon.")]
    Outside,
    #[error(
        "Not enough intersections between the segment and the polygon's edges."
    )]
    NotEnoughIntersections,
    #[error("Intersection with polygon isn't in the original segment.")]
    InvalidIntersection,
    #[error("One of the signums is NaN.")]
    InvalidSegments,
}

impl OneColorSegment {
    #[inline]
    pub fn new_45_deg(
        start: Point,
        end: Point,
        color: Color,
    ) -> Result<Self, InvalidPoints> {
        let distance_x = end.x - start.x;
        let distance_y = start.y - end.y;
        let mut decision = 2.0_f64.mul_add(distance_y, -distance_x);
        let mut points = Vec::new();
        let mut x = start.x;
        let mut y = start.y;

        let loop_condition = if distance_x > f64::from(i32::MAX)
            || distance_x < f64::from(i32::MIN)
            || distance_x.is_nan()
            || distance_x.is_infinite()
        {
            Err(InvalidPoints)
        } else {
            #[expect(
                clippy::cast_possible_truncation,
                clippy::as_conversions,
                reason = "If distance_x is invalid as i32, the function returns early."
            )]
            Ok(distance_x as i32)
        }?;

        for _ in 0..loop_condition {
            points.push((x, y).into());

            if decision > 0.0 {
                y -= 1.0;
                decision += 2.0 * (distance_y - distance_x);
            } else {
                decision += 2.0 * distance_y;
            }

            x += 1.0;
        }

        Ok(Self { color, points })
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
        let mut decision = 2.0_f64.mul_add(distance_y, -distance_x);
        let mut x = start.x;
        let mut y = start.y;
        let mut points = Vec::from([(x, y).into()]);
        while (x - end.x).abs() > ERROR_MARGIN
            || (y - end.y).abs() > ERROR_MARGIN
        {
            if decision > 0.0 {
                if swapped {
                    x += sign_x;
                } else {
                    y -= sign_y;
                }
                decision -= 2.0 * distance_x;
            }

            if swapped {
                y -= sign_y;
            } else {
                x += sign_x;
            }

            decision += 2.0 * distance_y;
            points.push((x, y).into());
        }

        Self { color, points }
    }

    #[inline]
    pub fn new_inside_polygon<T>(
        start: Point,
        end: Point,
        color: Color,
        polygon: &Polygon<T>,
    ) -> Result<Self, CutSegmentInsidePolygonError>
    where
        T: LineSegment + Into<Line>,
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
    pub fn cut_inside_polygon<T>(
        &mut self,
        polygon: &Polygon<T>,
    ) -> Result<(), CutSegmentInsidePolygonError>
    where
        T: LineSegment + Into<Line>,
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
            .ok_or(CutSegmentInsidePolygonError::InvalidIntersection)?;
        self.points.drain(..index);

        let index = self
            .points
            .iter()
            .rposition(|point| *point == end || *point == start)
            .ok_or(CutSegmentInsidePolygonError::InvalidIntersection)?;
        self.points.drain(index + 1..);

        Ok(())
    }

    fn get_start_end_inside_polygon<T>(
        start: Point,
        end: Point,
        polygon: &Polygon<T>,
    ) -> Result<(Point, Point), CutSegmentInsidePolygonError>
    where
        T: LineSegment + Into<Line>,
    {
        let polygon_contains_start = polygon.contains(start);
        let mut polygon_contains_end = None;

        if polygon_contains_start {
            polygon_contains_end = Some(polygon.contains(end));
            if polygon_contains_end == Some(true) {
                return Ok((start, end));
            }
        }

        let line = Line::new_from_points(start, end);

        let signums: Vec<i8> = polygon
            .vertices()
            .iter()
            .map(|point| {
                let signum =
                    (line.a.mul_add(point.x, line.b * point.y)
                        + line.c)
                        .signum();

                #[expect(
                    clippy::cast_possible_truncation,
                    clippy::as_conversions,
                    reason = "NaN case is handled as an error, otherwise f64::signum either returns -1, 0 or 1."
                )]
                if signum.is_nan() {
                    Err(CutSegmentInsidePolygonError::InvalidSegments)
                } else {
                    Ok(signum as i8)
                }
            })
            .collect::<Result<_, _>>()?;

        if signums.first().is_some_and(|first| {
            signums.iter().skip(1).all(|elem| first == elem)
        }) {
            return Err(CutSegmentInsidePolygonError::Outside);
        }

        #[expect(
            clippy::indexing_slicing,
            reason = "slice::windows only panics if size is 0, but we can't create a polygon with 0 sides."
        )]
        let intersections: Vec<Point> = signums
            .windows(2)
            .map(|signum| (signum[0], signum[1]))
            .chain(iter::once((signums[signums.len() - 1], signums[0])))
            .zip(polygon.edges())
            .filter(|&(signum, _)| (signum.0 != signum.1))
            .map(|(_, edge)| {
                let edge_line = Line::new_from_points(
                    edge.first_point(),
                    edge.last_point(),
                );

                line.intersection(&edge_line)
            })
            .collect();

        let start = if polygon_contains_start {
            start
        } else {
            let Some(intersection) = intersections.first() else {
                return Err(
                    CutSegmentInsidePolygonError::NotEnoughIntersections,
                );
            };
            *intersection
        };

        let polygon_contains_end = polygon_contains_end
            .map_or_else(|| polygon.contains(end), |old_value| old_value);

        let end = if polygon_contains_end {
            end
        } else {
            let Some(intersection) = intersections.get(1) else {
                return Err(
                    CutSegmentInsidePolygonError::NotEnoughIntersections,
                );
            };
            *intersection
        };

        Ok((start, end))
    }
}

impl From<OneColorSegment> for Line {
    #[inline]
    fn from(value: OneColorSegment) -> Self {
        Self::new(
            value.last_point().y - value.first_point().y,
            value.first_point().x - value.last_point().x,
            value.last_point().x.mul_add(
                value.first_point().y,
                -(value.first_point().x * value.last_point().y),
            ),
        )
    }
}

#[non_exhaustive]
#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SegmentDrawError<T>
where
    T: Renderer,
{
    #[error("Couldn't draw the segment.")]
    Draw(T::DrawError),
    #[error("Segment was empty.")]
    Empty,
}

impl<T> Renderable<T> for OneColorSegment
where
    T: Renderer,
{
    type Error = SegmentDrawError<T>;

    #[inline]
    fn render(&self, renderer: &mut T) -> Result<(), Self::Error>
    where
        T: Renderer,
    {
        let old_color = renderer.current_color();

        if self.points.is_empty() {
            return Err(SegmentDrawError::Empty);
        }

        renderer.set_color(self.color);
        renderer
            .draw_points(&self.points)
            .map_err(SegmentDrawError::Draw)?;

        renderer.set_color(old_color);

        Ok(())
    }
}

impl GeometricPrimitve for OneColorSegment {
    #[inline]
    fn points(&self) -> &[Point] {
        &self.points
    }
}

impl LineSegment for OneColorSegment {}

#[cfg(test)]
mod tests {
    use crate::{
        segment::{GeometricPrimitve, Line, OneColorSegment},
        Color,
    };

    #[test]
    fn new_segment_has_correct_start_and_end_points() {
        let start = (100, 100).into();
        let end = (100, 200).into();

        let segment = OneColorSegment::new(start, end, Color::RED);

        assert_eq!(segment.first_point(), start);
        assert_eq!(segment.last_point(), end);
    }

    #[test]
    fn new_segment_has_correct_general_form() {
        let start = (100, 100).into();
        let end = (100, 200).into();

        let line = Line::new_from_points(start, end);
        let segment_line: Line =
            OneColorSegment::new(start, end, Color::RED).into();

        assert_eq!(segment_line, line);
    }
}
