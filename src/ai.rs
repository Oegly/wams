use std::collections::HashMap;

use crate::asteroid::*;
use crate::broadcast::*;
use crate::physics::*;
use crate::shape::*;
use crate::ship::*;

use std::f64::consts::{PI,FRAC_PI_2,TAU};

pub fn build_brain(category: usize, id: u32) -> Box<dyn Brain> {
    match category {
        BELL => Box::new(BellBrain::new(id)),
        JALAPENO => Box::new(JalapenoBrain::new(id)),
        CAYENNE => Box::new(CayenneBrain::new(id)),
        CHICKPEA => Box::new(ChickpeaBrain::new(id)),
        _ => panic!("Invalid int: {:}", category),
    }
}

#[derive(Clone,Copy,Debug,PartialEq)]
pub enum Directive {
    SetDirection(f64),
    Rotate(f64),
    Thrust(f64),
    Brake,
    Aim(Point)
}

pub trait Brain {
    fn think(&mut self, time_delta: f64, cast: &Broadcast, actors: &HashMap<u32, ShipCache>, props: &Vec<Asteroid>) -> Vec<Directive>;

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
        let ideal_path = Vector::from(target - me.get_point());
        let plan = me.vector + Vector::new(ideal_path.direction, me.force * time_delta);
        
        //let mut offset = me.vector - ideal_path;
        //offset.magnitude *= 2.0;
        //offset.direction += PI;

        // Would we be closer to the target?
        if (ideal_path - plan).magnitude < ideal_path.magnitude {
            return vec![
                Directive::SetDirection(ideal_path.direction),
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
}

#[derive(Clone,Debug)]
pub struct ChickpeaBrain {
    id: u32,
    active: bool,
    previous_collisons: Vec<u32>,
}

impl ChickpeaBrain {
    fn new(id: u32) -> ChickpeaBrain {
        ChickpeaBrain {
            id: id,
            active: false,
            previous_collisons: Vec::new(),
        }
    }

    fn chase(&mut self, actors: &HashMap<u32, ShipCache>, target_id: &u32) -> Vector {
        let me = actors[&self.id].get_point();
        let target = &actors[target_id];
        let speed = actors[&self.id].vector.magnitude.abs();
        let horizon = (speed / FORCE[CHICKPEA]).max(FORCE[CHICKPEA] * 4.0);

        actors.iter()
        .filter(
            // Only separate from your own class
            |(id, ship)| match ship.category {
                CHICKPEA => true,
                _ => false
            } &&
            // Who are not me
            self.id != ship.id &&
            // No further than 25 pixels away
            me.distance(ship.get_point()) - RADIUS[CHICKPEA] <= horizon &&
            // If the target is between another ship, don't bother
            !Segment::new(me, ship.get_point()).check_collision_circle(&target.circle)
        )
        .fold(Vector::from(target.get_point() - me), |sum, (id, ship)| {
            sum + Vector::from(me - ship.get_point())
        })
    }
}

impl Brain for ChickpeaBrain {
    fn think(&mut self, time_delta: f64, cast: &Broadcast, actors: &HashMap<u32, ShipCache>, props: &Vec<Asteroid>) -> Vec<Directive> {
        let me = actors[&self.id].get_point();
        let d = self.chase(actors, &cast.player_id.unwrap());

        if d.magnitude > 0.0 {
            return vec![
                Directive::SetDirection(d.direction),
                Directive::Thrust(1.0 * time_delta)
            ];
        }

        vec![]//*Directive::Aim(cast.player_position),*/ Directive::Thrust(1.0 * time_delta)]
    }
}