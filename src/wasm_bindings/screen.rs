use std::collections::HashMap;
use wasm_bindgen::prelude::*;

use crate::physics::*;
use crate::ship::*;
use crate::wasm_bindings::*;

fn get_pallette(category: ShipCategory) ->  [String; 2] {
    match category {
        ShipCategory::Bell => ["#cc6666".to_string(), "#bb5555".to_string()],
        ShipCategory::Jalapeno => ["#66cc66".to_string(), "#55bb55".to_string()],
        ShipCategory::Cayenne => ["#ee4444".to_string(), "#dd5555".to_string()],
    }
}

fn get_max_health(category: ShipCategory) -> f64 {
    match category {
        ShipCategory::Bell => 100.0,
        ShipCategory::Jalapeno => 25.0,
        ShipCategory::Cayenne => 200.0,
    }
}

fn get_alpha(hp: f64, category: ShipCategory) -> f64 {
    let percentage_left = hp.max(0.0) / get_max_health(category);
    percentage_left * 0.8 + 0.2
}

pub struct ShipSprite {}

impl ShipSprite {
    pub fn draw(ctx: &web_sys::CanvasRenderingContext2d, ship: &ShipCache) {
        let [x, y, r] = [ship.circle.x, ship.circle.y, ship.circle.r];
        let colors = &get_pallette(ship.category);
        let alpha = get_alpha(ship.health, ship.category);

        ctx.set_global_alpha(alpha);
        ctx.set_fill_style(&JsValue::from(&colors[1]));

        ctx.translate(x, y);
        ctx.rotate(-ship.direction);
        ctx.scale(r, r);

        // Nozzle
        ctx.begin_path();
        ctx.move_to(0.0, 1.0);
        ctx.line_to(0.6, -1.2);
        ctx.line_to(-0.6, -1.2);
        ctx.fill();

        // Wings
        ctx.begin_path();
        ctx.move_to(0.0, 1.0);
        ctx.line_to(-1.5, -0.4);
        ctx.line_to(1.5, -0.4);
        ctx.fill();

        ctx.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0);

        // Body
        ctx.set_fill_style(&JsValue::from(&colors[0]));
        ctx.begin_path();
        ctx.arc(x, y, r, 0.0, std::f64::consts::PI * 2.0).unwrap();
        ctx.fill();
    }
}