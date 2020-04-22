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
            game: Game::new(player, mobs)
        }
    }

    pub fn update(&mut self, mouse_pressed: bool, pointer_x: f64, pointer_y: f64) -> bool {
        let mut pressed = Vec::new();

        if mouse_pressed {
            pressed.push('M')
        }

        self.game.update(&pressed, &(pointer_x, pointer_y))
    }

    pub fn render(&mut self, ctx: &web_sys::CanvasRenderingContext2d) {
        clear_canvas(ctx);

        self.game.render(|ship| {
            ShipSprite::draw(ctx, ship);
        });

        write_status(ctx, self.game.get_score(), self.game.get_player_health().ceil() as u32 );

        //log((self.game.get_player_health() as u32).to_string());
    }
}
