use std::collections::HashMap;

use crate::broadcast::*;
use crate::physics::*;
use crate::shape::*;
use crate::ship::*;

use std::f64::consts::{PI,FRAC_PI_2};

pub enum Directive {
    Rotate(f64),
    Thrust(f64),
    Brake,
    Aim(Point)
}

pub trait Brain {
    fn think(&mut self, time_delta: f64, cast: &Broadcast, actors: &HashMap<u32, ShipCache>) -> Vec<Directive>;
}

impl std::fmt::Debug for dyn Brain {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "I am a brain.")
    }
}

#[derive(Clone,Debug)]
pub struct BellBrain {
    id: u32,
    previous_collisons: Vec<u32>,
}

impl BellBrain {
    pub fn new(id: u32) -> BellBrain {
        BellBrain {
            id: id,
            previous_collisons: Vec::new(),
        }
    }
}

impl Brain for BellBrain {
    fn think(&mut self, time_delta: f64, cast: &Broadcast, actors: &HashMap<u32, ShipCache>) -> Vec<Directive> {
        let pressed: Vec<char> = cast.get_input();

        if pressed.contains(&'M') {
            return vec![Directive::Aim(Point::from_tuple(cast.cursor)), Directive::Thrust(1.0 * time_delta)];
        }
        else {
            let mut ret = Vec::<Directive>::new();

            if pressed.contains(&'L') {
                ret.push(Directive::Rotate(TAU * time_delta));
            }
            if pressed.contains(&'R') {
                ret.push(Directive::Rotate(-TAU * time_delta));
            }
            if pressed.contains(&'T') {
                ret.push(Directive::Thrust(1.0 * time_delta));
            }
            if pressed.contains(&'B') {
                ret.push(Directive::Brake);
            }

            return ret;
        }
    }
}

#[derive(Clone,Debug)]
pub struct JalapenoBrain {
    id: u32,
    previous_collisons: Vec<u32>,
}

impl JalapenoBrain {
    pub fn new(id: u32) -> JalapenoBrain {
        JalapenoBrain {
            id: id,
            previous_collisons: Vec::new(),
        }
    }
}

impl Brain for JalapenoBrain {
    fn think(&mut self, time_delta: f64, cast: &Broadcast, actors: &HashMap<u32, ShipCache>) -> Vec<Directive> {
        vec![Directive::Aim(cast.player_position), Directive::Thrust(1.0 * time_delta)]
    }
}

#[derive(Clone,Debug)]
pub struct CayenneBrain {
    id: u32,
    previous_collisons: Vec<u32>,
}

impl CayenneBrain {
    pub fn new(id: u32) -> CayenneBrain {
        CayenneBrain {
            id: id,
            previous_collisons: Vec::new(),
        }
    }

    pub fn chase(&mut self, time_delta: f64, me: &ShipCache, target: &ShipCache) -> Vec<Directive> {
        // How to get to target (from where we are now)
        let ideal_path = Segment::new(me.get_point(), target.get_point());

        // Where would we go if we thrusted now?
        let mut plan = me.vector.clone();
        plan.add_vector(Vector::new(ideal_path.get_direction(), me.force * time_delta));
        let planned_path = Segment::new(
            Point::new(me.circle.x + plan.get_dx(), me.circle.y + plan.get_dy()),
            target.get_point()
        );

        // Would we be closer to the target?
        if planned_path.get_length() < ideal_path.get_length() {
            return vec![
                Directive::Aim(target.get_point()),
                Directive::Thrust(1.0 * time_delta)
            ];
        }

        vec![Directive::Brake]
    }
}

impl Brain for CayenneBrain {
    fn think(&mut self, time_delta: f64, cast: &Broadcast, actors: &HashMap<u32, ShipCache>) -> Vec<Directive> {
        match cast.player_id {
            Some(id) => return self.chase(time_delta, &actors[&self.id], &actors[&id]),
            None => return vec![],
        }
    }
}
