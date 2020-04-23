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
use crate::broadcast::*;
use crate::wasm_bindings::screen::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: String);
}

pub fn clear_canvas(ctx: &web_sys::CanvasRenderingContext2d) {
    ctx.clear_rect(0.0, 0.0, 1024.0, 768.0);
}

#[wasm_bindgen]
pub struct GameWrapper {
    game: Game,
    inputs: Inputs,
}

#[wasm_bindgen]
impl GameWrapper {
    pub fn new() -> GameWrapper {
        let mut factory = ShipFactory::new();
        let player = factory.new_bell(400.0, 350.0);
        let mut mobs: Vec<Ship> = Vec::new();

        for i in 0..4 {
            mobs.push(factory.new_jalapeno(160.0 + 160.0 * i as f64, 100.0));
            mobs.push(factory.new_jalapeno(160.0 + 160.0 * i as f64, 600.0));
        }

        mobs.push(factory.new_cayenne(160.0, 350.0));
        mobs.push(factory.new_cayenne(640.0, 350.0));
        mobs.push(factory.new_cayenne(400.0, 600.0));

        GameWrapper {
            game: Game::new(player, mobs),
            inputs: Inputs::new(),
        }
    }

    pub fn update(&mut self) -> bool {
        let mut pressed = Vec::new();

        if self.inputs.pressed.iter().any(|&b| b == 'M')  {
            pressed.push('M')
        }

        self.game.update(&self.inputs.pressed, &self.inputs.cursor)
    }

    pub fn render(&mut self, ctx: &web_sys::CanvasRenderingContext2d) {
        clear_canvas(ctx);

        self.game.render(|ship| {
            ShipSprite::draw(ctx, ship);
        });

        write_status(ctx, self.game.get_score(), self.game.get_player_health().ceil() as u32 );

        //log((self.game.get_player_health() as u32).to_string());
    }

    pub fn say_hello(&self) -> f64 {
        1.0
    }

    pub fn pressed(&mut self, btn: u32) {
        match btn {
            38 => self.inputs.press('T'),
            40 => self.inputs.press('B'),
            37 => self.inputs.press('L'),
            39 => self.inputs.press('R'),
            _ => (),
        }
    }

    pub fn released(&mut self, btn: u32) {
        match btn {
            38 => self.inputs.release('T'),
            40 => self.inputs.release('B'),
            37 => self.inputs.release('L'),
            39 => self.inputs.release('R'),
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
    pub cursor: (f64, f64),
}

impl Inputs {
    pub fn new() -> Inputs {
        Inputs {
            pressed: Vec::new(),
            cursor: (0.0, 0.0)
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
        self.cursor = (x, y);
    }
}
