use std::collections::HashMap;

use crate::physics::*;
use crate::shape::*;

use std::f64::consts::{PI,FRAC_PI_2};

pub const TAU: f64 = PI * 2.0;

pub struct Asteroid {
    circle: Circle,
    elasticity: f64,
}

impl Asteroid {
    pub fn new(x: f64, y: f64, r: f64) -> Asteroid {
        Asteroid {
            circle: Circle::new(x, y, r),
            elasticity: 1.0,
        }
    }

    pub fn get_circle(&self) -> Circle {
        self.circle
    }

    pub fn get_elasticity(&self) -> f64 {
        self.elasticity
    }

    pub fn render_piston(&self) -> [f64; 3] {
        [self.circle.get_x(), self.circle.get_y(), self.circle.get_r()]
    }
}