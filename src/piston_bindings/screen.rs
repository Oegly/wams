extern crate opengl_graphics;
extern crate graphics;

use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

use piston::input::*;
use opengl_graphics::{GlGraphics, OpenGL};

use crate::asteroid::*;
use crate::camera::*;
use crate::game::*;
use crate::physics::*;
use crate::shape::*;
use crate::ship::*;

use std::f64::consts::{PI,FRAC_PI_2};

const BG_COLOR: [f32; 4] = [0.6, 0.6, 0.7, 1.0];
//const MAX_HEALTH: [f32; 3] = [100.0, 25.0, 200.0];
const PALETTE: [[[f32; 4]; 2]; 3] = [
    [[0.8, 0.4, 0.4, 1.0], [0.6, 0.2, 0.2, 1.0]],
    [[0.38, 0.49, 0.2, 1.0], [0.23, 0.39, 0.03, 1.0]],
    [[0.65, 0.1, 0.1, 1.0], [0.6, 0.2, 0.2, 1.0]],
];

fn get_palette(category: usize) -> [[f32; 4]; 2] {
    match PALETTE.get(category) {
        Some(p) => *p,
        None => [[0.4, 0.4, 0.4, 1.0], [0.2, 0.2, 0.2, 1.0]]
    }
}

fn change_alpha(color: [f32; 4], hp: f32, category: usize) -> [f32; 4] {
    let percentage_left = hp.max(0.0) / HEALTH[category] as f32;
    [color[0], color[1], color[2], percentage_left * 0.8 + 0.2]
}

pub struct PistonScreen {
    args: RenderArgs,
    offset: Point,
    gl: Rc<RefCell<GlGraphics>>,
}

impl PistonScreen {
    pub fn new(gl: Rc<RefCell<GlGraphics>>, args: RenderArgs) -> PistonScreen {
        PistonScreen {
            gl: gl,
            offset: Point::new(0.0, 0.0),
            args: RenderArgs {
                ext_dt: 0.0,
                width: 0,
                height: 0,
                draw_width: 0,
                draw_height: 0,
            },
        }
    }

    pub fn set_args(&mut self, r: RenderArgs) {
        self.args = r;
    }

    pub fn clear(&self) {
        self.gl.borrow_mut().draw(self.args.viewport(), |c, gl| {
            graphics::clear(BG_COLOR, gl);
        });
    }
}

impl Screen for PistonScreen {
    fn draw_ship(&mut self, ship: &ShipCache, time_delta: f64, tick: u64) {
        use graphics::Transformed;

        let [mut _x, mut _y, _r, _d] = ship.render_piston();
        _x -= self.offset.x;
        _y -= self.offset.y;

        let colors = get_palette(ship.category); //PALLETTE[ship.category as usize];

        let ship_color = change_alpha(colors[0], ship.health as f32, ship.category);
        let wing_color = change_alpha(colors[1], ship.health as f32, ship.category);

        let pv = Vector::new(ship.direction, ship.vector.magnitude);
        let [fx, fy] = [_x + ship.vector.get_dx(), _y + ship.vector.get_dy()];
        let [px, py] = [_x + pv.get_dx(), _y + pv.get_dy()];

        self.gl.borrow_mut().draw(self.args.viewport(), |c, gl| {
            let body = [_x - _r, _y - _r, _r * 2.0, _r * 2.0];
            let wing = [[0.0, -1.0], [-1.5, 0.4], [1.5, 0.4]];
            let nozzle = [[0.0, -1.0], [-0.6, 1.2], [0.6, 1.2]];

            let transform = c
                .transform
                .trans(_x, _y)
                .rot_rad(_d + FRAC_PI_2)
                .scale(_r, _r);

            graphics::polygon(wing_color, &nozzle, transform, gl);
            graphics::polygon(wing_color, &wing, transform, gl);
            graphics::ellipse(ship_color, body, c.transform, gl);

            //graphics::line(ship_color, 1.0, [_x, _y, fx, fy], c.transform, gl);
            //graphics::line(wing_color, 1.0, [_x, _y, px, py], c.transform, gl);

            //graphics::rectangle([1.0, 1.0, 1.0, 0.4], ship.trajectory.render_piston(), c.transform, gl);
        });
    }

    fn draw_asteroid(&self, asteroid: &Asteroid) {
        use graphics::Transformed;

        let [mut _x, mut _y, _r] = asteroid.render_piston();
        _x -= self.offset.x;
        _y -= self.offset.y;

        self.gl.borrow_mut().draw(self.args.viewport(), |c, gl| {
            let body = [_x - _r, _y - _r, _r * 2.0, _r * 2.0];
            graphics::ellipse([0.4, 0.4, 0.4, 1.0], body, c.transform, gl);
        });
    }

    fn set_offset(&mut self, point: Point) {
        self.offset = point;
    }

    fn draw_background(&self) {
        self.clear();
    }
}
