use std::collections::HashMap;

use rand::{Rng, SeedableRng, rngs::StdRng};

use crate::physics::*;
use crate::shape::*;
use crate::storage::*;

use std::f64::consts::{PI,FRAC_PI_2,TAU};

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

    pub fn from_wall_args(wall: &WallArgs) -> Vec<Asteroid> {
        let [x, y] = [wall.0, wall.1];
        let [w, h] = [wall.2, wall.3];
        let r = wall.4;

        if w < 0 || h < 0 {
            panic!("Width and height for the wall are supposed to be positive.");
        }

        // We start at (x, y), and then go horisontally if w > h, else vertically
        let n = match (w - h).signum() {
            -1 => h,
            _ => w,
        };

        let mut rng = StdRng::seed_from_u64(x as u64 + y as u64);

        let mut ret: Vec<Asteroid> = Vec::new();

        let mut i = 0;
        while i < n {
            let _r = rng.gen_range(r.0, r.1);
            let _x = x + (i % w);
            let _y = y + (i % h);

            ret.push(Asteroid::new(_x as f64, _y as f64, _r as f64));
            i += _r + rng.gen_range(r.0, r.1) * 2;
        }

        ret
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

impl From<&AsteroidArgs> for Asteroid {
    fn from(args: &AsteroidArgs) -> Asteroid {
        Asteroid::new(args.0, args.1, args.2)
    }
}
