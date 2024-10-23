use core::iter;

use graphics_introduction::{
    line::{LineSegment, OneColorLine},
    polygon::{Polygon, PolygonFromLinesError},
    Color, Point,
};

#[test]
fn polygon_from_one_line_is_error() {
    let line =
        OneColorLine::new((100, 100).into(), (200, 200).into(), Color::RED);

    let polygon = Polygon::new_from_lines(&[line]);

    assert!(polygon.is_err());
    assert_eq!(polygon.unwrap_err(), PolygonFromLinesError::NotEnoughLines);
}

#[test]
fn polygon_from_not_touching_lines_is_err() {
    let lines = [
        OneColorLine::new((100, 100).into(), (100, 200).into(), Color::RED),
        OneColorLine::new((200, 100).into(), (200, 200).into(), Color::RED),
        OneColorLine::new((300, 100).into(), (300, 200).into(), Color::RED),
    ];

    let polygon = Polygon::new_from_lines(&lines);

    assert!(polygon.is_err());
    assert_eq!(polygon.unwrap_err(), PolygonFromLinesError::LinesDontTouch);
}

#[test]
fn polygon_from_four_touching_lines_is_ok() {
    let lines = [
        OneColorLine::new((100, 100).into(), (100, 200).into(), Color::RED),
        OneColorLine::new((100, 200).into(), (200, 200).into(), Color::RED),
        OneColorLine::new((200, 200).into(), (200, 100).into(), Color::RED),
        OneColorLine::new((200, 100).into(), (100, 100).into(), Color::RED),
    ];

    let polygon = Polygon::new_from_lines(&lines);

    assert!(polygon.is_ok());
}

#[test]
fn polygon_from_four_touching_lines_has_correct_points() {
    let points = [
        (100, 100).into(),
        (100, 200).into(),
        (200, 200).into(),
        (200, 100).into(),
    ];

    let lines: Vec<OneColorLine> = points
        .windows(2)
        .map(|points| (points[0], points[1]))
        .chain(iter::once((points[points.len() - 1], points[0])))
        .map(|(start, end)| OneColorLine::new(start, end, Color::RED))
        .collect();

    let polygon = Polygon::new_from_lines(&lines).unwrap();

    assert_eq!(polygon.points(), points);
}

fn create_polygon() -> Polygon<OneColorLine> {
    Polygon::new(
        &[
            ((100, 100).into()),
            ((100, 200).into()),
            ((200, 200).into()),
            ((200, 100).into()),
        ],
        Color::RED,
    )
    .unwrap()
}

#[test]
fn line_not_inside_polygon_is_err() {
    let square = create_polygon();

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
    let square = create_polygon();

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
    let square = create_polygon();

    let line_inside_square = OneColorLine::new_inside_polygon(
        (50, 150).into(),
        (250, 150).into(),
        Color::RED,
        &square,
    );

    assert!(line_inside_square.is_ok());
    let line_inside_square = line_inside_square.unwrap();
    assert_eq!(line_inside_square.first_point(), Point::new(100.0, 150.0));
    assert_eq!(line_inside_square.last_point(), Point::new(200.0, 150.0));
}

#[test]
fn line_fully_inside_polygon_is_ok() {
    let square = create_polygon();

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
    let square = create_polygon();

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
    let square = create_polygon();

    let line_inside_square = OneColorLine::new_inside_polygon(
        (150, 50).into(),
        (150, 250).into(),
        Color::RED,
        &square,
    );

    assert!(line_inside_square.is_ok());
    let line_inside_square = line_inside_square.unwrap();
    assert_eq!(line_inside_square.first_point(), Point::new(150.0, 200.0));
    assert_eq!(line_inside_square.last_point(), Point::new(150.0, 100.0));
}

#[test]
fn line_cut_after_creation_is_ok() {
    let square = create_polygon();

    let line_inside_square =
        OneColorLine::new((150, 50).into(), (150, 250).into(), Color::RED)
            .cut_inside_polygon(&square);

    assert!(line_inside_square.is_ok());
}

#[test]
fn line_cut_after_creation_works() {
    let square = create_polygon();

    let mut line_inside_square =
        OneColorLine::new((150, 50).into(), (150, 250).into(), Color::RED);

    let res = line_inside_square.cut_inside_polygon(&square);

    assert!(res.is_ok());
    assert_eq!(line_inside_square.first_point(), Point::new(150.0, 100.0));
    assert_eq!(line_inside_square.last_point(), Point::new(150.0, 200.0));
}
