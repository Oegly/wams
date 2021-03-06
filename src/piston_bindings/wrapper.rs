extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::rc::Rc;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

use crate::broadcast::*;
use crate::game::*;
use crate::piston_bindings::screen::*;
use crate::physics::Point;
use crate::ship::*;
use crate::storage::*;

const OPENGL_VERSION: glutin_window::OpenGL = OpenGL::V3_2;
const BG_COLOR: [f32; 4] = [0.9, 0.9, 0.9, 1.0];
const UPS: f64 = 60.0;

struct GameWrapper {
    game: Game,
    inputs: Inputs,
    state: GameState,
    gl: Rc<RefCell<GlGraphics>>,
}

impl GameWrapper {
    pub fn new() -> GameWrapper {
        let args: Vec<String> = std::env::args().collect();
        let filename = match args.get(1) {
            Some(s) => format!("data/{}.json", s.to_string()),
            None => "data/game.json".to_string()
        };

        println!("{}", filename);
        let mut content = String::new();
        let mut file = File::open(&filename).expect(&format!("File {} not found.", &filename));
        BufReader::new(file).read_to_string(&mut content);
        
        GameWrapper {
            game: Game::from_json(content).expect("Invalid JSON."),
            inputs: Inputs::new(),
            state: GameState::Running,
            gl: Rc::new(RefCell::new(GlGraphics::new(OPENGL_VERSION))),
        }
    }

    pub fn init(&mut self, window: &mut GlutinWindow) {
        let mut events = Events::new(EventSettings::new()).ups(UPS as u64);
        while let Some(e) = events.next(window) {
            if let Some(r) = e.render_args() {
                self.render(r);
            }

            if let Some(u) = e.update_args() {
                if !self.update(&u) {
                    break;
                }
            }

            if let Some(k) = e.button_args() {
                if k.state == ButtonState::Press {
                    self.pressed(&k.button);
                }
                else if k.state == ButtonState::Release {
                    self.released(&k.button);
                }
            }

            if let Some(pos) = e.mouse_cursor_args() {
                self.cursor_moved(pos[0], pos[1]);
            }

        }
    }

    pub fn update(&mut self, u: &UpdateArgs) -> bool {
        match self.state {
            GameState::Running => {
                self.game.update(&self.inputs.pressed, self.inputs.cursor, 1.0/UPS)
            },
            GameState::Paused => true,
        }
    }

    pub fn pause(&mut self) {
        match self.state {
            GameState::Running => {self.state = GameState::Paused},
            GameState::Paused => {self.state = GameState::Running},
        }
    }

    pub fn render(&mut self, r: RenderArgs) {
        let mut screen = PistonScreen::new(self.gl.clone(), r);
        screen.set_args(r);
        
        self.game.render(&mut screen);
    }

    pub fn pressed(&mut self, btn: &Button) {
        match btn {
            &Button::Keyboard(Key::Up) | &Button::Keyboard(Key::W) => self.inputs.press('T'),
            &Button::Keyboard(Key::Down) | &Button::Keyboard(Key::S) => self.inputs.press('B'),
            &Button::Keyboard(Key::Left) | &Button::Keyboard(Key::A) => self.inputs.press('L'),
            &Button::Keyboard(Key::Right) | &Button::Keyboard(Key::D) => self.inputs.press('R'),
            &Button::Keyboard(Key::P) => {self.pause()},
            &Button::Mouse(MouseButton::Left) => self.inputs.press('M'),
            _ => (),
        }
    }

    pub fn cursor_moved(&mut self, x: f64, y: f64) {
        self.inputs.move_cursor(x, y);
    }

    pub fn released(&mut self, btn: &Button) {
        match btn {
            &Button::Keyboard(Key::Up) | &Button::Keyboard(Key::W) => self.inputs.release('T'),
            &Button::Keyboard(Key::Down) | &Button::Keyboard(Key::S) => self.inputs.release('B'),
            &Button::Keyboard(Key::Left) | &Button::Keyboard(Key::A) => self.inputs.release('L'),
            &Button::Keyboard(Key::Right) | &Button::Keyboard(Key::D) => self.inputs.release('R'),
            &Button::Mouse(MouseButton::Left) => self.inputs.release('M'),
            _ => (),
        }
    }
}

pub struct Inputs {
    pub pressed: Vec<char>,
    pub cursor: Point,
}

impl Inputs {
    pub fn new() -> Inputs {
        Inputs {
            pressed: Vec::new(),
            cursor: Point::new(0.0, 0.0)
        }
    }

    pub fn press(&mut self, button: char) {
        if !self.pressed.iter().any(|&b| b == button) {
            self.pressed.push(button);
        }
    }

    pub fn release(&mut self, released: char) {
        self.pressed.iter()
            .position(|&b| b == released)
            .map(|i| self.pressed.remove(i));
    }

    pub fn move_cursor(&mut self, x: f64, y: f64) {
        self.cursor = Point::new(x, y);
    }
}

pub fn main() {
    let mut window: GlutinWindow = WindowSettings::new(
        "Well-Adjusted, Mature Spaceships",
        [1024, 768])
        .opengl(OPENGL_VERSION)
        .exit_on_esc(true)
        .build()
        .unwrap();

    GameWrapper::new().init(&mut window);

    println!("Thank you for playing!");
}

pub enum GameState {
    Running,
    Paused,
}