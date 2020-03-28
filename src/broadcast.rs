use std::collections::HashMap;

use crate::physics::Vector;
use crate::shape::*;
use crate::ship::*;

pub struct Broadcast {
    pub cursor: (f64, f64),
    pub input: Vec<char>,
    pub player_id: Option<u32>,
    pub player_position: Point,
    pub messages: Vec<Message>
}

impl Broadcast {
    pub fn new() -> Broadcast {
        Broadcast {
            cursor: (0.0, 0.0),
            input: Vec::new(),
            player_id: None,
            player_position: Point::new(0.0, 0.0),
            messages: Vec::new(),
        }
    }

    pub fn record_actors(&mut self, actors: &HashMap<u32, ShipCache>, player_id: Option<u32>) {
        match player_id {
            Some(id) => self.record_player(&actors[&id]),
            None => println!("No player present. :("),
        }
    }

    pub fn record_player(&mut self, player: &ShipCache) {
        self.player_id = Some(player.id);
        self.player_position = Point::new(player.circle.get_x(), player.circle.get_y());
    }

    pub fn set_pressed(&mut self, pressed: &Vec<char>) {
        self.input = pressed.clone();
    }

    pub fn move_cursor(&mut self, cursor: &(f64, f64)) {
        self.cursor = cursor.clone();
    }

/*    pub fn press(&mut self, pressed: char) {
        if !self.input.iter().any(|&b| b == pressed) {
            self.input.push(pressed);
        }
    }

    pub fn release(&mut self, released: char) {
        self.input.iter()
            .position(|&b| b == released)
            .map(|i| self.input.remove(i));
    }*/

    pub fn get_input(&self) -> Vec<char> {
        self.input.to_vec()
    }
}

pub struct Message {
    recipient: u32,
    sender: u32,
    body: MessageBody,
}

pub enum MessageBody {
    Death,
    Collison(Vector)
}
