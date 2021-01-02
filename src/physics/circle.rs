use crate::physics::{Point,Rectangle,Segment,Shape,Vector};
use crate::physics::collision::{
    check_collison_circle_circle,
    check_collision_point_circle,
    check_collision_segment_circle
};

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

    fn check_collision_point(&self, point: &Point) -> bool {
        check_collision_point_circle(point, self)
    }

    fn check_collision_rectangle(&self, rect: &Rectangle) -> bool {
        //TODO
        false
    }

    fn check_collision_circle(&self, circle: &Circle) -> bool {
        check_collison_circle_circle(self, circle)
    }
}

