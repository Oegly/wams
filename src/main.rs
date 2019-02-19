extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod ship;
mod broadcast;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

use ship::Ship;
use broadcast::Broadcast;

pub struct Game {
    gl: GlGraphics,
    player: Ship,
    broadcast: Broadcast,
    pressed: Vec<char>,
}

impl Game {
    fn update(&mut self, args: &UpdateArgs) -> bool {
        //self.player.add_inputs(self.broadcast.input.to_vec());
        self.player.act_player(1.0/30.0, &self.broadcast);

        true
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics;

        //println!("{:?}", args.draw_width, args.draw_height);

        const BG_COLOR: [f32; 4] = [0.6, 0.6, 0.8, 1.0];

        let [_sx, _sy, _sd] = self.player.render_piston();

        self.gl.draw(args.viewport(), |c, gl| {
            graphics::clear(BG_COLOR, gl);

            graphics::ellipse(
                [0.8, 0.4, 0.4, 1.0],
                [_sx - 18.0, _sy - 18.0, 36.0, 36.0],
                c.transform,
                gl
            );

            if _sx == 1.0 || _sy != 1.0 {
                graphics::line(
                    [0.6, 1.0, 0.6, 1.0],
                    1.0,
                    [
                        _sx,
                        _sy,
                        _sx + _sd.to_radians().sin() * 18.0,
                        _sy + _sd.to_radians().cos() * 18.0
                    ],
                    c.transform,
                    gl
                );
            }
        });
    }

    fn pressed(&mut self, btn: &Button) {
        let mut pressed: Vec<char> = Vec::new();

        match btn {
            &Button::Keyboard(Key::Up) => self.broadcast.press('T'),
            &Button::Keyboard(Key::Left) => self.broadcast.press('L'),
            &Button::Keyboard(Key::Right) => self.broadcast.press('R'),
            _ => (),
        }

        self.pressed = pressed;
    }

    fn released(&mut self, btn: &Button) {
        match btn {
            &Button::Keyboard(Key::Up) => self.broadcast.release('T'),
            &Button::Keyboard(Key::Left) => self.broadcast.release('L'),
            &Button::Keyboard(Key::Right) => self.broadcast.release('R'),
            _ => (),
        }
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if this fails.
    let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new("Snake Game", [1200, 800])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game {
        gl: GlGraphics::new(opengl),
        player: Ship::new(240.0, 240.0),
        broadcast: Broadcast::new(),
        pressed: Vec::new(),
    };

    let mut events = Events::new(EventSettings::new()).ups(60);
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
    }

    println!("Thank you for playing!\nYour snek managed to eat 0 foods.");
}
