use std::collections::HashMap;

use crate::broadcast::*;
use crate::physics::*;
use crate::shape::*;
use crate::ship::*;

pub enum Directive {
    Rotate(f64),
    Thrust(f64),
    Brake,
    Aim(Point)
}

pub trait Brain {
    fn think(&mut self, time_delta: f64, cast: &Broadcast, actors: &HashMap<u32, ShipCache>) -> Vec<Directive>;
}

impl std::fmt::Debug for Brain {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "I am a brain.")
    }
}

#[derive(Clone,Debug)]
pub struct BellBrain {
    previous_collisons: Vec<u32>,
}

impl BellBrain {
    pub fn new() -> BellBrain {
        BellBrain {
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
    previous_collisons: Vec<u32>,
}

impl JalapenoBrain {
    pub fn new() -> JalapenoBrain {
        JalapenoBrain {
            previous_collisons: Vec::new(),
        }
    }
}

impl Brain for JalapenoBrain {
    fn think(&mut self, time_delta: f64, cast: &Broadcast, actors: &HashMap<u32, ShipCache>) -> Vec<Directive> {
        vec![Directive::Aim(cast.player_position), Directive::Thrust(1.0 * time_delta)]
    }
}
