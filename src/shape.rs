use crate::physics::*;

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
        false
    }
}

#[derive(Clone,Copy,Debug)]
pub struct Circle {
    x: f64,
    y: f64,
    r: f64,
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

    fn check_collision_rectangle(&self, rect: &Rectangle) -> bool {
        false
    }

    fn check_collision_circle(&self, circle: &Circle) -> bool {
        let dx = self.x - circle.get_x();
        let dy = self.y - circle.get_y();

        let distance = dx.hypot(dy);

        if distance < self.r + circle.get_r() {
            return true;
        }

        false
    }
}
