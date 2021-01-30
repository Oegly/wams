pub struct Widget {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub text: Vec<Paragraph>,
}

impl Widget {
    pub fn status(score: u32, health: u32, speed: f64, time: u32) -> Widget {
        Widget {
            x: 10.0,
            y: 10.0,
            width: 150.0,
            height: 120.0,
            text: vec![
                Paragraph {x: 12.0, y: 24.0, body: format!("Score: {:>6}", score)},
                Paragraph {x: 12.0, y: 48.0, body: format!("Health: {:>5}", health)},
                Paragraph {x: 12.0, y: 72.0, body: format!("Speed: {:>6}", speed.round())},
                Paragraph {x: 12.0, y: 96.0, body: format!("Time: {:>4}:{:02}", time / 60, time % 60)},
            ],
        }
    }

    // Size of widget is fixed, size of screen.ctx canvas to define position
    pub fn pause(width: f64, height: f64) -> Widget {
        Widget {
            x: (width / 2.0 - 160.0).max(0.0),
            y: (height / 2.0 - 40.0).max(0.0),
            width: 320.0,
            height: 80.0,
            text: vec![
                Paragraph {x: 24.0, y: 36.0, body: format!("{:^32}", "Game is paused.")},
                Paragraph {x: 24.0, y: 60.0, body: "Press 'P' to resume playing.".to_string()},
            ],
        }
    }
}

pub struct Paragraph {
    pub x: f64,
    pub y: f64,
    pub body: String,
}

