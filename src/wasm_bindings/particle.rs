use core::f64::consts::PI;

use wasm_bindgen::prelude::*;

use crate::physics::*;

pub struct Particle {
    pub x: f64,
    pub y: f64,
    pub vector: Vector,
    pub color: JsValue,
    pub size: f64,
    pub elapsed: f64,
    pub lifetime: f64,
}

impl Particle {
    pub fn new(x: f64, y: f64, vector: Vector, size: f64, lifetime: f64, color: JsValue,) -> Particle {
        Particle {
            x: x,
            y: y,
            vector: vector,
            size: size,
            elapsed: 0.0,
            lifetime: lifetime,
            color: color,
        }
    }

    pub fn tick(&mut self, time_delta: f64) {
        self.elapsed += time_delta;
        self.x += self.vector.get_dx() * time_delta;
        self.y += self.vector.get_dy() * time_delta;
        
    }

    pub fn get_size(&self) -> f64 {
        self.size * ((self.lifetime - self.elapsed) / self.lifetime)
    }

    pub fn new_trail(x: f64, y: f64, mut vector: Vector) -> Particle {
        vector.rotate(PI);
        vector.magnitude *= 0.6;

        Particle {
            x: x,
            y: y,
            vector: vector,
            size: 4.0,
            elapsed: 0.0,
            lifetime: 4.0,
            color: JsValue::from("#eeeebb99"),
        }
    }
}
