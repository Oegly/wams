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

    pub fn new(d: f64, m: f64) -> Vector {
        Vector {
            direction: d,
            magnitude: m,
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

    pub fn subtract_vector(&mut self, v: Vector) {
        let _x: f64 = self.get_dx() - v.get_dx();
        let _y: f64 = self.get_dy() - v.get_dy();

        self.direction = _x.atan2(_y);
        self.magnitude = _x.hypot(_y);
    }

    pub fn rotate(&mut self, angle: f64) {
        let delta = angle - self.direction;

        self.direction = (self.direction + delta * 2.0) % TAU;
    }

    pub fn radian_delta(&self, mut r: f64) -> f64{
        r %= TAU;

        if self.direction > r {
            if self.direction - r >= PI {
                return TAU + r - self.direction;
            }
        }

        if r - self.direction > PI {
            return (TAU - r) * -1.0 - self.direction
        }

        r - self.direction
    }
}

impl From<(f64, f64)> for Vector {
    fn from(t: (f64, f64)) -> Vector {
        Vector::new(t.0, t.1)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{PI,FRAC_PI_2};
    use crate::physics::*;

    const TAU: f64 = PI * 2.0;

    #[test]
    fn test_radian_delta() {
        let v1 = Vector::new(FRAC_PI_2, 0.0);
        let v2 = Vector::new(FRAC_PI_2 * 3.0, 0.0);

        assert_eq!(v1.radian_delta(PI), FRAC_PI_2);
        assert_eq!(v1.radian_delta(1.0), 1.0 - FRAC_PI_2);
        assert_eq!(v1.radian_delta(6.0), 6.0 - TAU - FRAC_PI_2);
        assert_eq!(v2.radian_delta(6.0), 6.0 - FRAC_PI_2 * 3.0);
        assert_eq!(v2.radian_delta(4.0), 4.0 - FRAC_PI_2 * 3.0);
        assert_eq!(v2.radian_delta(1.0), TAU - FRAC_PI_2 * 3.0 + 1.0);

        assert_eq!(v1.radian_delta(v2.direction), PI);
        assert_eq!(v2.radian_delta(v1.direction), PI);
    }
}
