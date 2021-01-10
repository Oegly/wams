use crate::physics::Vector;

pub const EPSILON: f64 = 0.001;

#[derive(Clone,Copy,Debug,PartialEq)]
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

impl From<Vector> for Point {
    fn from(v: Vector) -> Point {
        Point::new(v.get_dx(), v.get_dy())
    }
}

impl From<(f64, f64)> for Point {
    fn from(t: (f64, f64)) -> Point {
        Point::new(t.0, t.1)
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::SubAssign for Point {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}

use std::fmt;

impl fmt::Display for Point {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "x: {:.2}, y: {:.2}", self.x, self.y)
    }
}