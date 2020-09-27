use std::collections::HashMap;

use crate::asteroid::*;
use crate::broadcast::*;
use crate::physics::*;
use crate::shape::*;
use crate::ship::*;
use crate::spawner::*;

const UPS: u64 = 60;

pub struct Game {
    tick: u64,
    player: Ship,
    score: u32,
    spawner: ShipSpawner,
    mobs: Vec<Ship>,
    asteroids: Vec<Asteroid>,
    ship_count: u32,
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
            spawner: ShipSpawner::new(),
            mobs: Vec::new(),
            asteroids: vec![Asteroid::new(200.0, 200.0, 20.0), Asteroid::new(400.0, 400.0, 8.0)],
            ship_count: 1,
            cached_actors: HashMap::new(),
            broadcast: Broadcast::new(),
            pressed: Vec::new(),
        }
    }

    pub fn update(&mut self, pressed: &Vec<char>, cursor: &Point) -> bool {
        self.tick += 1;

        self.spawner.act(&self.broadcast);

        self.broadcast.update(self.tick);

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
        self.player.act(1.0/UPS as f64, &self.broadcast, &self.cached_actors, &self.asteroids);

        for mob in self.mobs.iter_mut() {
            mob.act(1.0/UPS as f64, &self.broadcast, &self.cached_actors, &self.asteroids);
        }

        true
    }

    pub fn render<F: Fn(&ShipCache) -> (), G: Fn(&Asteroid)>(&mut self, draw_ship: F, draw_asteroid: G) {
        //clear(); //r.clear();

        for (id, ship) in self.cached_actors.iter() {
            draw_ship(&ship);
        }

        for asteroid in self.asteroids.iter() {
            draw_asteroid(&asteroid)
        }
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }

    pub fn get_player_health(&self) -> f64 {
        self.cached_actors[&self.player.get_id()].health
    }

    fn create_ship(&mut self, ship: ShipBuilder) {
        self.ship_count += 1;
        
        self.mobs.push(ship.tag(self.ship_count).build());
    }

    fn read_messages(&mut self) {
        let messages = self.broadcast.messages.iter()
            .filter(|m| m.recipient == 0)
            .cloned()
            .collect::<Vec<Message>>();

        for msg in messages {
            match msg.body {
                MessageBody::Death => self.process_death(msg.sender),
                MessageBody::Birth(ship) => self.create_ship(ship),
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
