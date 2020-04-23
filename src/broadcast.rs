use std::collections::HashMap;
use std::cell::RefCell;

use crate::physics::Vector;
use crate::shape::*;
use crate::ship::*;

pub struct Broadcast {
    pub cursor: Point,
    pub input: Vec<char>,
    pub player_id: Option<u32>,
    pub player_position: Point,
    pub messages: Vec<Message>,
    outbox: RefCell<Vec<Message>>,
}

impl Broadcast {
    pub fn new() -> Broadcast {
        Broadcast {
            cursor: Point::new(0.0, 0.0),
            input: Vec::new(),
            player_id: None,
            player_position: Point::new(0.0, 0.0),
            outbox: RefCell::new(Vec::new()),
            messages: Vec::new(),
        }
    }

    pub fn update(&mut self) {
        self.messages = self.outbox.replace(Vec::new());
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

    pub fn move_cursor(&mut self, cursor: &Point) {
        self.cursor = cursor.clone();
    }

    pub fn get_input(&self) -> Vec<char> {
        self.input.to_vec()
    }

    pub fn send_message(&self, msg: Message) {
        self.outbox.borrow_mut().push(msg);
    }
}

#[derive(Clone,Copy,Debug)]
pub struct Message {
    pub recipient: u32,
    pub sender: u32,
    pub body: MessageBody,
}

impl Message {
    pub fn new(recipient: u32, sender: u32, body: MessageBody) -> Message {
        Message {
            recipient: recipient,
            sender: sender,
            body: body,
        }
    }
}

#[derive(Clone,Copy,Debug)]
pub enum MessageBody {
    Death,
    Collison(Vector)
}
