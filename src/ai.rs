use std::collections::HashMap;

use crate::asteroid::*;
use crate::broadcast::*;
use crate::physics::*;
use crate::shape::*;
use crate::ship::*;

use std::f64::consts::{PI,FRAC_PI_2,TAU};

pub fn build_brain(category: ShipCategory, id: u32) -> Box<dyn Brain> {
    match category {
        ShipCategory::Bell => Box::new(BellBrain::new(id)),
        ShipCategory::Jalapeno => Box::new(JalapenoBrain::new(id)),
        ShipCategory::Cayenne => Box::new(CayenneBrain::new(id)),
    }
}

pub enum Directive {
    Rotate(f64),
    Thrust(f64),
    Brake,
    Aim(Point)
}

pub trait Brain {
    fn think(&mut self, time_delta: f64, cast: &Broadcast, actors: &HashMap<u32, ShipCache>, props: &Vec<Asteroid>) -> Vec<Directive>;
    fn box_clone(&self) -> Box<dyn Brain>;

    fn target_visible(&self, target: Point, me: Point, props: &Vec<Asteroid>) -> bool {
        let path = Segment::new(target, me);

        for prop in props.iter() {
            if path.check_collision_circle(&prop.get_circle()) {
                return false;
            }
        }

        true
    }
}

impl Clone for Box<dyn Brain> {
    fn clone(&self) -> Box<dyn Brain> {
        self.box_clone()
    }
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
    fn think(&mut self, time_delta: f64, cast: &Broadcast, actors: &HashMap<u32, ShipCache>, props: &Vec<Asteroid>) -> Vec<Directive> {
        let pressed: Vec<char> = cast.get_input();

        if pressed.contains(&'M') {
            return vec![Directive::Aim(cast.cursor), Directive::Thrust(1.0 * time_delta)];
        }
        else {
            let mut ret = Vec::<Directive>::new();

            if pressed.contains(&'L') {
                ret.push(Directive::Rotate(-TAU * time_delta));
            }
            if pressed.contains(&'R') {
                ret.push(Directive::Rotate(TAU * time_delta));
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

    fn box_clone(&self) -> Box<dyn Brain> {
        Box::new((*self).clone())
    }
}

#[derive(Clone,Debug)]
pub struct JalapenoBrain {
    id: u32,
    active: bool,
    player_position: Point,
    previous_collisons: Vec<u32>,
}

impl JalapenoBrain {
    pub fn new(id: u32) -> JalapenoBrain {
        JalapenoBrain {
            id: id,
            active: false,
            player_position: Point::new(0.0, 0.0),
            previous_collisons: Vec::new(),
        }
    }
}

impl Brain for JalapenoBrain {
    fn think(&mut self, time_delta: f64, cast: &Broadcast, actors: &HashMap<u32, ShipCache>, props: &Vec<Asteroid>) -> Vec<Directive> {
        if cast.player_id.is_some() &&
        self.target_visible(cast.player_position, actors[&self.id].get_point(), props) {
            self.active = true;
            self.player_position = cast.player_position;
        }

        return match self.active {
            true => vec![Directive::Aim(self.player_position), Directive::Thrust(1.0 * time_delta)],
            false => vec![Directive::Rotate(FRAC_PI_2 * time_delta)],
        }
    }

    fn box_clone(&self) -> Box<dyn Brain> {
        Box::new((*self).clone())
    }
}

#[derive(Clone,Debug)]
pub struct CayenneBrain {
    id: u32,
    active: bool,
    player_position: Point,
    previous_collisons: Vec<u32>,
}

impl CayenneBrain {
    pub fn new(id: u32) -> CayenneBrain {
        CayenneBrain {
            id: id,
            active: false,
            player_position: Point::new(0.0, 0.0),
            previous_collisons: Vec::new(),
        }
    }

    pub fn chase(&mut self, time_delta: f64, me: &ShipCache, target: Point) -> Vec<Directive> {
        // How to get to target (from where we are now)
        let ideal_path = Segment::new(me.get_point(), target);

        // Where would we go if we thrusted now?
        let mut plan = me.vector.clone();
        plan.add_vector(Vector::new(ideal_path.get_direction(), me.force * time_delta));
        let planned_path = Segment::new(
            Point::new(me.circle.x + plan.get_dx(), me.circle.y + plan.get_dy()),
            target
        );

        // Would we be closer to the target?
        if planned_path.get_length() < ideal_path.get_length() {
            return vec![
                Directive::Aim(target),
                Directive::Thrust(1.0 * time_delta)
            ];
        }

        vec![Directive::Brake]
    }
}

impl Brain for CayenneBrain {
    fn think(&mut self, time_delta: f64, cast: &Broadcast, actors: &HashMap<u32, ShipCache>, props: &Vec<Asteroid>) -> Vec<Directive> {
        if cast.player_id.is_some() && self.target_visible(cast.player_position, actors[&self.id].get_point(), props) {
            self.active = true;
            self.player_position = cast.player_position;
        }

        return match self.active {
            true => self.chase(time_delta, &actors[&self.id], self.player_position),
            false => vec![Directive::Rotate(FRAC_PI_2 * time_delta)],
        }
    }

    fn box_clone(&self) -> Box<dyn Brain> {
        Box::new((*self).clone())
    }
}
