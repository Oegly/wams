use crate::physics::*;

const POINT_EPSILON: f64 = 0.01;

fn check_collision_point_point(a: &Point, b: &Point) -> bool {
    (a.get_x() - b.get_x()).abs() >= POINT_EPSILON &&
    (a.get_y() - b.get_y()).abs() >= POINT_EPSILON
}

fn check_collision_point_segment(point: &Point, segment: &Segment) -> bool {
    let distance = point.distance(segment.point0) + point.distance(segment.point1);
    (segment.get_length() - distance).abs() <= POINT_EPSILON
}

fn check_collision_point_circle(point: &Point, circle: &Circle) -> bool {
    point.distance(Point::new(circle.x, circle.y)) < circle.r
}

fn check_collision_segment_circle(segment: &Segment, circle: &Circle) -> bool {
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

fn check_collison_circle_circle(a: &Circle, b: &Circle) -> bool {
    let dx = a.get_x() - b.get_x();
    let dy = a.get_y() - b.get_y();

    let distance = dx.hypot(dy);

    if distance < a.get_r() + b.get_r() {
        return true;
    }

    false
}

pub trait Shape {
    fn top(&self) -> f64;
    fn right(&self) -> f64;
    fn bottom(&self) -> f64;
    fn left(&self) -> f64;

    fn check_collision_rectangle(&self, rect: &Rectangle) -> bool;
    fn check_collision_circle(&self, circle: &Circle) -> bool;

    fn check_collision_shape(&self, shape: &dyn Shape) -> bool {
        if (self.left() < shape.right() &&
            self.right() > shape.left() &&
            self.top() < shape.bottom() &&
            self.bottom() > shape.top()) {
            return true;
        }

        false
    }
}

#[derive(Clone,Copy,Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point {
            x: x,
            y: y,
        }
    }

    pub fn from_tuple(t: (f64, f64)) -> Point {
        Point::new(t.0, t.1)
    }

    pub fn get_x(&self) -> f64 {
        self.x
    }

    pub fn get_y(&self) -> f64 {
        self.y
    }

    pub fn distance(&self, point: Point) -> f64 {
        (self.x - point.x).abs().hypot((self.y - point.y).abs())
    }
}

#[derive(Clone,Copy,Debug)]
pub struct Segment {
    pub point0: Point,
    pub point1: Point,
}

impl Segment {
    pub fn new(p0: Point, p1: Point) -> Segment {
        Segment {
            point0: p0,
            point1: p1,
        }
    }

    pub fn from_tuples(t0: (f64, f64), t1: (f64, f64)) -> Segment {
        Segment::new(Point::from_tuple(t0), Point::from_tuple(t1))
    }

    pub fn from_vector(x: f64, y: f64, vector: Vector) -> Segment {
        Segment {
            point0: Point::new(x, y),
            point1: Point::new(x + vector.get_dx(), y + vector.get_dy())
        }
    }

    pub fn get_dx(&self) -> f64 {
        self.point1.x - self.point0.x
    }

    pub fn get_dy(&self) -> f64 {
        self.point1.y - self.point0.y
    }

    pub fn get_direction(&self) -> f64 {
        self.get_dx().atan2(self.get_dy())
    }

    pub fn get_length(&self) -> f64 {
        self.get_dx().hypot(self.get_dy())
    }

    pub fn check_collision_point(&self, point: &Point) -> bool {
        check_collision_point_segment(point, self)
    }

    pub fn check_collision_circle(&self, circle: &Circle) -> bool {
        check_collision_segment_circle(self, circle)
    }

}

#[derive(Clone,Copy,Debug)]
pub struct Rectangle {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

impl Rectangle {
    pub fn new(x: f64, y: f64, w: f64, h: f64) -> Rectangle {
        Rectangle {
            x: x,
            y: y,
            width: w,
            height: h,
        }
    }

    pub fn from_bounds(top: f64, right: f64, bottom: f64, left: f64) -> Rectangle {
        Rectangle {
            x: left,
            y: top,
            width: right - left,
            height: bottom - top,
        }
    }

    pub fn render_piston(&self) -> [f64; 4] {
        [self.x, self.y, self.width, self.height]
    }
}

impl Shape for Rectangle {
    fn top(&self) -> f64 {
        self.y
    }

    fn right(&self) -> f64 {
        self.x + self.width
    }

    fn bottom(&self) -> f64 {
        self.y + self.height
    }

    fn left(&self) -> f64 {
        self.x
    }

    fn check_collision_rectangle(&self, rect: &Rectangle) -> bool {
        if (self.left() < rect.right() &&
            self.right() > rect.left() &&
            self.top() < rect.bottom() &&
            self.bottom() > rect.top()) {
            return true;
        }

        false
    }

    fn check_collision_circle(&self, circle: &Circle) -> bool {
        //TODO
        false
    }
}

#[derive(Clone,Copy,Debug)]
pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub r: f64,
}

impl Circle {
    pub fn new(x: f64, y: f64, r: f64) -> Circle {
        Circle {
            x: x,
            y: y,
            r: r,
        }
    }

    pub fn get_x(&self) -> f64 {
        self.x
    }

    pub fn get_y(&self) -> f64{
        self.y
    }

    pub fn get_r(&self) -> f64 {
        self.r
    }

    pub fn move_by(&mut self, x: f64, y: f64) {
        self.x += x;
        self.y += y;
    }

    pub fn move_by_vector(&mut self, v: Vector) {
        self.x += v.get_dx();
        self.y += v.get_dy();
    }

    pub fn check_collision_point(&self, point: &Point) -> bool {
        check_collision_point_circle(point, self)
    }
}

impl Shape for Circle {
    fn top(&self) -> f64 {
        self.y - self.r
    }

    fn right(&self) -> f64 {
        self.x + self.r
    }

    fn bottom(&self) -> f64 {
        self.y + self.r
    }

    fn left(&self) -> f64 {
        self.x - self.r
    }

    fn check_collision_rectangle(&self, rect: &Rectangle) -> bool {
        //TODO
        false
    }

    fn check_collision_circle(&self, circle: &Circle) -> bool {
        check_collison_circle_circle(self, circle)
    }
}
