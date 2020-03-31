extern crate opengl_graphics;
extern crate graphics;

use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

use piston::input::*;
use opengl_graphics::{GlGraphics, OpenGL};

use crate::physics::*;
use crate::ship::*;
use crate::game::*;

const BG_COLOR: [f32; 4] = [0.6, 0.6, 0.8, 1.0];

fn get_pallette(category: ShipCategory) -> [[f32; 4]; 2]{
    match category {
        ShipCategory::Bell => [[0.8, 0.4, 0.4, 1.0], [0.6, 0.2, 0.2, 1.0]],
        ShipCategory::Jalapeno => [[0.38, 0.49, 0.2, 1.0], [0.23, 0.39, 0.03, 1.0]],
        ShipCategory::Cayenne => [[0.65, 0.1, 0.1, 1.0], [0.6, 0.2, 0.2, 1.0]],
    }
}

fn get_max_health(category: ShipCategory) -> f32 {
    match category {
        ShipCategory::Bell => 100.0,
        ShipCategory::Jalapeno => 25.0,
        ShipCategory::Cayenne => 200.0,
    }
}

fn change_alpha(color: [f32; 4], hp: f32, category: ShipCategory) -> [f32; 4] {
    let percentage_left = hp.max(0.0) / get_max_health(category);
    [color[0], color[1], color[2], percentage_left * 0.8 + 0.2]
}

pub struct ShipSprite {
    gl: Rc<RefCell<GlGraphics>>,
    args: RenderArgs,
}

impl ShipSprite {
    pub fn new(gl: Rc<RefCell<GlGraphics>>, args: RenderArgs) -> ShipSprite {
        ShipSprite {
            gl: gl,
            args: args,
        }
    }

    pub fn clear(&self) {
        self.gl.borrow_mut().draw(self.args.viewport(), |c, gl| {
            graphics::clear(BG_COLOR, gl);
        });
    }

    pub fn draw(&self, ship: &ShipCache) {
        use graphics::Transformed;

        let [_x, _y, _r, _d] = ship.render_piston();
        let colors = get_pallette(ship.category);

        let ship_color = change_alpha(colors[0], ship.health as f32, ship.category);
        let wing_color = change_alpha(colors[1], ship.health as f32, ship.category);

        let pv = Vector::new(ship.direction, ship.vector.magnitude);
        let [fx, fy] = [_x + ship.vector.get_dx(), _y + ship.vector.get_dy()];
        let [px, py] = [_x + pv.get_dx(), _y + pv.get_dy()];

        self.gl.borrow_mut().draw(self.args.viewport(), |c, gl| {
            let body = [_x - _r, _y - _r, _r * 2.0, _r * 2.0];
            let wing = [[0.0, 1.0], [0.0, -0.4], [-1.5, -0.4]];
            let nozzle = [[0.0, 1.0], [0.6, -1.2], [-0.6, -1.2]];

            let transform = c
                .transform
                .trans(_x, _y)
                .rot_rad(-_d)
                .scale(_r, _r);

            graphics::polygon(wing_color, &nozzle, transform, gl);
            graphics::polygon(wing_color, &wing, transform, gl);
            graphics::polygon(wing_color, &wing, transform.flip_h(), gl);
            graphics::ellipse(ship_color, body, c.transform, gl);

            //graphics::line(ship_color, 1.0, [_x, _y, fx, fy], c.transform, gl);
            //graphics::line(wing_color, 1.0, [_x, _y, px, py], c.transform, gl);

            //graphics::rectangle([1.0, 1.0, 1.0, 0.4], ship.trajectory.render_piston(), c.transform, gl);
        });
    }
}
