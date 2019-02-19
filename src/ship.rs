use crate::broadcast::Broadcast;

#[derive(Debug)]
pub struct Vector {
    direction: f64,
    magnitude: f64,
}

impl Vector {
    pub fn new() -> Vector {
        Vector {
            direction: 0.0,
            magnitude: 1.0,
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

#[derive(Debug)]
pub struct Ship {
    x: f64,
    y: f64,
    direction: f64,
    vector: Vector,
}

impl Ship {
    pub fn new(x: f64, y: f64) -> Ship {
        Ship {
            x: x,
            y: y,
            direction: 0.0,
            vector: Vector::new(),
        }
    }

    pub fn get_x(&self) -> f64 {
        self.x
    }

    pub fn get_y(&self) -> f64{
        self.y
    }

    pub fn set_direction(&mut self, d: f64) {
        self.direction = d;
    }

    pub fn thrust(&mut self, m: f64) {
        self.vector.add_vector(Vector {
            direction: self.direction,
            magnitude: m
        });
    }

    pub fn abide_physics(&mut self, time_delta: f64) {
        self.x += self.vector.get_dx() * time_delta;
        self.y += self.vector.get_dy() * time_delta;
    }

    pub fn rotate(&mut self, d: f64) {
        self.direction += d;
    }

    pub fn act_player(&mut self, time_delta: f64, cast: &Broadcast) {
        let pressed: Vec<char> = cast.get_input();

        if pressed.contains(&'L') {
            self.rotate(360.0 * time_delta);
        }
        if pressed.contains(&'R') {
            self.rotate(-360.0 * time_delta);
        }
        if pressed.contains(&'T') {
            self.thrust(40.0 * time_delta);
        }

        self.abide_physics(time_delta);
    }

    pub fn render_piston(&self) -> [f64; 3] {
        [self.x, self.y, self.direction]
    }
}
