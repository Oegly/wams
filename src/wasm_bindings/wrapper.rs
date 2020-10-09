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

pub fn clear_canvas(ctx: &web_sys::CanvasRenderingContext2d) {
    //log(format!("{}", ctx.canvas().height()));
    ctx.clear_rect(0.0, 0.0, 1920.0, 1080.0);
}

#[wasm_bindgen]
pub struct GameWrapper {
    game: Game,
    inputs: Inputs,
}

#[wasm_bindgen]
impl GameWrapper {
    pub fn new(s: String) -> GameWrapper {
        match Game::from_json(s) {
            Ok(game) => GameWrapper {
                game: game,
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

    pub fn render(&mut self, ctx: &web_sys::CanvasRenderingContext2d) {
        let mut screen = WasmScreen::new(ctx);
        screen.clear();

        self.game.render(&mut screen);

        screen.write_status(self.game.get_score(), self.game.get_player_health().ceil() as u32 );
    }

    pub fn pressed(&mut self, btn: u32) {
        log(format!("{}", btn));
        match btn {
            38 => self.inputs.press('T'),
            40 => self.inputs.press('B'),
            37 => self.inputs.press('L'),
            39 => self.inputs.press('R'),
            80 => self.inputs.press('P'),
            _ => (),
        }
    }

    pub fn released(&mut self, btn: u32) {
        match btn {
            38 => self.inputs.release('T'),
            40 => self.inputs.release('B'),
            37 => self.inputs.release('L'),
            39 => self.inputs.release('R'),
            80 => {self.inputs.release('P'); self.game.pause()},
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
