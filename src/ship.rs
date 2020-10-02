use std::collections::HashMap;

use crate::ai::*;
use crate::asteroid::*;
use crate::broadcast::*;
use crate::physics::*;
use crate::shape::*;
use crate::storage::*;

use std::f64::consts::{PI,FRAC_PI_2};

pub const TAU: f64 = PI * 2.0;

#[repr(usize)]
#[derive(Debug,Copy,Clone,Eq,PartialEq,Hash)]
pub enum ShipCategory {
    Bell = 0,
    Jalapeno = 1,
    Cayenne = 2,
}

impl From<u8> for ShipCategory {
    fn from(u: u8) -> ShipCategory {
        match u {
            0 => ShipCategory::Bell,
            1 => ShipCategory::Jalapeno,
            2 => ShipCategory::Cayenne,
            _ => ShipCategory::Bell,
        }
    }
}

const RADIUS: [f64; 3] = [18.0, 16.0, 20.0];
const HEALTH: [f64; 3] = [100.0, 25.0, 200.0];
const FORCE: [f64; 3] = [80.0, 24.0, 16.0];
const MASS: [f64; 3] = [1.0, 0.8, 1.2];

#[derive(Clone,Debug)]
pub struct Ship {
    id: u32,
    category: ShipCategory,
    brain: Box<dyn Brain>,
    vector: Vector,
    circle: Circle,
    health: f64,
    direction: f64,
    force: f64,
    mass: f64,
    elasticity: f64,
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

    pub fn aim(&mut self, point: Point) {
        let dx = point.x - self.get_x();
        let dy = point.y - self.get_y();

        self.direction = dy.atan2(dx);
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

    pub fn check_collisions(&mut self, time_delta: f64, actors: &HashMap<u32, ShipCache>, props: &Vec<Asteroid>) -> bool {
        let mut collision = false;
        let trajectory = self.get_trajectory_bounds(time_delta);

        for (id, actor) in actors.iter() {
            if *id != self.id &&
                trajectory.check_collision_rectangle(&actor.trajectory) &&
                self.circle.check_collision_circle(&actor.circle) {
                    collision = true;

                    //println!("Ship #{:} has {:.2} HP left.", self.id, self.health as f32 / 100.0);
                    self.collision_bounce(actor.circle, actor.vector, actor.elasticity, actor.mass);
            }
        }

        for prop in props.iter() {
            let circle = prop.get_circle();

            if trajectory.check_collision_shape(&circle) &&
                self.circle.check_collision_circle(&circle) {
                collision = true;

                //println!("Ship #{:} has {:.2} HP left.", self.id, self.health as f32 / 100.0);
                self.collision_bounce(circle, Vector::empty(), prop.get_elasticity(), f64::powf(2.0, 63.0)-1.0);
            }
        }

        collision
    }

    pub fn collision_bounce(&mut self, circle: Circle, vector: Vector, elasticity: f64, mass: f64) {
        let dx = self.circle.get_x() - circle.get_x();
        let dy = self.circle.get_y() - circle.get_y();

        // Move out of the other ship before changing trajectory
        // We overcompensate slightly to avoid ships sticking to each other
        let phi = dy.atan2(dx);
        let mut collision_vector = Vector::new(
            phi, (self.circle.get_r() + circle.get_r() - dx.hypot(dy)) * 2.0
        );
        self.circle.move_by_vector(collision_vector);

        // Shorter names for visibility
        let [v1, r1, m1] = [self.vector.magnitude, self.vector.direction, self.mass];
        let [v2, r2, m2] = [vector.magnitude * elasticity, vector.direction, mass];

        // Calculate new vector
        let a = v1 * ((r1 - phi).cos() *(m1 - m2));
        let b = (2.0 * m2 * v2 * (r2 - phi).cos());
        let base = (a + b) / (m1 + m2);
        let vx = (base * phi.cos()) + (v1 * (r1 - phi).sin() * (phi + FRAC_PI_2).cos());
        let vy = (base * phi.sin()) + (v1 * (r1 - phi).sin() * (phi + FRAC_PI_2).sin());

        self.vector = Vector::from_deltas(vx, vy);
        self.vector.magnitude *= self.elasticity;

        // Take damage
        self.health -= base / 10.0;
    }

    pub fn act(&mut self, time_delta: f64, cast: &Broadcast, actors: &HashMap<u32, ShipCache>, props: &Vec<Asteroid>) {
        // Store state before collisions
        let alive = self.health > 0.0;

        self.check_collisions(time_delta, actors, props);
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
            mass: self.mass,
            elasticity: self.elasticity,
            trajectory: self.get_trajectory_bounds(time_delta),
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

#[derive(Clone,Debug)]
pub struct ShipBuilder {
    id: u32,
    category: ShipCategory,
    pos: Point,
    vector: Vector,
}

impl ShipBuilder {
    pub fn default() -> ShipBuilder {
        ShipBuilder::new(ShipCategory::Bell)
    }

    pub fn new(category: ShipCategory) -> ShipBuilder {
        ShipBuilder {
            id: 0,
            category: category,
            pos: Point::new(0.0, 0.0),
            vector: Vector::empty(),
        }
    }

    pub fn set_vector(mut self, vector: Vector) -> Self {
        self.vector = vector;
        self
    }

    pub fn place(mut self, x: f64, y: f64) -> Self{
        self.pos = Point::new(x, y);
        self
    }

    pub fn tag(mut self, id: u32) -> Self {
        self.id = id;
        self
    }

    pub fn build(self) -> Ship {
        let cat = self.category as usize;

        Ship {
            id: self.id,
            category: self.category,
            brain: build_brain(self.category, self.id),
            vector: self.vector,
            circle: Circle::new(self.pos.x, self.pos.y, RADIUS[cat]),
            health: HEALTH[cat],
            direction: PI,
            force: FORCE[cat],
            mass: MASS[cat],
            elasticity: 2.0/3.0,
        }
    }
}

impl From<&ShipArgs> for ShipBuilder {
    fn from(s: &ShipArgs) -> ShipBuilder {
        ShipBuilder {
            id: 0,
            category: ShipCategory::from(s.0),
            pos: Point::new(s.1, s.2),
            vector: Vector::from(s.3),
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

    pub fn new_bell(&mut self, x: f64, y: f64) -> Ship {
        self.count += 1;

        ShipBuilder::new(ShipCategory::Bell)
        .place(x, y)
        .tag(self.count)
        .build()
    }

    pub fn new_jalapeno(&mut self, x: f64, y: f64) -> Ship {
        self.count += 1;

        ShipBuilder::new(ShipCategory::Jalapeno)
        .place(x, y)
        .tag(self.count)
        .build()
    }

    pub fn new_cayenne(&mut self, x: f64, y: f64) -> Ship {
        self.count += 1;

        ShipBuilder::new(ShipCategory::Cayenne)
        .place(x, y)
        .tag(self.count)
        .build()
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
    pub mass: f64,
    pub elasticity: f64,
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
        let mut a = ShipBuilder::new(ShipCategory::Jalapeno)
        .tag(0)
        .build();

        let mut b = ShipBuilder::new(ShipCategory::Jalapeno)
        .place(90.0, 0.0)
        .tag(1)
        .build();

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
