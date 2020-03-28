#![allow(unused)]

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

pub mod ai;
pub mod game;
pub mod physics;
pub mod ship;
pub mod shape;
pub mod sprite;
pub mod broadcast;

use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

use broadcast::*;
use game::*;
use ship::*;
use sprite::*;

const opengl_version: glutin_window::OpenGL = OpenGL::V3_2;
const BG_COLOR: [f32; 4] = [0.6, 0.6, 0.8, 1.0];
const UPS: u64 = 60;

pub struct Game {
    tick: u64,
    player: Ship,
    mobs: Vec<Ship>,
    cached_actors: HashMap<u32, ShipCache>,
    broadcast: Broadcast,
    pressed: Vec<char>,
}

impl Game {
    pub fn new(player: Ship, mobs: Vec<Ship>) -> Game {
        Game {
            tick: 0,
            player: player,
            mobs: mobs,
            cached_actors: HashMap::new(),
            broadcast: Broadcast::new(),
            pressed: Vec::new(),
        }
    }

    pub fn update(&mut self, pressed: &Vec<char>, cursor: &(f64, f64)) -> bool {
        self.tick += 1;

        println!("{:?}", pressed);
        self.broadcast.set_pressed(pressed);
        self.broadcast.move_cursor(cursor);

        // Flush cache
        self.cached_actors = HashMap::new();

        // Cache player
        self.cached_actors.insert(self.player.get_id(), self.player.get_cache(1.0/UPS as f64));

        // Cache non-player characters
        for mob in self.mobs.iter() {
            self.cached_actors.insert(mob.get_id(), mob.get_cache(1.0/UPS as f64));
        }

        self.broadcast.record_actors(&self.cached_actors, Some(1));

        //self.player.add_inputs(self.broadcast.input.to_vec());
        self.player.act(1.0/UPS as f64, &self.broadcast, &self.cached_actors);

        for mob in self.mobs.iter_mut() {
            mob.act(1.0/UPS as f64, &self.broadcast, &self.cached_actors);
        }

        true
    }

    pub fn render<F: Fn(&ShipCache) -> ()>(&mut self, draw: F) {
        //clear(); //r.clear();

        for (id, ship) in self.cached_actors.iter() {
            draw(&ship)
        }
    }
/*
    pub fn pressed(&mut self, btn: char) {
        self.broadcast.press(btn);
    }

    pub fn cursor_moved(&mut self, x: f64, y: f64) {
        self.broadcast.move_cursor(x, y);
    }

    pub fn released(&mut self, btn: char) {
        self.broadcast.release(btn);
    }*/
}


struct GameWrapper {
    game: Game,
    inputs: Inputs,
    gl: Rc<RefCell<GlGraphics>>,
}

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
            gl: Rc::new(RefCell::new(GlGraphics::new(opengl_version))),
        }
    }

    pub fn init(&mut self, window: &mut GlutinWindow) {
        let mut events = Events::new(EventSettings::new()).ups(UPS);
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
        self.game.update(&self.inputs.pressed, &self.inputs.cursor)
    }

    pub fn render(&mut self, r: RenderArgs) {
        let s = sprite::ShipSprite::new(self.gl.clone(), r);
        s.clear();
        self.game.render(|ship| {
            s.draw(ship);
        });
    }

    pub fn pressed(&mut self, btn: &Button) {
        match btn {
            &Button::Keyboard(Key::Up) => self.inputs.press('T'),
            &Button::Keyboard(Key::Down) => self.inputs.press('B'),
            &Button::Keyboard(Key::Left) => self.inputs.press('L'),
            &Button::Keyboard(Key::Right) => self.inputs.press('R'),
            &Button::Mouse(MouseButton::Left) => self.inputs.press('M'),
            _ => (),
        }
    }

    pub fn cursor_moved(&mut self, x: f64, y: f64) {
        self.inputs.move_cursor(x, y);
    }

    pub fn released(&mut self, btn: &Button) {
        match btn {
            &Button::Keyboard(Key::Up) => self.inputs.release('T'),
            &Button::Keyboard(Key::Down) => self.inputs.release('B'),
            &Button::Keyboard(Key::Left) => self.inputs.release('L'),
            &Button::Keyboard(Key::Right) => self.inputs.release('R'),
            &Button::Mouse(MouseButton::Left) => self.inputs.release('M'),
            _ => (),
        }
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

/*
pub trait RenderingContext {
    fn clear(&self);
    fn draw(&self, ship: &ShipCache);
}
 */

fn main() {
    let mut window: GlutinWindow = WindowSettings::new(
        "Well-Adjusted, Mature Spaceships",
        [1024, 768])
        .opengl(opengl_version)
        .exit_on_esc(true)
        .build()
        .unwrap();

    GameWrapper::new().init(&mut window);

    println!("Thank you for playing!");
}
