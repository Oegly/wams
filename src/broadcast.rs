use crate::physics::Vector;
use crate::shape::*;
use crate::ship::*;

pub struct Broadcast {
    pub cursor: (f64, f64),
    pub input: Vec<char>,
    pub player_position: (f64, f64),
    pub messages: Vec<Message>
}

impl Broadcast {
    pub fn new() -> Broadcast {
        Broadcast {
            cursor: (0.0, 0.0),
            input: Vec::new(),
            player_position: (0.0, 0.0),
            messages: Vec::new(),
        }
    }

    pub fn record_actors(&mut self, actors: &Vec<ShipCache>, player_id: Option<usize>) {
        match player_id {
            Some(id) => self.player_position = (actors[id].circle.get_x(), actors[id].circle.get_y()),
            None => println!("No player present. :("),
        }
    }

    pub fn move_cursor(&mut self, x: f64, y: f64) {
        self.cursor = (x, y);
    }

    pub fn press(&mut self, pressed: char) {
        if !self.input.iter().any(|&b| b == pressed) {
            self.input.push(pressed);
        }
    }

    pub fn release(&mut self, released: char) {
        self.input.iter()
            .position(|&b| b == released)
            .map(|i| self.input.remove(i));
    }

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
