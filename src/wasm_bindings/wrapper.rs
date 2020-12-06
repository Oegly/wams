extern crate console_error_panic_hook;

use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::cell::Cell;
use std::rc::Rc;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::game::*;
use crate::ship::*;
use crate::shape::*;
use crate::storage::*;
use crate::broadcast::*;
use crate::wasm_bindings::screen::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: String);
}

#[wasm_bindgen]
pub struct GameWrapper {
    game: Game,
    screen: WasmScreen,
    inputs: Inputs,
}

#[wasm_bindgen]
impl GameWrapper {
    pub fn new(s: String, ctx: web_sys::CanvasRenderingContext2d) -> GameWrapper {
        match Game::from_json(s) {
            Ok(game) => GameWrapper {
                game: game,
                screen: WasmScreen::new(ctx),
                inputs: Inputs::new(),
            },
            Err(e) => panic!(e)
        }
    }

    pub fn update(&mut self) -> bool {
        let mut pressed = Vec::new();

        if self.inputs.pressed.iter().any(|&b| b == 'M')  {
            pressed.push('M')
        }

        self.game.update(&self.inputs.pressed, self.inputs.cursor)
    }

    pub fn render(&mut self) {
        //let mut screen = WasmScreen::new(self.screen.ctx);

        self.game.render(&mut self.screen);

        self.screen.write_status(self.game.get_score(), self.game.get_player_health().ceil() as u32 );
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
            "p" => {self.inputs.press('P'); self.game.pause()},
            _ => log(format!("btn: {}", btn.to_string())),
        }
    }

    pub fn released(&mut self, btn: &str) {
        match btn {
            "arrowup" | "w" => self.inputs.release('T'),
            "arrowdown" | "s" => self.inputs.release('B'),
            "arrowleft" | "a" => self.inputs.release('L'),
            "arrowright" | "d" => self.inputs.release('R'),
            "p" => self.inputs.release('P'),
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
