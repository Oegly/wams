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
            direction: dx.atan2(dy).to_degrees(),
            magnitude: dx.hypot(dy),
        }
    }

    pub fn get_dx(&self) -> f64 {
        self.direction.to_radians().sin() * self.magnitude
    }

    pub fn get_dy(&self) -> f64 {
        self.direction.to_radians().cos() * self.magnitude
    }

    pub fn add_vector(&mut self, v: Vector) {
        let _x: f64 = self.get_dx() + v.get_dx();
        let _y: f64 = self.get_dy() + v.get_dy();

        self.direction = _x.atan2(_y).to_degrees();
        self.magnitude = _x.hypot(_y);
    }
}

#[derive(Clone,Copy,Debug)]
pub struct Circle {
    x: f64,
    y: f64,
    r: f64,
    vector: Vector,
}

impl Circle {
    pub fn new(x: f64, y: f64, r: f64) -> Circle {
        Circle {
            x: x,
            y: y,
            r: r,
            vector: Vector::new(),
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

    pub fn get_vector(&self) -> Vector {
        self.vector
    }

    pub fn move_by(&mut self, x: f64, y: f64) {
        self.x += x;
        self.y += y;
    }

    pub fn set_vector(&mut self, vector: Vector) {
        self.vector = vector;
    }

    pub fn add_vector(&mut self, vector: Vector) {
        self.vector.add_vector(vector);
    }

    pub fn get_dx(&self) -> f64 {
        self.vector.get_dx()
    }

    pub fn get_dy(&self) -> f64 {
        self.vector.get_dy()
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

    pub fn thrust(&mut self, v: Vector) {
        self.vector.add_vector(v);
    }

    pub fn abide_physics(&mut self, time_delta: f64) {
        self.x += self.vector.get_dx() * time_delta;
        self.y += self.vector.get_dy() * time_delta;
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn works() {
        assert_eq!(true, true);
    }
}
