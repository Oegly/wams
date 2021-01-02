extern crate console_error_panic_hook;

use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::cell::Cell;
use std::rc::Rc;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::broadcast::*;
use crate::game::*;
use crate::physics::Point;
use crate::ship::*;
use crate::storage::*;
use crate::wasm_bindings::widget::*;
use crate::wasm_bindings::screen::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: String);
}

pub fn now() -> f64 {
    web_sys::window().unwrap().performance().unwrap().now()
}

#[wasm_bindgen]
pub struct GameWrapper {
    game: Game,
    state: GameState,
    last_pause: f64,
    screen: WasmScreen,
    inputs: Inputs,
    idle: f64,
}

#[wasm_bindgen]
impl GameWrapper {
    pub fn new(s: String, ctx: web_sys::CanvasRenderingContext2d) -> GameWrapper {
        match Game::from_json(s) {
            Ok(game) => GameWrapper {
                game: game,
                state: GameState::Running,
                last_pause: 0.0,
                screen: WasmScreen::new(ctx),
                inputs: Inputs::new(),
                idle: now(),
            },
            Err(e) => panic!(e)
        }
    }

    pub fn update(&mut self) -> bool {
        match self.state {
            GameState::Running => {
                self.game.update(&self.inputs.pressed, self.inputs.cursor)
            },
            GameState::Paused => true,
        }
    }

    pub fn pause(&mut self) {
        match self.state {
            GameState::Running => {
                self.state = GameState::Paused;
                self.last_pause = now();
            },
            GameState::Paused => {
                self.state = GameState::Running;
                self.idle += now() - self.last_pause;
            },
        }
    }

    pub fn get_successor_args(&mut self) -> String {
        self.game.get_successor_args()
    }

    pub fn next_state(&mut self, s: String) {
        log(format!("Ran {:} ticks over {:} ms.\nAn average of {:.4} tps.",
        self.game.get_broadcast().tick, now() - self.idle, self.game.get_broadcast().tick as f64 / ((now() - self.idle) / 1000.0)));
        self.game = Game::from_json(s).unwrap();

        // If we go from a state with following camera to one with locked, this must be reset.
        self.screen.set_offset(Point::new(0.0, 0.0));

        // Reset staring time
        self.idle = now();
    }

    pub fn render(&mut self) {
        self.game.render(&mut self.screen);
        self.screen.draw_collision(self.game.get_broadcast());
        self.screen.draw_particles();

        match self.state {
            GameState::Paused => {
                self.screen.draw_widget(Widget::pause(
                    self.screen.size.x, self.screen.size.y
                ))
            },
            _ => ()
        };

        let time = match self.state {
            GameState::Running => now() - self.idle,
            GameState::Paused => self.last_pause - self.idle,
        };

        self.screen.draw_widget(Widget::status(
            self.game.get_score(),
            self.game.get_player_health().ceil() as u32,
            time.floor() as u32 / 1000
        ));
    }

    pub fn resize(&mut self) {
        self.screen.resize();
    }

    pub fn pressed(&mut self, btn: &str) {
        match btn {
            "arrowup" | "w" => self.inputs.press('T'),
            "arrowdown" | "s" => self.inputs.press('B'),
            "arrowleft" | "a"=> self.inputs.press('L'),
            "arrowright" | "d"=> self.inputs.press('R'),
            "p" => {self.pause()},
            _ => log(format!("btn: {}", btn.to_string())),
        }
    }

    pub fn released(&mut self, btn: &str) {
        match btn {
            "arrowup" | "w" => self.inputs.release('T'),
            "arrowdown" | "s" => self.inputs.release('B'),
            "arrowleft" | "a" => self.inputs.release('L'),
            "arrowright" | "d" => self.inputs.release('R'),
            _ => (),
        }
    }

    pub fn mouse_pressed(&mut self) {
        self.inputs.press('M')
    }

    pub fn mouse_released(&mut self) {
        self.inputs.release('M')
    }

    pub fn cursor_moved(&mut self, x: f64, y: f64) {
        self.inputs.move_cursor(x, y);
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

enum GameState {
    Running,
    Paused,
}
