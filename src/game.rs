use std::collections::HashMap;
use serde::{Serialize,Deserialize};
use serde_json::{Result, Value, Deserializer};

use crate::asteroid::*;
use crate::broadcast::*;
use crate::camera::*;
use crate::physics::Point;
use crate::ship::*;
use crate::storage::*;
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
    victory: Option<bool>,
    next: String,
    camera: Camera,
    broadcast: Broadcast,
    pressed: Vec<char>,
}

impl Game {
    pub fn new(
        player: ShipArgs,
        mobs: Vec<ShipArgs>,
        asteroids: Vec<AsteroidArgs>,
        walls: Vec<WallArgs>,
        spawner: bool,
        next: String,
        camera_follow: (bool, bool),
    ) -> Game {
        let mut factory = ShipFactory::new();

        let mut game = Game {
            tick: 0,
            player: ShipBuilder::from(&player).tag(1).build(),
            score: 0,
            spawner: ShipSpawner::new(spawner),
            mobs: Vec::new(),
            asteroids: Vec::new(),
            ship_count: 1,
            cached_actors: HashMap::new(),
            victory: None,
            next: next,
            camera: Camera::new(1024.0, 768.0, 1.0, camera_follow),
            broadcast: Broadcast::new(),
            pressed: Vec::new(),
        };

        for ship in mobs.iter() {
            game.create_ship(ShipBuilder::from(ship));
        }

        for asteroid in asteroids.iter() {
            game.asteroids.push(Asteroid::from(asteroid))
        }

        for wall in walls.iter() {
            game.asteroids.append(&mut Asteroid::from_wall_args(wall));
        }

        game
    }

    pub fn from_json(s: String) -> Result<Game> {
        let json: Value = serde_json::from_str(&s).unwrap();

        let player: ShipArgs = serde_json::from_value(json["player"].clone())?;
        let mobs: Vec<ShipArgs> = serde_json::from_value(json["mobs"].clone()).unwrap_or(vec![]);
        let asteroids: Vec<AsteroidArgs> = serde_json::from_value(json["asteroids"].clone()).unwrap_or(vec![]);
        let walls: Vec<WallArgs> = serde_json::from_value(json["walls"].clone()).unwrap_or(vec![]);
        let spawner = serde_json::from_value(json["spawner"].clone()).unwrap_or(false);
        let next = serde_json::from_value(json["next"].clone()).unwrap_or("level1".to_string());
        let camera_follow = serde_json::from_value(json["camera_follow"].clone()).unwrap_or((false, false));

        Ok(Game::new(player, mobs, asteroids, walls, spawner, next, camera_follow))
    }

    pub fn update(&mut self, pressed: &Vec<char>, cursor: Point, time_delta: f64) -> bool {
        self.tick += 1;

        self.broadcast.update(self.tick);
        self.broadcast.set_pressed(pressed);
        self.broadcast.move_cursor(cursor + self.camera.get_offset());
        self.read_messages();

        self.spawner.act(&self.broadcast);

        // Flush cache
        self.cached_actors.clear();

        // Cache player
        self.cached_actors.insert(self.player.get_id(), self.player.get_cache(time_delta as f64));

        // Cache non-player characters
        for mob in self.mobs.iter() {
            self.cached_actors.insert(mob.get_id(), mob.get_cache(time_delta as f64));
        }

        self.broadcast.record_actors(&self.cached_actors, Some(self.player.get_id()));

        //self.player.add_inputs(self.broadcast.input.to_vec());
        self.player.act(time_delta as f64, &self.broadcast, &self.cached_actors, &self.asteroids);

        for mob in self.mobs.iter_mut() {
            mob.act(time_delta as f64, &self.broadcast, &self.cached_actors, &self.asteroids);
        }

        match self.victory {
            Some(_) => false,
            None => true,
        }
    }

    pub fn render<S: Screen>(&mut self, screen: &mut S) {
        let status = self.camera.get_status();
        if status.0 || status.1 {
            self.camera.follow(self.player.get_x(), self.player.get_y());
            screen.set_offset(self.camera.get_offset());
        }

        screen.draw_background();
        
        for (id, ship) in self.cached_actors.iter() {
            screen.draw_ship(&ship, 1.0/60.0, self.tick);
        }

        for asteroid in self.asteroids.iter() {
            screen.draw_asteroid(&asteroid)
        }

        //println!("They hatin'?");
    }

    pub fn get_broadcast(&self) -> &Broadcast {
        &self.broadcast
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }

    pub fn get_player_health(&self) -> f64 {
        self.cached_actors[&self.player.get_id()].health
    }

    pub fn get_player_speed(&self) -> f64 {
        self.cached_actors[&self.player.get_id()].vector.magnitude.abs()
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
        if id == self.player.get_id() {
            self.victory = Some(false);
            return;
        }

        self.score += match self.cached_actors[&id].category {
            1 => 100,
            2 => 300,
            _ => 0,
        };

        println!("Ship #{} was killed. New score: {}", id, self.score);

        // How many ships are left?
        let alive = self.mobs.iter()
        // We do not care about the trajectory bounds, so time_delta can be arbitrary
        .filter(|m| m.get_cache(1.0/60.0 as f64).health > 0.0)
        .count();

        if alive == 0 {
            self.victory = Some(true);
        }
    }

    pub fn get_successor_args(&mut self) -> String {
        match self.victory.unwrap() {
            true => self.next.to_string(),
            false => "level1".to_string()
        }
    }
}

pub trait Screen {
    fn set_offset(&mut self, point: Point);
    fn draw_ship(&mut self, ship: &ShipCache, time_delta: f64, tick: u64);
    fn draw_asteroid(&self, asteroid: &Asteroid);
    fn draw_background(&self);
}