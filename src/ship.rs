
use crate::broadcast::Broadcast;
use crate::physics::*;
use crate::shape::*;

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
            direction: PI,
            color: [0.8, 0.4, 0.4, 1.0],
        }
    }

    pub fn new_r(x: f64, y: f64, r: f64) -> Ship {
        Ship {
            id: 0,
            vector: Vector::empty(),
            circle: Circle::new(x, y, r),
            direction: PI,
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

    pub fn get_trajectory(&self, time_delta: f64) -> Rectangle {
        //Rectangle::from_bounds(self.circle.top(), self.circle.right(),
        //                       self.circle.bottom(), self.circle.left())
        Rectangle::from_bounds(
            self.circle.top() + (self.vector.get_dy() * time_delta).min(0.0),
            self.circle.right() + (self.vector.get_dx() * time_delta).max(0.0),
            self.circle.bottom() + (self.vector.get_dy() * time_delta).max(0.0),
            self.circle.left() + (self.vector.get_dx() * time_delta).min(0.0)
            )
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

    pub fn brake(&mut self, time_delta: f64) {
        if (self.vector.magnitude == 0.0) {
            return;
        }

        // Get in position. First find the opposite angle.
        self.direction = (self.vector.direction + PI) % TAU;

        // Are we in position?
        if (self.direction % TAU == (self.vector.direction + PI) % TAU) {
            // Reduce speed each tick until we reach 0.0
            self.vector.magnitude = (self.vector.magnitude - 80.0 * time_delta).max(0.0);
        }

        /*
        let radian_delta: f64 = goal - self.direction;
        self.direction += radian_delta.abs().min(TAU * time_delta) * (radian_delta - PI).signum();
        self.direction %= TAU;
         */
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
        //self.vector.rotate(f64::atan2(dx, dy) + FRAC_PI_2);
        let mut vector_delta = ship.vector.clone();
        vector_delta.subtract_vector(self.vector);
        self.vector.add_vector(vector_delta);
        self.vector.magnitude *= 2.0/3.0;
    }

    pub fn act_player(&mut self, time_delta: f64, cast: &Broadcast, actors: &Vec<ShipCache>) {
        let mut collision = false;
        let trajectory = self.get_trajectory(time_delta);

        for actor in actors.iter() {
            if actor.id != self.id &&
                trajectory.check_collision_rectangle(&actor.trajectory) &&
                self.circle.check_collision_circle(&actor.circle) {
                collision = true;

                self.collision_bounce(actor);
            }
        }

        if collision {
            self.color = [0.7, 0.3, 0.3, 1.0];
        } else {
            self.color = [0.8, 0.4, 0.4, 1.0];
        }

        self.abide_physics(time_delta);

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
            if pressed.contains(&'B') {
                self.brake(time_delta);
            }
        }
    }

    pub fn act_npc(&mut self, time_delta: f64, cast: &Broadcast, actors: &Vec<ShipCache>) {
        //self.rotate(PI * time_delta);


        let mut collision = false;
        let trajectory = self.get_trajectory(time_delta);

        for actor in actors.iter() {
            if actor.id != self.id &&
                trajectory.check_collision_rectangle(&actor.trajectory) &&
                self.circle.check_collision_circle(&actor.circle) {
                    collision = true;

                    self.collision_bounce(actor);
            }
        }

        if collision {
            self.color = [0.7, 0.3, 0.3, 1.0];
        } else {
            self.color = [0.8, 0.4, 0.4, 1.0];
        }

        self.abide_physics(time_delta);

        let dx = self.get_x() - cast.player_position.0;
        let dy = self.get_y() - cast.player_position.1;

        self.direction = f64::atan2(-dx, -dy);
        self.thrust(8.0 * time_delta);

    }

    pub fn get_cache(&self) -> ShipCache {
        ShipCache {
            id: self.id,
            vector: self.vector,
            circle: self.circle,
            direction: self.direction,
            trajectory: self.get_trajectory(1.0/60.0),
            color: self.color,
        }
    }
}

impl std::fmt::Display for Ship {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ship #{} at x: {:.2}, y: {:.2}), facing {:.2}. Moving at {:.2} {:.2}",
               self.id, self.circle.get_x(), self.circle.get_y(), self.direction,
               self.vector.direction, self.vector.magnitude
        )
    }
}

pub struct ShipFactory {
    count: u32,
}

impl ShipFactory {
    pub fn new() -> ShipFactory {
        //println!("{}", std::mem::size_of::<u64>());
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
    pub trajectory: Rectangle,
    pub color: [f32; 4],
}

impl ShipCache {
    pub fn render_piston(&self) -> [f64; 4] {
        [self.circle.get_x(), self.circle.get_y(), self.circle.get_r(), self.direction]
    }

    pub fn get_color(&self) -> [f32; 4] {
        self.color
    }

    pub fn test_trajectory(&self) {
        let rect = self.trajectory;

        println!("{:?}, {:?}, {:?}, {:?}, {:?}",
                 rect,
                 self.circle.top() == rect.top(),
                 self.circle.right() == rect.right(),
                 self.circle.bottom() == rect.bottom(),
                 self.circle.left() == rect.left(),
        );
    }
}

impl std::fmt::Display for ShipCache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ship #{} at x: {:.2}, y: {:.2}), facing {:.2}. Moving at {:.2} {:.2}",
               self.id, self.circle.get_x(), self.circle.get_y(), self.direction,
               self.vector.direction, self.vector.magnitude
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::ship::*;
    use crate::physics::*;

    fn collide(a: Vector, b: Vector) {
        let mut a = Ship {
            id: 0,
            vector: a,
            circle: Circle::new(0.0, 0.0, 18.0),
            direction: PI,
            color: [0.8, 0.4, 0.4, 1.0],
        };

        let mut b = Ship {
            id: 1,
            vector: b,
            circle: Circle::new(90.0, 0.0, 18.0),
            direction: PI,
            color: [0.8, 0.4, 0.4, 1.0],
        };

        let cast = Broadcast::new();

        for i in 0..2 {
            let actors = vec![a.get_cache(), b.get_cache()];

            a.act_player(1.0, &cast, &actors);
            b.act_player(1.0, &cast, &actors);
        }

        println!("{:}\n{:}", a, b)
    }

    #[test]
    fn cases() {
        println!("Case #1: Moving and still ship:");
        collide(Vector::new(std::f64::consts::FRAC_PI_2, 90.0), Vector::empty());
    }
}
