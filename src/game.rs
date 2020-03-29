use std::collections::HashMap;

use crate::broadcast::*;
use crate::ship::*;

const UPS: u64 = 60;

pub struct Game {
    tick: u64,
    player: Ship,
    mobs: Vec<Ship>,
    cached_actors: HashMap<u32, ShipCache>,
    broadcast: Broadcast,
    pressed: Vec<char>,
}

impl Game {
    pub fn new(player: Ship, mobs: Vec<Ship>) -> Game {
        Game {
            tick: 0,
            player: player,
            mobs: mobs,
            cached_actors: HashMap::new(),
            broadcast: Broadcast::new(),
            pressed: Vec::new(),
        }
    }

    pub fn update(&mut self, pressed: &Vec<char>, cursor: &(f64, f64)) -> bool {
        self.tick += 1;

        self.broadcast.set_pressed(pressed);
        self.broadcast.move_cursor(cursor);

        // Flush cache
        self.cached_actors = HashMap::new();

        // Cache player
        self.cached_actors.insert(self.player.get_id(), self.player.get_cache(1.0/UPS as f64));

        // Cache non-player characters
        for mob in self.mobs.iter() {
            self.cached_actors.insert(mob.get_id(), mob.get_cache(1.0/UPS as f64));
        }

        self.broadcast.record_actors(&self.cached_actors, Some(1));

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
            draw(&ship)
        }
    }
/*
    pub fn pressed(&mut self, btn: char) {
        self.broadcast.press(btn);
    }

    pub fn cursor_moved(&mut self, x: f64, y: f64) {
        self.broadcast.move_cursor(x, y);
    }

    pub fn released(&mut self, btn: char) {
        self.broadcast.release(btn);
    }*/
}
