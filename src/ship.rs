use std::collections::HashMap;

use crate::ai::*;
use crate::broadcast::*;
use crate::physics::*;
use crate::shape::*;

use std::f64::consts::{PI,FRAC_PI_2};

pub const TAU: f64 = PI * 2.0;

#[derive(Debug,Copy,Clone,Eq,PartialEq,Hash)]
pub enum ShipCategory {
    Bell,
    Jalapeno,
    Cayenne,
}

#[derive(Debug)]
pub struct Ship {
    id: u32,
    category: ShipCategory,
    brain: Box<Brain>,
    vector: Vector,
    circle: Circle,
    health: f64,
    direction: f64,
    force: f64,
}

impl Ship {
    pub fn get_id(&self) -> u32 {
        self.id
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

    pub fn get_trajectory_bounds(&self, time_delta: f64) -> Rectangle {
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

    pub fn aim(&mut self, point: Point) {
        let dx = self.get_x() - point.x;
        let dy = self.get_y() - point.y;

        self.direction = f64::atan2(-dx, -dy);
    }

    pub fn thrust(&mut self, m: f64) {
        // Add to the vector. m is a percentage of the maximum force.
        self.vector.add_vector(
            Vector {
                direction: self.direction,
                magnitude: self.force.min(self.force * m)
            });
    }

    pub fn brake(&mut self, time_delta: f64) {
        if (self.vector.magnitude == 0.0) {
            return;
        }

        // Get in position. First find the opposite angle.
        self.direction = (self.vector.direction + PI) % TAU;

        // Are we in position? Any explicit rotation will be overridden.
        if (self.direction % TAU == (self.vector.direction + PI) % TAU) {
            // Reduce speed each tick until we reach 0.0
            self.vector.magnitude = (self.vector.magnitude - self.force * time_delta).max(0.0);
        }
    }

    pub fn abide_physics(&mut self, time_delta: f64) {
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

    pub fn check_collisions(&mut self, time_delta: f64, actors: &HashMap<u32, ShipCache>) -> bool {
        let mut collision = false;
        let trajectory = self.get_trajectory_bounds(time_delta);

        for (id, actor) in actors.iter() {
            if *id != self.id &&
                trajectory.check_collision_rectangle(&actor.trajectory) &&
                self.circle.check_collision_circle(&actor.circle) {
                    collision = true;

                    //println!("Ship #{:} has {:.2} HP left.", self.id, self.health as f32 / 100.0);
                    self.collision_bounce(actor);
            }
        }

        collision
    }

    pub fn collision_bounce(&mut self, ship: &ShipCache) {
        let dx = self.circle.get_x() - ship.circle.get_x();
        let dy = self.circle.get_y() - ship.circle.get_y();

        // Move out of the other ship before changing trajectory
        // We overcompensate slightly to avoid ships sticking to each other
        self.circle.move_by_vector(Vector {
            direction: dx.atan2(dy),
            magnitude: (self.circle.get_r() + ship.circle.get_r() - dx.hypot(dy)) * 1.2
        });

        // Change trajectory according to the angle of the collision
        //self.vector.rotate(f64::atan2(dx, dy) + FRAC_PI_2);
        let old_magnitude = self.vector.magnitude;
        let mut vector_delta = ship.vector.clone();
        vector_delta.subtract_vector(self.vector);
        self.vector.add_vector(vector_delta);
        self.vector.magnitude *= 2.0/3.0;

        // Take damage
        self.health -= (self.vector.magnitude - old_magnitude).abs() / 10.0;

        //println!("Ship #{:} has {:.2} HP left after taking {:.2} damage.", self.id, self.health,
        //(self.vector.magnitude - old_magnitude).abs() / 10.0);
    }

    pub fn act(&mut self, time_delta: f64, cast: &Broadcast, actors: &HashMap<u32, ShipCache>) {
        // Store state before collisions
        let alive = self.health > 0.0;

        self.check_collisions(time_delta, actors);
        self.abide_physics(time_delta);

        if self.health <= 0.0 {
            // If we were alive before collisions, notify the rest of our death
            if alive {
                cast.send_message(Message::new(0, self.id, MessageBody::Death));
            }

            return ();
        }

        for d in self.brain.think(time_delta, cast, actors) {
            match d {
                Directive::Rotate(n) => self.rotate(n),
                Directive::Thrust(n) => self.thrust(n),
                Directive::Brake => self.brake(time_delta),
                Directive::Aim(p) => self.aim(p),
            }
        }
    }

    pub fn get_cache(&self, time_delta: f64) -> ShipCache {
        ShipCache {
            id: self.id,
            category: self.category,
            vector: self.vector,
            circle: self.circle,
            health: self.health,
            direction: self.direction,
            force: self.force,
            trajectory: self.get_trajectory_bounds(1.0/60.0),
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

    pub fn new_bell(&mut self, x: f64, y: f64) -> Ship {
        self.count += 1;

        Ship {
            id: self.count,
            category: ShipCategory::Bell,
            brain: Box::new(BellBrain::new(self.count)),
            vector: Vector::empty(),
            circle: Circle::new(x, y, 18.0),
            health: 100.0,
            direction: PI,
            force: 80.0,
        }
    }

    pub fn new_jalapeno(&mut self, x: f64, y: f64) -> Ship {
        self.count += 1;

        Ship {
            id: self.count,
            category: ShipCategory::Jalapeno,
            brain: Box::new(JalapenoBrain::new(self.count)),
            vector: Vector::empty(),
            health: 25.0,
            circle: Circle::new(x, y, 18.0),
            direction: PI,
            force: 8.0,
        }
    }

    pub fn new_cayenne(&mut self, x: f64, y: f64) -> Ship {
        self.count += 1;

        Ship {
            id: self.count,
            category: ShipCategory::Cayenne,
            brain: Box::new(CayenneBrain::new(self.count)),
            vector: Vector::empty(),
            circle: Circle::new(x, y, 18.0),
            health: 200.0,
            direction: PI,
            force: 40.0,
        }
    }
}

pub struct ShipCache {
    pub id: u32,
    pub category: ShipCategory,
    pub vector: Vector,
    pub circle: Circle,
    pub health: f64,
    pub direction: f64,
    pub force: f64,
    pub trajectory: Rectangle,
}

impl ShipCache {
    pub fn get_point(&self) -> Point {
        Point::new(self.circle.x, self.circle.y)
    }

    pub fn render_piston(&self) -> [f64; 4] {
        [self.circle.get_x(), self.circle.get_y(), self.circle.get_r(), self.direction]
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
            category: ShipCategory::Jalapeno,
            brain: Box::new(JalapenoBrain::new()),
            vector: a,
            circle: Circle::new(0.0, 0.0, 18.0),
            direction: PI,
        };

        let mut b = Ship {
            id: 1,
            category: ShipCategory::Jalapeno,
            brain: Box::new(JalapenoBrain::new()),
            vector: b,
            circle: Circle::new(90.0, 0.0, 18.0),
            direction: PI,
        };

        let cast = Broadcast::new();

        /*
        for i in 0..2 {
            let actors = vec![a.get_cache(1.0/60.0), b.get_cache(1.0/60.0)];

            a.act_player(1.0, &cast, &actors);
            b.act_player(1.0, &cast, &actors);
        }*/

        println!("{:}\n{:}", a, b)
    }

    #[test]
    fn cases() {
        println!("Case #1: Moving and still ship:");
        collide(Vector::new(std::f64::consts::FRAC_PI_2, 90.0), Vector::empty());
    }
}
