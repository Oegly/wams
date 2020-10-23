use std::collections::HashMap;
use wasm_bindgen::prelude::*;

use crate::asteroid::*;
use crate::camera::*;
use crate::game::*;
use crate::physics::*;
use crate::shape::*;
use crate::ship::*;
use crate::wasm_bindings::*;

use std::f64::consts::{PI,FRAC_PI_2};

const FONT_COLOR: &str = "#444444";
const HUD_COLOR: &str = "#666688";

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: String);
}

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

pub struct WasmScreen<'a> {
    ctx: &'a web_sys::CanvasRenderingContext2d,
    offset: Point,
}

impl<'a> WasmScreen<'a> {
    pub fn new(ctx: &'a web_sys::CanvasRenderingContext2d) -> WasmScreen {
        WasmScreen {
            ctx: ctx,
            offset: Point::new(0.0, 0.0),
        }
    }

    pub fn clear(&self) {
        self.ctx.clear_rect(0.0, 0.0, 1024.0, 768.0);
    }

    pub fn write_status(&self, score: u32, health: u32) {
        self.ctx.set_global_alpha(0.4);
        self.ctx.set_fill_style(&JsValue::from(&HUD_COLOR.to_string()));
        self.ctx.fill_rect(10.0, 10.0, 120.0, 60.0);
        self.ctx.set_global_alpha(1.0);
        self.ctx.set_fill_style(&JsValue::from(&FONT_COLOR.to_string()));
        self.ctx.set_font("16px Arial");
        self.ctx.fill_text(&"Score:", 24.0, 36.0);
        self.ctx.fill_text(&format!("{:>18}", score), 24.0, 36.0);
        self.ctx.fill_text(&"Health", 24.0, 60.0);
        self.ctx.fill_text(&format!("{:>18}", health), 24.0, 60.0);
    }
}

impl<'a> Screen for WasmScreen<'a> {
    fn draw_ship(&self, ship: &ShipCache) {
        let [mut x, mut y, r] = [ship.circle.x, ship.circle.y, ship.circle.r];
        x -= self.offset.x;
        y -= self.offset.y;

        let colors = &get_pallette(ship.category);
        let alpha = get_alpha(ship.health, ship.category);

        self.ctx.set_global_alpha(alpha);
        self.ctx.set_fill_style(&JsValue::from(&colors[1]));

        self.ctx.translate(x, y);
        self.ctx.rotate(ship.direction + FRAC_PI_2);

        // Nozzle
        self.ctx.begin_path();
        self.ctx.move_to(0.0 * r, -1.0 * r);
        self.ctx.line_to(-0.6 * r, 1.2 * r);
        self.ctx.line_to(0.6 * r, 1.2 * r);
        self.ctx.fill();

        // Wings
        self.ctx.begin_path();
        self.ctx.move_to(0.0 * r, -1.0 * r);
        self.ctx.line_to(1.5 * r, 0.4 * r);
        self.ctx.line_to(-1.5 * r, 0.4 * r);
        self.ctx.fill();

        self.ctx.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0);

        // Body
        self.ctx.set_fill_style(&JsValue::from(&colors[0]));
        self.ctx.begin_path();
        self.ctx.arc(x, y, r, 0.0, std::f64::consts::PI * 2.0).unwrap();
        self.ctx.fill();

        self.ctx.set_global_alpha(1.0);
    }

    fn draw_asteroid(&self, asteroid: &Asteroid) {
        let circle = asteroid.get_circle();
        let [mut x, mut y, r] = [circle.x, circle.y, circle.r];
        x -= self.offset.x;
        y -= self.offset.y;

        self.ctx.set_fill_style(&JsValue::from("#888"));
        self.ctx.begin_path();
        self.ctx.arc(x, y, r, 0.0, std::f64::consts::PI * 2.0).unwrap();
        self.ctx.fill();
    }

    fn set_offset(&mut self, point: Point) {
        self.offset = point;
        //self.draw_background();
    }

    fn draw_background(&self) {
        self.clear();
        self.ctx.set_fill_style(&JsValue::from("#ccccee".to_string()));
        self.ctx.fill_rect(0.0, 0.0, 1024.0, 768.0);
    }
}
