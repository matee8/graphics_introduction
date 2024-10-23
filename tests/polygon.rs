use core::iter;

use graphics_introduction::{
    polygon::{Polygon, PolygonFromSegmentsError},
    segment::{LineSegment, OneColorSegment},
    Color, Point,
};

#[test]
fn polygon_from_one_segment_is_err() {
    let segment =
        OneColorSegment::new((100, 100).into(), (200, 200).into(), Color::RED);

    let polygon = Polygon::new_from_segments(&[segment]);

    assert!(polygon.is_err());
    assert_eq!(polygon.unwrap_err(), PolygonFromSegmentsError::NotEnough);
}

#[test]
fn polygon_from_not_touching_segment_is_err() {
    let segments = [
        OneColorSegment::new((100, 100).into(), (100, 200).into(), Color::RED),
        OneColorSegment::new((200, 100).into(), (200, 200).into(), Color::RED),
        OneColorSegment::new((300, 100).into(), (300, 200).into(), Color::RED),
    ];

    let polygon = Polygon::new_from_segments(&segments);

    assert!(polygon.is_err());
    assert_eq!(polygon.unwrap_err(), PolygonFromSegmentsError::NotTouching);
}

#[test]
fn polygon_from_four_touching_segments_is_ok() {
    let segments = [
        OneColorSegment::new((100, 100).into(), (100, 200).into(), Color::RED),
        OneColorSegment::new((100, 200).into(), (200, 200).into(), Color::RED),
        OneColorSegment::new((200, 200).into(), (200, 100).into(), Color::RED),
        OneColorSegment::new((200, 100).into(), (100, 100).into(), Color::RED),
    ];

    let polygon = Polygon::new_from_segments(&segments);

    assert!(polygon.is_ok());
}

#[test]
fn polygon_from_four_touching_segments_has_correct_points() {
    let points = [
        (100, 100).into(),
        (100, 200).into(),
        (200, 200).into(),
        (200, 100).into(),
    ];

    let segments: Vec<OneColorSegment> = points
        .windows(2)
        .map(|points| (points[0], points[1]))
        .chain(iter::once((points[points.len() - 1], points[0])))
        .map(|(start, end)| OneColorSegment::new(start, end, Color::RED))
        .collect();

    let polygon = Polygon::new_from_segments(&segments).unwrap();

    assert_eq!(polygon.vertices(), points);
}

fn create_polygon() -> Polygon<OneColorSegment> {
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
fn segment_not_inside_polygon_is_err() {
    let square = create_polygon();

    let segment_inside_square = OneColorSegment::new_inside_polygon(
        (500, 500).into(),
        (500, 600).into(),
        Color::RED,
        &square,
    );

    assert!(segment_inside_square.is_err());
}

#[test]
fn segment_inside_polygon_is_ok() {
    let square = create_polygon();

    let segment_inside_square = OneColorSegment::new_inside_polygon(
        (50, 150).into(),
        (250, 150).into(),
        Color::RED,
        &square,
    );

    assert!(segment_inside_square.is_ok());
}

#[test]
fn segment_inside_polygon_cuts() {
    let square = create_polygon();

    let segment_inside_square = OneColorSegment::new_inside_polygon(
        (50, 150).into(),
        (250, 150).into(),
        Color::RED,
        &square,
    )
    .unwrap();

    assert_eq!(
        segment_inside_square.first_point(),
        Point::new(100.0, 150.0)
    );
    assert_eq!(segment_inside_square.last_point(), Point::new(200.0, 150.0));
}

#[test]
fn segment_fully_inside_polygon_is_ok() {
    let square = create_polygon();

    let segment_inside_square = OneColorSegment::new_inside_polygon(
        (125, 150).into(),
        (175, 150).into(),
        Color::RED,
        &square,
    );

    assert!(segment_inside_square.is_ok());
}

#[test]
fn segment_fully_inside_polygon_doesnt_cut() {
    let square = create_polygon();

    let p1 = (125, 150).into();
    let p2 = (175, 150).into();
    let segment_inside_square =
        OneColorSegment::new_inside_polygon(p1, p2, Color::RED, &square)
            .unwrap();

    assert_eq!(segment_inside_square.first_point(), p1);
    assert_eq!(segment_inside_square.last_point(), p2);
}

#[test]
fn segment_vertically_cut_works() {
    let square = create_polygon();

    let segment_inside_square = OneColorSegment::new_inside_polygon(
        (150, 50).into(),
        (150, 250).into(),
        Color::RED,
        &square,
    )
    .unwrap();

    assert_eq!(
        segment_inside_square.first_point(),
        Point::new(150.0, 200.0)
    );
    assert_eq!(segment_inside_square.last_point(), Point::new(150.0, 100.0));
}

#[test]
fn segment_cut_after_creation_is_ok() {
    let square = create_polygon();

    let segment_inside_square =
        OneColorSegment::new((150, 50).into(), (150, 250).into(), Color::RED)
            .cut_inside_polygon(&square);

    assert!(segment_inside_square.is_ok());
}

#[test]
fn segment_cut_after_creation_works() {
    let square = create_polygon();

    let mut segment_inside_square =
        OneColorSegment::new((150, 50).into(), (150, 250).into(), Color::RED);

    let res = segment_inside_square.cut_inside_polygon(&square);

    assert!(res.is_ok());
    assert_eq!(
        segment_inside_square.first_point(),
        Point::new(150.0, 100.0)
    );
    assert_eq!(segment_inside_square.last_point(), Point::new(150.0, 200.0));
}
