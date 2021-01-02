use crate::physics::{Circle,Point,Shape,Vector};
use crate::physics::collision::{
    check_collision_point_shape
};

#[derive(Clone,Copy,Debug)]
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
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

    pub fn move_by(&mut self, x: f64, y: f64) {
        self.x += x;
        self.y += y;
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

    fn check_collision_point(&self, point: &Point) -> bool {
        check_collision_point_shape(point, self)
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