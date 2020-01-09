use crate::broadcast::Broadcast;

#[derive(Clone,Copy,Debug)]
pub struct Vector {
    direction: f64,
    magnitude: f64,
}

impl Vector {
    pub fn new() -> Vector {
        Vector {
            direction: 0.0,
            magnitude: 0.0,
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

    pub fn get_dx(&self) -> f64 {
        self.vector.get_dx()
    }

    pub fn get_dy(&self) -> f64 {
        self.vector.get_dy()
    }

    pub fn thrust(&mut self, v: Vector) {
        self.vector.add_vector(v);
    }

    pub fn abide_physics(&mut self, time_delta: f64) {
        self.x += self.vector.get_dx() * time_delta;
        self.y += self.vector.get_dy() * time_delta;
    }
}

#[derive(Debug)]
pub struct Ship {
    circle: Circle,
    direction: f64,
}

impl Ship {
    pub fn new(x: f64, y: f64) -> Ship {
        Ship {
            circle: Circle::new(x, y, 18.0),
            direction: 0.0,
        }
    }

    pub fn get_x(&self) -> f64 {
        self.circle.x
    }

    pub fn get_y(&self) -> f64{
        self.circle.y
    }

    pub fn set_direction(&mut self, d: f64) {
        self.direction = d;
    }

    pub fn thrust(&mut self, m: f64) {
        self.circle.thrust(Vector {
            direction: self.direction,
            magnitude: m
        });
    }

    pub fn abide_physics(&mut self, time_delta: f64) {
        self.circle.abide_physics(time_delta);
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
        [self.circle.x, self.circle.y, self.direction]
    }
}

pub struct ShipCache {
    circle: Circle,
    direction: f64,
}
