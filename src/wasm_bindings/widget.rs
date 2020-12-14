pub struct Widget {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub text: Vec<Paragraph>,
}

impl Widget {
    pub fn new_status(score: u32, health: u32, time: u32, ) -> Widget {
        Widget {
            x: 10.0,
            y: 10.0,
            width: 140.0,
            height: 90.0,
            text: vec![
                Paragraph {x: 24.0, y: 36.0, body: format!("Score: {:>6}", score)},
                Paragraph {x: 24.0, y: 60.0, body: format!("Health: {:>5}", health)},
                Paragraph {x: 24.0, y: 84.0, body: format!("Time: {:>4}:{:02}", time / 60, time % 60)},
            ],
        }
    }
}

pub struct Paragraph {
    pub x: f64,
    pub y: f64,
    pub body: String,
}

