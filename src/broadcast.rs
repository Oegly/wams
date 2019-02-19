pub struct Broadcast {
    pub input: Vec<char>,
}

impl Broadcast {
    pub fn new() -> Broadcast {
        Broadcast {
            input: Vec::new(),
        }
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

enum Message {
    Death
}
