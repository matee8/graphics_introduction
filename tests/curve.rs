use graphics_introduction::{
    curve::{CurveFromSegmentsError, OneColorCurve},
    segment::OneColorSegment,
    Color, GeometricPrimitive,
};

#[test]
fn curve_from_one_segment_is_err() {
    let segment =
        OneColorSegment::new((100, 100).into(), (100, 200).into(), Color::RED);

    let curve = OneColorCurve::from_segments(&[segment], Color::RED);

    assert!(curve.is_err());
    assert_eq!(curve.unwrap_err(), CurveFromSegmentsError::NotEnough);
}

#[test]
fn curve_from_not_touching_segments_is_err() {
    let segments = [
        OneColorSegment::new((100, 100).into(), (100, 200).into(), Color::RED),
        OneColorSegment::new((200, 100).into(), (200, 200).into(), Color::RED),
        OneColorSegment::new((300, 100).into(), (300, 200).into(), Color::RED),
    ];

    let curve = OneColorCurve::from_segments(&segments, Color::RED);

    assert!(curve.is_err());
    assert_eq!(curve.unwrap_err(), CurveFromSegmentsError::NotTouching);
}

#[test]
fn curve_from_four_touching_segments_is_ok() {
    let segments = [
        OneColorSegment::new((100, 100).into(), (100, 200).into(), Color::RED),
        OneColorSegment::new((100, 200).into(), (200, 200).into(), Color::RED),
        OneColorSegment::new((200, 200).into(), (200, 100).into(), Color::RED),
        OneColorSegment::new((200, 100).into(), (100, 100).into(), Color::RED),
    ];

    let curve = OneColorCurve::from_segments(&segments, Color::RED);

    assert!(curve.is_ok());
}

#[test]
fn curve_from_four_touching_segments_has_correct_start_and_end_points() {
    let points = [
        (100, 100).into(),
        (100, 200).into(),
        (200, 200).into(),
        (200, 300).into(),
    ];

    let segments: Vec<OneColorSegment> = points
        .windows(2)
        .map(|points| (points[0], points[1]))
        .map(|(start, end)| OneColorSegment::new(start, end, Color::RED))
        .collect();

    let curve = OneColorCurve::from_segments(&segments, Color::RED).unwrap();

    assert_eq!(curve.points().first(), points.first());
    assert_eq!(curve.points().iter().last(), points.iter().last());
}
