use crate::broadcast::Broadcast;
use crate::physics::*;
use std::f64::consts::{PI,FRAC_PI_2};

const TAU: f64 = PI * 2.0;

#[derive(Debug)]
pub struct Ship {
    id: u32,
    vector: Vector,
    circle: Circle,
    direction: f64,
    color: [f32; 4],
}

impl Ship {
    pub fn new(x: f64, y: f64) -> Ship {
        Ship {
            id: 0,
            vector: Vector::empty(),
            circle: Circle::new(x, y, 18.0),
            direction: 180.0,
            color: [0.8, 0.4, 0.4, 1.0],
        }
    }

    pub fn new_r(x: f64, y: f64, r: f64) -> Ship {
        Ship {
            id: 0,
            vector: Vector::empty(),
            circle: Circle::new(x, y, r),
            direction: 180.0,
            color: [0.8, 0.4, 0.4, 1.0],
        }
    }

    pub fn get_x(&self) -> f64 {
        self.circle.get_x()
    }

    pub fn get_y(&self) -> f64{
        self.circle.get_y()
    }

    pub fn set_direction(&mut self, d: f64) {
        self.direction = d;
    }

    pub fn get_elasticity(&self, rad: f64) -> f64 {
        // Todo: Make it possible to do something different with elasticity
        0.9 - f64::abs(self.direction - rad) / 400.0
    }

    pub fn thrust(&mut self, m: f64) {
        self.vector.add_vector(
            Vector {
                direction: self.direction,
                magnitude: m
            });
    }

    pub fn abide_physics(&mut self, time_delta: f64) {
        //self.circle.abide_physics(time_delta);
        self.circle.move_by(
            self.vector.get_dx() * time_delta,
            self.vector.get_dy() * time_delta
        );
    }

    pub fn rotate(&mut self, d: f64) {
        self.direction = (self.direction + d) % TAU;

        if (self.direction > 0.0) {
            self.direction += TAU;
        }
    }

    pub fn collision_bounce(&mut self, ship: &ShipCache) {
        let dx = ship.circle.get_x() - self.circle.get_x();
        let dy = ship.circle.get_y() - self.circle.get_y();

        // Move out of the other ship before changing trajectory
        self.circle.move_by_vector(Vector {
            direction: self.vector.direction + PI,
            magnitude: self.circle.get_r() + ship.circle.get_r() - dx.hypot(dy)
        });

        // Change trajectory according to the angle of the collision
        self.vector.rotate(f64::atan2(dx, dy) + FRAC_PI_2);
    }

    pub fn act_player(&mut self, time_delta: f64, cast: &Broadcast, actors: &Vec<ShipCache>) {
        let pressed: Vec<char> = cast.get_input();

        if pressed.contains(&'M') {
            let dx = self.get_x() - cast.cursor.0;
            let dy = self.get_y() - cast.cursor.1;

            self.direction = f64::atan2(-dx, -dy);
            self.thrust(80.0 * time_delta);
        }
        else {
            if pressed.contains(&'L') {
                self.rotate(TAU * time_delta);
            }
            if pressed.contains(&'R') {
                self.rotate(-TAU * time_delta);
            }
            if pressed.contains(&'T') {
                self.thrust(80.0 * time_delta);
            }
        }

        self.abide_physics(time_delta);

        let mut collision = false;

        for actor in actors.iter() {
            if actor.id != self.id
                && self.circle.check_collision_circle(&actor.circle) {
                collision = true;

                self.collision_bounce(actor);
            }
        }

        if collision {
            self.color = [0.4, 0.8, 0.4, 1.0];
        } else {
            self.color = [0.8, 0.4, 0.4, 1.0];
        }
    }

    pub fn act_npc(&mut self, time_delta: f64, cast: &Broadcast, actors: &Vec<ShipCache>) {
        //self.rotate(PI * time_delta);
        self.abide_physics(time_delta);
    }

    pub fn get_cache(&self) -> ShipCache {
        ShipCache {
            id: self.id,
            vector: self.vector,
            circle: self.circle,
            direction: self.direction,
            color: self.color,
        }
    }
}

pub struct ShipFactory {
    count: u32,
}

impl ShipFactory {
    pub fn new() -> ShipFactory {
        ShipFactory {
            count: 0,
        }
    }

    pub fn new_ship(&mut self, x: f64, y: f64) -> Ship {
        self.count += 1;

        Ship {
            id: self.count,
            vector: Vector::empty(),
            circle: Circle::new(x, y, 18.0),
            direction: PI,
            color: [0.8, 0.4, 0.4, 1.0],
        }
    }

    pub fn register_ship(&mut self, ship: Ship) -> Ship {
        self.count += 1;

        let cache = ship.get_cache();

        Ship {
            id: self.count,
            vector: cache.vector,
            circle: cache.circle,
            direction: cache.direction,
            color: cache.color,
        }
    }
}

pub struct ShipCache {
    pub id: u32,
    pub vector: Vector,
    pub circle: Circle,
    pub direction: f64,
    pub color: [f32; 4],
}

impl ShipCache {
    pub fn render_piston(&self) -> [f64; 4] {
        [self.circle.get_x(), self.circle.get_y(), self.circle.get_r(), self.direction]
    }

    pub fn get_color(&self) -> [f32; 4] {
        self.color
    }
}

/*
#[cfg(test)]
mod tests {
    #[test]
    fn works() {
        assert_eq!(true, true);
    }
}
*/
