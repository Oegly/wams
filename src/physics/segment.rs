use crate::physics::{Circle,Point,Vector};
use crate::physics::collision::{
    check_collision_point_segment,
    check_collision_segment_circle
};

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
        Segment::new(Point::from(t0), Point::from(t1))
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
        self.get_dy().atan2(self.get_dx())
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