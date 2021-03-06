use crate::broadcast::*;
use crate::physics::{Circle,Vector};
use crate::ship::*;

use std::f64::consts::{TAU};

pub struct ShipSpawner {
    active: bool,
}

impl ShipSpawner {
    pub fn new(a: bool) -> ShipSpawner {
        ShipSpawner {
            active: a,
        }
    }

    pub fn act(&self, cast: &Broadcast) {
        if self.active && cast.tick % 360 == 0 {
            self.create_ship(cast);
        }
    }

    fn create_ship(&self, cast: &Broadcast) {
        let m = 600.0;
        let d = (cast.tick as f64 / 360.0 ) % TAU;
        let v = Vector::new(d, m);

        let cat: usize;
        if cast.tick % 1080 == 0 {
            cat = 2;
        } else {
            cat = 1;
        }
        println!("{:?}, {}", cat, cast.tick);
        let ship = ShipBuilder::new(cat).place(v.get_dx() + 512.0, v.get_dy() + 384.0);
        cast.send_message(Message::new(0, 0, MessageBody::Birth(ship)));
    }
}