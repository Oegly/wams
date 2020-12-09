//use rand::{Rng, SeedableRng, rngs::StdRng};

use wasm_bindgen::prelude::*;
use crate::physics::*;
use crate::shape::*;

use std::f64::consts::{PI,FRAC_PI_2,TAU};

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
        // Very cheap RNG
        let r = ((x + y + vector.magnitude) % 0.002) * 999.9 - 1.0;

        vector.rotate(FRAC_PI_2 + r);
        vector.magnitude *= 0.6;

        Particle {
            x: x,
            y: y,
            vector: vector,
            size: 2.0,
            elapsed: 0.0,
            lifetime: 3.0,
            color: JsValue::from("#eeee66"),
        }
    }

    pub fn new_ship_collision(a: u32, b: u32, p: Point) -> Vec<Particle> {
        let mut ret = Vec::new();
        let color = match a > b {
            true => "#eeee66aa",
            false => "#ee6666aa",
        };

        for i in 0..12 {
            let d = (TAU / 12.0) * i as f64;

            ret.push(Particle {
                x: p.x,
                y: p.y,
                vector: Vector::new(d, 12.0),
                size: 2.0,
                elapsed: 0.0,
                lifetime: 3.0,
                color: JsValue::from(color),
            });
        }   

        ret
    }
}
