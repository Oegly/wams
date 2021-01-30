use wasm_bindgen::prelude::*;
use crate::physics::{Circle,Point,Vector};

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

    pub fn get_alpha(&self) -> f64 {
        ((self.lifetime - self.elapsed) / self.lifetime) / 2.0
    }

    pub fn new_trail(x: f64, y: f64, mut vector: Vector) -> Particle {
        // Very cheap RNG
        let r = (x + y / vector.magnitude).sin() % 0.8 - 0.4;

        vector.rotate(FRAC_PI_2 + r);
        vector.magnitude *= 0.2;

        Particle::new(x, y, vector, 3.0, 1.8, JsValue::from("#eeee66"))
    }

    pub fn new_collision(x: f64, y: f64, m: f64, color: &str) -> Vec<Particle> {
        let mut ret = Vec::new();

        for i in 0..12 {
            let d = (TAU / 12.0) * i as f64;
            ret.push(Particle::new(x, y, Vector::new(d, 12.0), 2.0, 3.0, JsValue::from(color)));
        }   

        ret
    }

    pub fn new_ship_collision(a: u32, b: u32, p: Point) -> Vec<Particle> {
        let color = match a > b {
            true => "#ee6666aa",
            false => "#eeee66aa",
        };

        Particle::new_collision(p.x, p.y, 12.0, color)
    }

    pub fn new_asteroid_collision(p: Point) -> Vec<Particle> {
        Particle::new_collision(p.x, p.y, 12.0, "#ffdd66aa")
    }
}
