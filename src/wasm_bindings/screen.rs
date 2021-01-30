use std::collections::HashMap;
use wasm_bindgen::prelude::*;

use crate::ai::*;
use crate::asteroid::*;
use crate::broadcast::*;
use crate::camera::*;
use crate::game::*;
use crate::physics::{Point,Rectangle,Shape,Vector};
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
    pub rect: Rectangle,
}

impl WasmScreen {
    pub fn new(ctx: web_sys::CanvasRenderingContext2d) -> WasmScreen {
        let canvas = ctx.canvas().unwrap();
        
        WasmScreen {
            ctx: ctx,
            size: Point::new(canvas.width().into(), canvas.height().into()),
            offset: Point::new(0.0, 0.0),
            particles: Vec::new(),
            rect: Rectangle::new(0.0, 0.0, canvas.width().into(), canvas.height().into()),
        }
    }

    pub fn resize(&mut self) {
        let canvas = self.ctx.canvas().unwrap();

        self.size = Point::new(
            canvas.width().into(),
            canvas.width().into()
        );
        self.rect = Rectangle::new(
            0.0, 0.0,
            canvas.width().into(),
            canvas.height().into()
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
    
    pub fn draw_offscreen_ship(&self, ship: &ShipCache) {
        let colors = get_palette(ship.category as usize);

        // Distance from border
        let b = 30.0;
        let x = match ship.circle.x < self.offset.x + b {
            true => f64::max(ship.circle.x - self.offset.x, b),
            false => f64::min(ship.circle.x - self.offset.x, self.rect.width - b)
        };
        let y = match ship.circle.y < self.offset.y + b {
            true => f64::max(ship.circle.y - self.offset.y, b),
            false => f64::min(ship.circle.y - self.offset.y, self.rect.height - b)
        };

        let center = Point::new(
            self.offset.x + self.rect.width / 2.0,
            self.offset.y + self.rect.height / 2.0
        );
        let v = Vector::from(ship.get_point() - (Point::new(x, y) + self.offset));
        let d = v.direction;

        //let d = v.direction;
        //let x = self.rect.width / 2.0 + d.cos() * 359.0;
        //let y = self.rect.height / 2.0 + d.sin() * 359.0;
        let r = 10.0;
        let clamped = r * 4.0 / (1.0 + std::f64::consts::E.powf(-(v.magnitude/(self.rect.width*4.0))));
        
        self.ctx.set_fill_style(&JsValue::from("#999"));
        self.ctx.begin_path();
        self.ctx.arc(x, y, r, 0.0, std::f64::consts::PI * 2.0).unwrap();
        self.ctx.fill();

        self.ctx.begin_path();
        self.ctx.move_to(x + (d + FRAC_PI_2).cos() * r * 0.8, y + (d + FRAC_PI_2).sin() * r * 0.8);
        self.ctx.line_to(x + d.cos() * clamped, y + d.sin() * clamped);
        self.ctx.line_to(x + (d - FRAC_PI_2).cos() * r * 0.8, y + (d - FRAC_PI_2).sin() * r * 0.8);
        self.ctx.fill();

        self.ctx.set_fill_style(&JsValue::from(&colors[0]));
        self.ctx.begin_path();
        self.ctx.arc(x, y, r * 0.8, 0.0, std::f64::consts::PI * 2.0).unwrap();
        self.ctx.fill();
    }

    pub fn draw_particles(&mut self) {
        for p in self.particles.iter_mut() {
            p.tick(1.0/60.0);
        }

        self.particles.retain(|p| p.lifetime > p.elapsed);

        for p in &self.particles {
            self.ctx.set_fill_style(&p.color);
            self.ctx.set_global_alpha(p.get_alpha());
            self.ctx.begin_path();
            self.ctx.arc(p.x - self.offset.x, p.y - self.offset.y, p.get_size(), 0.0, std::f64::consts::PI * 2.0).unwrap();
            self.ctx.fill();
            self.ctx.set_global_alpha(1.0);
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


    fn draw_euclidean_vectors(&self, ship: &ShipCache) {
        let [mut x, mut y, r] = [ship.circle.x, ship.circle.y, ship.circle.r];
        x -= self.offset.x;
        y -= self.offset.y;

        self.ctx.set_stroke_style(&JsValue::from(&"#000000".to_string()));

        let plan = Vector::new(ship.direction, ship.vector.magnitude);
        let celta = plan.radian_delta(ship.vector.direction).cos();
        let coffset = Vector::new(ship.direction, ship.vector.magnitude * celta);

        self.ctx.begin_path();
        self.ctx.move_to(x, y);
        self.ctx.line_to(x + coffset.get_dx(), y + coffset.get_dy());
        self.ctx.stroke();

        self.ctx.begin_path();
        self.ctx.move_to(x, y);
        self.ctx.line_to(x - coffset.get_dx(), y - coffset.get_dy());
        self.ctx.stroke();

        let delta = plan.radian_delta(ship.vector.direction).sin();
        let offset = Vector::new(ship.direction + FRAC_PI_2 * delta.signum(), ship.vector.magnitude * delta);

        self.ctx.set_stroke_style(&JsValue::from(&"#ee2222".to_string()));

        self.ctx.begin_path();
        self.ctx.move_to(x, y);
        self.ctx.line_to(x + offset.get_dx(), y + offset.get_dy());
        self.ctx.stroke();

        self.ctx.set_stroke_style(&JsValue::from(&"#2222ee".to_string()));
        let v = Vector::new(offset.direction + PI, offset.magnitude);
        self.ctx.begin_path();
        self.ctx.move_to(x, y);
        self.ctx.line_to(x + v.get_dx(), y + v.get_dy());
        self.ctx.stroke();
    }

    fn draw_manhattan_vectors(&self, ship: &ShipCache) {
        let [mut x, mut y, r] = [ship.circle.x, ship.circle.y, ship.circle.r];
        x -= self.offset.x;
        y -= self.offset.y;

        self.ctx.begin_path();
        self.ctx.move_to(x, y);
        self.ctx.line_to(x + ship.vector.get_dx(), y);
        self.ctx.stroke();

        self.ctx.begin_path();
        self.ctx.move_to(x, y);
        self.ctx.line_to(x, y + ship.vector.get_dy());
        self.ctx.stroke();
    }
}

impl Screen for WasmScreen {
    fn draw_ship(&mut self, ship: &ShipCache, time_delta: f64, tick: u64) {
        let [mut x, mut y, r] = [ship.circle.x, ship.circle.y, ship.circle.r];
        let colors = get_palette(ship.category as usize);
        let alpha = get_alpha(ship.health, ship.category);

        let rect = Rectangle::new(self.offset.x, self.offset.y,
            self.size.x, self.size.y);

        if !self.rect.check_collision_shape(&ship.circle) {
            self.draw_offscreen_ship(ship);
            return ();
        }

        // Trail
        let thrusting = ship.actions.iter().any(|d| match d {
            Directive::Thrust(_) => true,
            _ => false,  
        });


        if tick % 1 == 0 && thrusting {
            let v = Vector::new(ship.direction + PI, r * 1.4);
            self.particles.push(
                Particle::new_trail(x + v.get_dx(), y + v.get_dy(), ship.vector.clone()));
        }

        x -= self.offset.x;
        y -= self.offset.y;

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

        self.ctx.set_global_alpha(1.0);

        //self.draw_euclidean_vectors(ship);
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
        self.rect.x = point.x;
        self.rect.y = point.y;
        //self.draw_background();
    }

    fn draw_background(&self) {
        self.clear();
        self.ctx.set_fill_style(&JsValue::from("#666688".to_string()));
        self.ctx.fill_rect(0.0, 0.0, self.size.x, self.size.y);
    }
}
