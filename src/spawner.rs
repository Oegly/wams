use crate::broadcast::*;
use crate::physics::*;
use crate::ship::*;

pub struct ShipSpawner {}

impl ShipSpawner {
    pub fn new() -> ShipSpawner {
        ShipSpawner {
        }
    }

    pub fn act(&self, cast: &Broadcast) {
        if cast.tick % 360 == 0 {
            self.create_ship(cast);
        }
    }

    fn create_ship(&self, cast: &Broadcast) {
        let m = 600.0;
        let d = (cast.tick as f64 / 360.0 ) % TAU;
        let v = Vector::new(d, m);

        let cat: ShipCategory;
        if cast.tick % 1080 == 0 {
            cat = ShipCategory::Cayenne;
        } else {
            cat = ShipCategory::Jalapeno;
        }
        println!("{:?}, {}", cat, cast.tick);
        let ship = ShipBuilder::new(cat).place(v.get_dx() + 512.0, v.get_dy() + 384.0);
        cast.send_message(Message::new(0, 0, MessageBody::Birth(ship)));
    }
}