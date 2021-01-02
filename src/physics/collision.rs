use crate::physics::{Circle,Point,Rectangle,Segment,Shape};
use crate::physics::point::EPSILON;

pub fn check_collision_point_point(a: &Point, b: &Point) -> bool {
    (a.get_x() - b.get_x()).abs() >= EPSILON &&
    (a.get_y() - b.get_y()).abs() >= EPSILON
}

pub fn check_collision_point_shape(point: &Point, shape: &dyn Shape) -> bool {
    if (point.x < shape.right() &&
        point.x > shape.left() &&
        point.y < shape.bottom() &&
        point.y > shape.top()) {
        return true;
    }

    false
}

pub fn check_collision_point_segment(point: &Point, segment: &Segment) -> bool {
    let distance = point.distance(segment.point0) + point.distance(segment.point1);
    (segment.get_length() - distance).abs() <= EPSILON
}

pub fn check_collision_point_circle(point: &Point, circle: &Circle) -> bool {
    point.distance(Point::new(circle.x, circle.y)) < circle.r
}

pub fn check_collision_segment_circle(segment: &Segment, circle: &Circle) -> bool {
    if circle.check_collision_point(&segment.point0) || circle.check_collision_point(&segment.point1) {
        return true;
    }

    let dot = (((circle.x - segment.point0.x) * (segment.point1.x - segment.point0.x)) +
               ((circle.y - segment.point0.y) * (segment.point1.y - segment.point0.y))) /
        segment.get_length().powf(2.0);

    let p = Point::new(
        segment.point0.x + dot * (segment.point1.x - segment.point0.x),
        segment.point0.y + dot * (segment.point1.y - segment.point0.y)
    );

    circle.check_collision_point(&p) && segment.check_collision_point(&p)
}

pub fn check_collison_circle_circle(a: &Circle, b: &Circle) -> bool {
    let dx = a.get_x() - b.get_x();
    let dy = a.get_y() - b.get_y();

    let distance = dx.hypot(dy);

    if distance < a.get_r() + b.get_r() {
        return true;
    }

    false
}