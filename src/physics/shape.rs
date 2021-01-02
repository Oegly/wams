use crate::physics::{Circle,Point,Rectangle};

pub trait Shape {
    fn top(&self) -> f64;
    fn right(&self) -> f64;
    fn bottom(&self) -> f64;
    fn left(&self) -> f64;

    fn check_collision_point(&self, point: &Point) -> bool;
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