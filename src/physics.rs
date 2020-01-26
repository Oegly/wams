use std::f64::consts::{PI,FRAC_PI_2};

const TAU: f64 = PI * 2.0;

#[derive(Clone,Copy,Debug)]
pub struct Vector {
    pub direction: f64,
    pub magnitude: f64,
}

impl Vector {
    pub fn empty() -> Vector {
        Vector {
            direction: 0.0,
            magnitude: 0.0,
        }
    }

    pub fn new() -> Vector {
        Vector {
            direction: 0.0,
            magnitude: 0.0,
        }
    }

    pub fn from_deltas(dx: f64, dy: f64) -> Vector {
        Vector {
            direction: dx.atan2(dy),
            magnitude: dx.hypot(dy),
        }
    }

    pub fn get_dx(&self) -> f64 {
        self.direction.sin() * self.magnitude
    }

    pub fn get_dy(&self) -> f64 {
        self.direction.cos() * self.magnitude
    }

    pub fn add_vector(&mut self, v: Vector) {
        let _x: f64 = self.get_dx() + v.get_dx();
        let _y: f64 = self.get_dy() + v.get_dy();

        self.direction = _x.atan2(_y);
        self.magnitude = _x.hypot(_y);
    }

    pub fn rotate(&mut self, angle: f64) {
        let delta = angle - self.direction;

        self.direction = (self.direction + delta * 2.0) % TAU;
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

    pub fn check_collision_circle(&self, circle: &Circle) -> bool {
        let dx = self.x - circle.get_x();
        let dy = self.y - circle.get_y();

        let distance = dx.hypot(dy);

        if distance < self.r + circle.get_r() {
            return true;
        }

        false

    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn works() {
        assert_eq!(true, true);
    }
}
