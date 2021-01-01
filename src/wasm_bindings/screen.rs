use std::collections::HashMap;
use wasm_bindgen::prelude::*;

use crate::ai::*;
use crate::asteroid::*;
use crate::broadcast::*;
use crate::camera::*;
use crate::game::*;
use crate::physics::*;
use crate::shape::*;
use crate::ship::*;
use crate::wasm_bindings::*;
use crate::wasm_bindings::particle::*;


use std::f64::consts::{PI,FRAC_PI_2};

const FONT: &str = "16px Monospace";
const FONT_COLOR: &str = "#ffffff";
const HUD_COLOR: &str = "#404060c0";

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: String);
}

const PALETTE: [[&str; 2]; 3] = [
    ["#aa4444", "#993333"],
    ["#55bb55", "#44aa44"],
    ["#ee4444", "#dd5555"],
];

fn get_palette(category: usize) -> [String; 2] {
    match PALETTE.get(category) {
        Some(p) => [p[0].to_string(), p[1].to_string()],
        None => ["#555555".to_string(), "#333333".to_string()]
    }
}

fn get_alpha(hp: f64, category: usize) -> f64 {
    let percentage_left = hp.max(0.0) / HEALTH[category];//get_max_health(category);
    percentage_left * 0.8 + 0.2
}

pub struct WasmScreen {
    ctx: web_sys::CanvasRenderingContext2d,
    particles: Vec<Particle>,
    pub size: Point,
    pub offset: Point,
}

impl WasmScreen {
    pub fn new(ctx: web_sys::CanvasRenderingContext2d) -> WasmScreen {
        let canvas = ctx.canvas().unwrap();
        
        WasmScreen {
            ctx: ctx,
            size: Point::new(canvas.width().into(), canvas.height().into()),
            offset: Point::new(0.0, 0.0),
            particles: Vec::new(),
        }
    }

    pub fn resize(&mut self) {
        self.size = Point::new(
            self.ctx.canvas().unwrap().width().into(),
            self.ctx.canvas().unwrap().width().into()
        );
    }

    pub fn clear(&self) {
        self.ctx.clear_rect(0.0, 0.0, self.size.x, self.size.y);
    }

    pub fn draw_widget(&self, w: widget::Widget) {
        self.ctx.set_fill_style(&JsValue::from(&HUD_COLOR.to_string()));
        self.ctx.fill_rect(w.x, w.y, w.width, w.height);
        self.ctx.set_fill_style(&JsValue::from(&FONT_COLOR.to_string()));
        self.ctx.set_font(FONT);

        for p in w.text {
            self.ctx.fill_text(&p.body, w.x + p.x, w.y + p.y);
        }
    }
    
    pub fn draw_particles(&mut self) {
        for p in self.particles.iter_mut() {
            p.tick(1.0/60.0);
        }

        self.particles.retain(|p| p.lifetime > p.elapsed);

        for p in &self.particles {
            self.ctx.set_fill_style(&p.color);
            self.ctx.begin_path();
            self.ctx.arc(p.x - self.offset.x, p.y - self.offset.y, p.get_size(), 0.0, std::f64::consts::PI * 2.0).unwrap();
            self.ctx.fill();
        }
    }

    pub fn draw_collision(&mut self, cast: &Broadcast) {
        cast.messages.iter()
            .filter(|m| m.recipient == 0)
            .for_each(|msg| match msg.body {
                MessageBody::ShipCollision(id, p) => {
                    self.particles.append(&mut Particle::new_ship_collision(msg.sender, id, p));
                },
                MessageBody::AsteroidCollision(n, p) => {
                    self.particles.append(&mut Particle::new_asteroid_collision(p));
                },
                _ => ()
            });
    }
}

impl Screen for WasmScreen {
    fn draw_ship(&mut self, ship: &ShipCache, time_delta: f64, tick: u64) {
        let [mut x, mut y, r] = [ship.circle.x, ship.circle.y, ship.circle.r];

        // Trail
        let thrusting = ship.actions.iter().any(|d| match d {
            Directive::Thrust(_) => true,
            _ => false,  
        });

        if tick % 6 == 0 && thrusting {
            let v = Vector::new(ship.direction + PI, r * 1.4);
            self.particles.push(
                Particle::new_trail(x + v.get_dx(), y + v.get_dy(), ship.vector.clone()));
        }

        x -= self.offset.x;
        y -= self.offset.y;

        let colors = get_palette(ship.category as usize);
        let alpha = get_alpha(ship.health, ship.category);

        self.ctx.translate(x, y);
        self.ctx.rotate(ship.direction + FRAC_PI_2);
        
        self.ctx.set_global_alpha(alpha);
        self.ctx.set_fill_style(&JsValue::from(&colors[1]));

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

        /*self.ctx.set_global_alpha(0.2);

        self.ctx.set_fill_style(&JsValue::from(&colors[0]));
        self.ctx.begin_path();
        self.ctx.arc(x, y, ship.vector.magnitude, 0.0, std::f64::consts::PI * 2.0).unwrap();
        self.ctx.fill();
        */
        self.ctx.set_global_alpha(1.0);
    }

    fn draw_asteroid(&self, asteroid: &Asteroid) {
        let circle = asteroid.get_circle();
        let [mut x, mut y, r] = [circle.x, circle.y, circle.r];
        x -= self.offset.x;
        y -= self.offset.y;

        self.ctx.set_fill_style(&JsValue::from("#999"));
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
        self.ctx.set_fill_style(&JsValue::from("#666688".to_string()));
        self.ctx.fill_rect(0.0, 0.0, self.size.x, self.size.y);
    }
}
