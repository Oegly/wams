use std::collections::HashMap;

use crate::broadcast::*;
use crate::physics::*;
use crate::shape::*;
use crate::ship::*;

const UPS: u64 = 60;

pub struct Game {
    tick: u64,
    player: Ship,
    score: u32,
    mobs: Vec<Ship>,
    factory: ShipFactory,
    cached_actors: HashMap<u32, ShipCache>,
    broadcast: Broadcast,
    pressed: Vec<char>,
}

impl Game {
    pub fn new(player: Ship, mobs: Vec<Ship>) -> Game {
        let mut factory = ShipFactory::new();

        Game {
            tick: 0,
            player: factory.new_bell(400.0, 350.0),
            score: 0,
            mobs: Vec::new(),
            factory: factory,
            cached_actors: HashMap::new(),
            broadcast: Broadcast::new(),
            pressed: Vec::new(),
        }
    }

    pub fn update(&mut self, pressed: &Vec<char>, cursor: &Point) -> bool {
        self.tick += 1;

        if self.tick % 360 == 0 {
            self.create_ship();
        }

        self.broadcast.update();

        self.read_messages();

        self.broadcast.set_pressed(pressed);
        self.broadcast.move_cursor(cursor);

        // Flush cache
        self.cached_actors.clear();

        // Cache player
        self.cached_actors.insert(self.player.get_id(), self.player.get_cache(1.0/UPS as f64));

        // Cache non-player characters
        for mob in self.mobs.iter() {
            self.cached_actors.insert(mob.get_id(), mob.get_cache(1.0/UPS as f64));
        }

        self.broadcast.record_actors(&self.cached_actors, Some(self.player.get_id()));

        //self.player.add_inputs(self.broadcast.input.to_vec());
        self.player.act(1.0/UPS as f64, &self.broadcast, &self.cached_actors);

        for mob in self.mobs.iter_mut() {
            mob.act(1.0/UPS as f64, &self.broadcast, &self.cached_actors);
        }

        true
    }

    pub fn render<F: Fn(&ShipCache) -> ()>(&mut self, draw: F) {
        //clear(); //r.clear();

        for (id, ship) in self.cached_actors.iter() {
            draw(&ship);
        }
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }

    pub fn get_player_health(&self) -> f64 {
        self.cached_actors[&self.player.get_id()].health
    }

    fn create_ship(&mut self) {
        let m = 600.0;
        let d = (self.tick as f64 / 360.0 ) % TAU;
        let v = Vector::new(d, m);

        if self.tick % 1080 == 0 {
            self.mobs.push(self.factory.new_cayenne(v.get_dx() + 512.0, v.get_dy() + 384.0));
        } else {
            self.mobs.push(self.factory.new_jalapeno(v.get_dx() + 512.0, v.get_dy() + 384.0));
        }
    }

    fn read_messages(&mut self) {
        let messages = self.broadcast.messages.iter()
            .filter(|m| m.recipient == 0)
            .cloned()
            .collect::<Vec<Message>>();

        for msg in messages {
            match msg.body {
                MessageBody::Death => self.process_death(msg.sender),
                _ => ()
            }
        }
    }

    fn process_death(&mut self, id: u32) {
        //println!("{}", id);

        if id == self.player.get_id() {
            println!("u ded");
            return;
        }

        self.score += match self.cached_actors[&id].category {
            ShipCategory::Jalapeno => 100,
            ShipCategory::Cayenne => 300,
            _ => 0,
        };

        println!("Ship #{} was killed. New score: {}", id, self.score);
    }
}
