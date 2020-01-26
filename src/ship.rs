use crate::broadcast::Broadcast;
use crate::physics::*;

#[derive(Debug)]
pub struct Ship {
    id: u32,
    circle: Circle,
    direction: f64,
    color: [f32; 4],
}

impl Ship {
    pub fn new(x: f64, y: f64) -> Ship {
        Ship {
            id: 0,
            circle: Circle::new(x, y, 18.0),
            direction: 180.0,
            color: [0.8, 0.4, 0.4, 1.0],
        }
    }

    pub fn new_r(x: f64, y: f64, r: f64) -> Ship {
        Ship {
            id: 0,
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
        self.circle.thrust(Vector {
            direction: self.direction,
            magnitude: m
        });
    }

    pub fn abide_physics(&mut self, time_delta: f64) {
        self.circle.abide_physics(time_delta);
    }

    pub fn rotate(&mut self, d: f64) {
        self.direction = (self.direction + d) % 360.0;

        if (self.direction > 0.0) {
            self.direction += 360.0;
        }
    }

    pub fn collision_bounce(&mut self, ship: &ShipCache) {
        // TODO: Clean this method up!
        // Doing some work already done in Circle::check_collision_circle.
        // Could it be optimised without convoluting the code?
        let (ax, ay, ar, ad) = (self.circle.get_x(), self.circle.get_y(), self.circle.get_r(),
                                self.circle.get_vector().direction.to_radians());
        let (bx, by, br, bd) = (ship.circle.get_x(), ship.circle.get_y(), ship.circle.get_r(),
                                ship.circle.get_vector().direction.to_radians());

        let dx = ax - bx;
        let dy = ay - by;

        // Find point of impact
        let px = ((ax * ar) + (bx * br)) / (ar + br);
        let py = ((ay * ar) + (by * br)) / (ar + br);

        let av = self.circle.get_vector();

        let collision_rad = f64::atan2(dx, dy);
        let collision_deg = collision_rad.to_degrees();
        let perpendicular = collision_deg + 90.0;
        let degree_delta = perpendicular - av.direction;

        // Move out of the other ship before changing trajectory
        let overlap = ar + br - dx.hypot(dy);
        let correction_vector = Vector {direction: av.direction + 180.0, magnitude: overlap};
        let (mx, my) = (correction_vector.get_dx(), correction_vector.get_dy());
        self.circle.move_by(mx, my);

        //println!("{:.2} - {:.2} = {:.2}", collision_deg, av.direction, collision_deg - av.direction);
        //println!("Met #{:}. {:.2} != {:.2} Overlap: {:.2}. Moving by {:.2}, {:.2}", ship.id, dx.hypot(dy), ar + br, overlap, mx, my);

        self.circle.set_vector(Vector {
            direction: (av.direction + degree_delta * 2.0) % 360.0,
            magnitude: av.magnitude,
        });
    }

    pub fn act_player(&mut self, time_delta: f64, cast: &Broadcast, actors: &Vec<ShipCache>) {
        let pressed: Vec<char> = cast.get_input();

        if pressed.contains(&'M') {
            let dx = self.get_x() - cast.cursor.0;
            let dy = self.get_y() - cast.cursor.1;

            self.direction = f64::atan2(-dx, -dy).to_degrees();
            self.thrust(80.0 * time_delta);
        }
        else {
            if pressed.contains(&'L') {
                self.rotate(360.0 * time_delta);
            }
            if pressed.contains(&'R') {
                self.rotate(-360.0 * time_delta);
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
        //self.rotate(180.0 * time_delta);
        self.abide_physics(time_delta);
    }

    pub fn get_cache(&self) -> ShipCache {
        ShipCache {
            id: self.id,
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
            circle: Circle::new(x, y, 18.0),
            direction: 180.0,
            color: [0.8, 0.4, 0.4, 1.0],
        }
    }

    pub fn register_ship(&mut self, ship: Ship) -> Ship {
        self.count += 1;

        let cache = ship.get_cache();

        Ship {
            id: self.count,
            circle: cache.circle,
            direction: cache.direction,
            color: cache.color,
        }
    }
}

pub struct ShipCache {
    pub id: u32,
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
