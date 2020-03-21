#![allow(unused)]

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod ai;
mod physics;
mod ship;
mod shape;
mod sprite;
mod broadcast;

use std::cell::RefCell;
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

use ship::*;
use broadcast::*;

const BG_COLOR: [f32; 4] = [0.6, 0.6, 0.8, 1.0];
const UPS: u64 = 60;

pub struct Game {
    tick: u64,
    gl: RefCell<GlGraphics>,
    player: Ship,
    mobs: Vec<Ship>,
    cached_actors: Vec<ShipCache>,
    broadcast: Broadcast,
    pressed: Vec<char>,
}

impl Game {
    fn update(&mut self, args: &UpdateArgs) -> bool {
        self.tick += 1;

        // Flush cache
        self.cached_actors = Vec::new();

        // Cache player
        self.cached_actors.push(self.player.get_cache(1.0/UPS as f64));

        // Cache non-player characters
        for mob in self.mobs.iter() {
            self.cached_actors.push(mob.get_cache(1.0/UPS as f64));
        }

        self.broadcast.record_actors(&self.cached_actors, Some(0));

        //self.player.add_inputs(self.broadcast.input.to_vec());
        self.player.act(1.0/UPS as f64, &self.broadcast, &self.cached_actors);

        for mob in self.mobs.iter_mut() {
            mob.act(1.0/UPS as f64, &self.broadcast, &self.cached_actors);
        }

        true
    }

    fn render(&mut self, args: &RenderArgs) {
        self.gl.borrow_mut().draw(args.viewport(), |c, gl| {
            graphics::clear(BG_COLOR, gl);
        });

        let _c = self.player.get_cache(1.0/UPS as f64);

        for ship in self.cached_actors.iter() {
            sprite::ShipSprite::draw(&mut self.gl.borrow_mut(), args, &ship)
        }
    }

    fn pressed(&mut self, btn: &Button) {
        let mut pressed: Vec<char> = Vec::new();

        match btn {
            &Button::Keyboard(Key::Up) => self.broadcast.press('T'),
            &Button::Keyboard(Key::Down) => self.broadcast.press('B'),
            &Button::Keyboard(Key::Left) => self.broadcast.press('L'),
            &Button::Keyboard(Key::Right) => self.broadcast.press('R'),
            &Button::Mouse(MouseButton::Left) => self.broadcast.press('M'),
            _ => (),
        }

        self.pressed = pressed;
    }

    fn cursor_moved(&mut self, x: f64, y: f64) {
        self.broadcast.move_cursor(x, y);
    }

    fn released(&mut self, btn: &Button) {
        match btn {
            &Button::Keyboard(Key::Up) => self.broadcast.release('T'),
            &Button::Keyboard(Key::Down) => self.broadcast.release('B'),
            &Button::Keyboard(Key::Left) => self.broadcast.release('L'),
            &Button::Keyboard(Key::Right) => self.broadcast.release('R'),
            &Button::Mouse(MouseButton::Left) => self.broadcast.release('M'),
            _ => (),
        }
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if this fails.
    let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new("Well-Adjusted, Mature Spaceships", [1024, 768])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut factory = ShipFactory::new();
    let player = factory.new_bell(400.0, 350.0);
    let mut mobs: Vec<Ship> = Vec::new();


    for i in 0..4 {
        mobs.push(factory.new_jalapeno(160.0 + 160.0 * i as f64, 100.0));
        mobs.push(factory.new_jalapeno(160.0 + 160.0 * i as f64, 600.0));
    }

    mobs.push(factory.new_cayenne(160.0, 350.0));

    let mut game = Game {
        tick: 0,
        gl: RefCell::new(GlGraphics::new(opengl)),
        player: player,
        mobs: mobs,
        cached_actors: Vec::new(),
        broadcast: Broadcast::new(),
        pressed: Vec::new(),
    };

    let mut events = Events::new(EventSettings::new()).ups(UPS);
    while let Some(e) = events.next(&mut window) {
        //println!("{:?}", e);
        if let Some(r) = e.render_args() {
            game.render(&r);
        }

        if let Some(u) = e.update_args() {
            if !game.update(&u) {
                break;
            }
        }

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                game.pressed(&k.button);
            }
            else if k.state == ButtonState::Release {
                game.released(&k.button);
            }
        }

        if let Some(pos) = e.mouse_cursor_args() {
            game.broadcast.move_cursor(pos[0], pos[1]);
        }

    }

    println!("Thank you for playing!");
}
