use std::collections::HashMap;
use serde::{Serialize,Deserialize};
use serde_json::{Result, Value, Deserializer};

use crate::asteroid::*;
use crate::broadcast::*;
use crate::camera::*;
use crate::physics::*;
use crate::shape::*;
use crate::ship::*;
use crate::storage::*;
use crate::spawner::*;

const UPS: u64 = 60;

pub struct Game {
    tick: u64,
    paused: u64,
    last_pause: u64,
    player: Ship,
    score: u32,
    spawner: ShipSpawner,
    mobs: Vec<Ship>,
    asteroids: Vec<Asteroid>,
    ship_count: u32,
    cached_actors: HashMap<u32, ShipCache>,
    camera: Camera,
    broadcast: Broadcast,
    pressed: Vec<char>,
}

impl Game {
    pub fn new(
        player: ShipArgs,
        mobs: Vec<ShipArgs>,
        asteroids: Vec<AsteroidArgs>,
        spawner: bool,
        camera_lock: bool,
    ) -> Game {
        let mut factory = ShipFactory::new();

        let mut game = Game {
            tick: 0,
            paused: 0,
            last_pause: 0,
            player: ShipBuilder::from(&player).tag(1).build(),
            score: 0,
            spawner: ShipSpawner::new(spawner),
            mobs: Vec::new(),
            asteroids: Vec::new(),
            ship_count: 1,
            cached_actors: HashMap::new(),
            camera: Camera::new(1024.0, 768.0, 1.0, camera_lock),
            broadcast: Broadcast::new(),
            pressed: Vec::new(),
        };

        for ship in mobs.iter() {
            game.create_ship(ShipBuilder::from(ship));
        }

        for asteroid in asteroids.iter() {
            game.asteroids.push(Asteroid::new(asteroid.0, asteroid.1, asteroid.2))
        }

        game
    }

    pub fn from_json(s: String) -> Result<Game> {
        let json: Value = serde_json::from_str(&s).unwrap();

        let player: ShipArgs = serde_json::from_value(json["player"].clone())?;
        let mobs: Vec<ShipArgs> = serde_json::from_value(json["mobs"].clone())?;
        let asteroids: Vec<AsteroidArgs> = serde_json::from_value(json["asteroids"].clone())?;
        let spawner = serde_json::from_value(json["spawner"].clone())?;
        let camera_lock = serde_json::from_value(json["camera_lock"].clone())?;

        Ok(Game::new(player, mobs, asteroids, spawner, camera_lock))
    }

    pub fn update(&mut self, pressed: &Vec<char>, cursor: Point) -> bool {
        if self.paused > 0 {
            self.paused += 1;
            return true;
        }

        self.tick += 1;

        self.broadcast.update(self.tick);
        self.broadcast.set_pressed(pressed);
        self.broadcast.move_cursor(cursor.clone().add(self.camera.get_offset()));
        self.read_messages();

        self.spawner.act(&self.broadcast);

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

    pub fn render<S: Screen>(&mut self, screen: &mut S) {
        if !self.camera.get_status() {
            self.camera.follow(self.player.get_x(), self.player.get_y());
            screen.set_offset(self.camera.get_offset());
        }

        screen.draw_background();
        
        for (id, ship) in self.cached_actors.iter() {
            screen.draw_ship(&ship);
        }

        for asteroid in self.asteroids.iter() {
            screen.draw_asteroid(&asteroid)
        }

        //println!("They hatin'?");
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }

    pub fn get_player_health(&self) -> f64 {
        self.cached_actors[&self.player.get_id()].health
    }

    pub fn pause(&mut self) {
        self.paused = (self.paused == 0) as u64;
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

pub trait Screen {
    fn set_offset(&mut self, point: Point);
    fn draw_ship(&self, ship: &ShipCache);
    fn draw_asteroid(&self, asteroid: &Asteroid);
    fn draw_background(&self);
}