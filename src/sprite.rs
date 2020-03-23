extern crate opengl_graphics;

use std::collections::HashMap;

use piston::input::*;
use opengl_graphics::{GlGraphics, OpenGL};

use crate::physics::*;
use crate::ship::*;

fn get_pallette(category: ShipCategory) -> [[f32; 4]; 2]{
    match category {
        ShipCategory::Bell => [[0.8, 0.4, 0.4, 1.0], [0.6, 0.2, 0.2, 1.0]],
        ShipCategory::Jalapeno => [[0.38, 0.49, 0.2, 1.0], [0.23, 0.39, 0.03, 1.0]],
        ShipCategory::Cayenne => [[0.65, 0.1, 0.1, 1.0], [0.6, 0.2, 0.2, 1.0]],
    }
}

fn change_alpha(color: [f32; 4], alpha: f32) -> [f32; 4] {
    [color[0], color[1], color[2], alpha]
}

pub struct ShipSprite {}

impl ShipSprite {
    pub fn draw(gl: &mut GlGraphics, args: &RenderArgs, ship: &ShipCache) {
        use crate::graphics::Transformed;

        let [_x, _y, _r, _d] = ship.render_piston();
        let colors = get_pallette(ship.category);

        let ship_color = change_alpha(colors[0], ship.health as f32 * 0.008 + 0.2);
        let wing_color = change_alpha(colors[1], ship.health as f32 * 0.008 + 0.2);

        let pv = Vector::new(ship.direction, ship.vector.magnitude);
        let [fx, fy] = [_x + ship.vector.get_dx(), _y + ship.vector.get_dy()];
        let [px, py] = [_x + pv.get_dx(), _y + pv.get_dy()];

        gl.draw(args.viewport(), |c, gl| {
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
